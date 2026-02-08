use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};
use tauri::{AppHandle, Manager};

use super::downloader::extract_archive;

#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "source_type", rename_all = "snake_case")]
pub enum InputSource {
    File {
        path: String,
        original_name: Option<String>,
        selected_markdown: Option<String>,
    },
    Text {
        content: String,
        suggested_name: Option<String>,
    },
}

#[derive(Debug, Clone, Serialize)]
pub struct PreparedInput {
    pub markdown_path: String,
    pub assets_dir: String,
    pub image_count: usize,
    pub copied_images: Vec<String>,
    pub markdown_files: Vec<String>,
    pub source_name: Option<String>,
    pub source_dir: Option<String>,
}

pub async fn prepare_input(
    app_handle: &AppHandle,
    source: InputSource,
) -> Result<PreparedInput, String> {
    let session_dir = build_session_dir(app_handle)?;
    fs::create_dir_all(&session_dir).map_err(|e| format!("Failed to create session dir: {}", e))?;

    let assets_dir = session_dir.join("assets");
    fs::create_dir_all(&assets_dir).map_err(|e| format!("Failed to create assets dir: {}", e))?;

    match source {
        InputSource::File {
            path,
            original_name,
            selected_markdown,
        } => {
            let input_path = PathBuf::from(path.clone());
            if !input_path.exists() {
                return Err("File not found".to_string());
            }

            let file_name = original_name.or_else(|| {
                input_path
                    .file_name()
                    .map(|n| n.to_string_lossy().to_string())
            });

            let (markdown_path, copied_images, markdown_files) = handle_file_input(
                &input_path,
                &session_dir,
                &assets_dir,
                selected_markdown.as_deref(),
            )
            .await?;

            Ok(PreparedInput {
                markdown_path: markdown_path.to_string_lossy().to_string(),
                assets_dir: assets_dir.to_string_lossy().to_string(),
                image_count: copied_images.len(),
                copied_images,
                markdown_files,
                source_name: file_name,
                source_dir: input_path.parent().map(|p| p.to_string_lossy().to_string()),
            })
        }
        InputSource::Text {
            content,
            suggested_name,
        } => {
            let markdown_path = session_dir.join("document.md");
            let (copied_images, rewritten) = extract_and_copy_images(&content, None, &assets_dir)?;
            fs::write(&markdown_path, rewritten)
                .map_err(|e| format!("Failed to write markdown: {}", e))?;

            Ok(PreparedInput {
                markdown_path: markdown_path.to_string_lossy().to_string(),
                assets_dir: assets_dir.to_string_lossy().to_string(),
                image_count: copied_images.len(),
                copied_images,
                markdown_files: vec![markdown_path.to_string_lossy().to_string()],
                source_name: suggested_name,
                source_dir: None,
            })
        }
    }
}

async fn handle_file_input(
    input_path: &Path,
    session_dir: &Path,
    assets_dir: &Path,
    selected_markdown: Option<&str>,
) -> Result<(PathBuf, Vec<String>, Vec<String>), String> {
    let lower_name = input_path
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("")
        .to_lowercase();

    let (markdown_path, base_dir, markdown_files) = if is_archive(&lower_name) {
        let extract_dir = session_dir.join("extracted");
        fs::create_dir_all(&extract_dir)
            .map_err(|e| format!("Failed to create extract dir: {}", e))?;

        extract_archive(input_path, &extract_dir)
            .await
            .map_err(|e| format!("Failed to extract archive: {}", e))?;

        let md_files = collect_markdown_files(&extract_dir, &extract_dir);
        let selected_rel = selected_markdown
            .and_then(|sel| md_files.iter().find(|p| p.as_str() == sel).cloned())
            .or_else(|| md_files.get(0).cloned())
            .ok_or_else(|| "No markdown file found in archive".to_string())?;

        let md_file = extract_dir.join(&selected_rel);
        let target_md = session_dir.join("document.md");
        fs::copy(&md_file, &target_md).map_err(|e| format!("Failed to copy markdown: {}", e))?;

        (
            target_md,
            md_file.parent().map(|p| p.to_path_buf()),
            md_files,
        )
    } else {
        // treat as a direct markdown/text file
        let target_md = session_dir.join("document.md");
        fs::copy(input_path, &target_md).map_err(|e| format!("Failed to copy markdown: {}", e))?;
        (
            target_md.clone(),
            input_path.parent().map(|p| p.to_path_buf()),
            vec![target_md.to_string_lossy().to_string()],
        )
    };

    let content = fs::read_to_string(&markdown_path)
        .map_err(|e| format!("Failed to read markdown: {}", e))?;

    let (copied_images, rewritten) =
        extract_and_copy_images(&content, base_dir.as_deref(), assets_dir)?;
    fs::write(&markdown_path, rewritten)
        .map_err(|e| format!("Failed to write processed markdown: {}", e))?;

    Ok((markdown_path, copied_images, markdown_files))
}

fn is_archive(name: &str) -> bool {
    name.ends_with(".zip")
        || name.ends_with(".tar.gz")
        || name.ends_with(".tar.xz")
        || name.ends_with(".7z")
}

fn collect_markdown_files(dir: &Path, base: &Path) -> Vec<String> {
    let mut results = Vec::new();
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_file() {
                if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                    let ext = ext.to_lowercase();
                    if ext == "md" || ext == "markdown" || ext == "txt" {
                        if let Ok(rel) = path.strip_prefix(base) {
                            results.push(rel.to_string_lossy().to_string());
                        }
                    }
                }
            } else if path.is_dir() {
                let nested = collect_markdown_files(&path, base);
                results.extend(nested);
            }
        }
    }
    results.sort();
    results
}

fn extract_and_copy_images(
    content: &str,
    base_dir: Option<&Path>,
    assets_dir: &Path,
) -> Result<(Vec<String>, String), String> {
    let img_regex = Regex::new(r"!\[(?P<alt>[^\]]*)\]\((?P<path>[^)]+)\)")
        .map_err(|e| format!("Failed to compile regex: {}", e))?;

    let mut copied = Vec::new();
    let mut name_map: HashMap<String, String> = HashMap::new();

    let rewritten = img_regex
        .replace_all(content, |caps: &regex::Captures| {
            let img_path_str = caps.name("path").map(|m| m.as_str().trim()).unwrap_or("");

            if img_path_str.starts_with("http://") || img_path_str.starts_with("https://") {
                return caps.get(0).map(|m| m.as_str()).unwrap_or("").to_string();
            }

            if let Some(existing) = name_map.get(img_path_str) {
                let alt = caps.name("alt").map(|m| m.as_str()).unwrap_or("");
                return format!("![{}](assets/{})", alt, existing);
            }

            let resolved = resolve_image_path(img_path_str, base_dir);
            if let Some(source) = resolved {
                if source.is_file() {
                    let base_name = source
                        .file_name()
                        .map(|n| n.to_string_lossy().to_string())
                        .unwrap_or_else(|| "image".to_string());

                    let unique_name = make_unique_name(&base_name, assets_dir);
                    let target = assets_dir.join(&unique_name);

                    if let Err(err) = fs::copy(&source, &target) {
                        eprintln!("Failed to copy image {}: {}", img_path_str, err);
                        return caps.get(0).map(|m| m.as_str()).unwrap_or("").to_string();
                    }

                    name_map.insert(img_path_str.to_string(), unique_name.clone());
                    copied.push(target.to_string_lossy().to_string());

                    let alt = caps.name("alt").map(|m| m.as_str()).unwrap_or("");
                    return format!("![{}](assets/{})", alt, unique_name);
                }
            }

            caps.get(0).map(|m| m.as_str()).unwrap_or("").to_string()
        })
        .into_owned();

    Ok((copied, rewritten))
}

fn resolve_image_path(img: &str, base_dir: Option<&Path>) -> Option<PathBuf> {
    let candidate = Path::new(img);
    if candidate.is_absolute() {
        return Some(candidate.to_path_buf());
    }

    if let Some(base) = base_dir {
        let joined = base.join(candidate);
        if joined.exists() {
            return Some(joined);
        }
    }

    None
}

fn make_unique_name(base_name: &str, assets_dir: &Path) -> String {
    let mut candidate = base_name.to_string();
    let mut counter = 1;

    let mut parts = base_name.rsplitn(2, '.').collect::<Vec<_>>();
    parts.reverse();

    while assets_dir.join(&candidate).exists() {
        if parts.len() == 2 {
            candidate = format!("{}_{}.{}", parts[0], counter, parts[1]);
        } else {
            candidate = format!("{}_{}", base_name, counter);
        }
        counter += 1;
    }

    candidate
}

fn build_session_dir(app_handle: &AppHandle) -> Result<PathBuf, String> {
    let root = app_handle
        .path()
        .cache_dir()
        .map_err(|e| format!("Failed to get cache dir: {}", e))?;

    let millis = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|e| format!("Time error: {}", e))?
        .as_millis();

    Ok(root
        .join("format-tools")
        .join(format!("session-{}", millis)))
}
