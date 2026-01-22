use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::SystemTime;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tauri::{AppHandle, Manager};

use super::config::{get_pandoc_executable_path, get_crossref_executable_path};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConvertOptions {
    pub input_file: String,
    pub output_file: Option<String>,
    pub source_dir: Option<String>,
    pub source_name: Option<String>,
    pub reference_doc: Option<String>,
    pub metadata: Option<Value>,  // Pandoc 元数据对象
    pub metadata_file: Option<String>,
    pub use_crossref: bool,
}

pub async fn convert_md_to_docx(app: &AppHandle, options: ConvertOptions) -> Result<String, String> {
    let pandoc_exe = get_pandoc_executable_path(app)?;
    
    if !pandoc_exe.exists() {
        return Err("Pandoc not installed. Please install it first.".to_string());
    }
    
    // 如果有元数据，先将其写入 Markdown 文件头部
    if let Some(metadata) = &options.metadata {
        inject_metadata_to_markdown(&options.input_file, metadata)?;
    }
    
    let output_path = resolve_output_path(&options);
    if let Some(parent) = output_path.parent() {
        fs::create_dir_all(parent)
            .map_err(|e| format!("Failed to create output directory: {}", e))?;
    }

    let mut cmd = Command::new(&pandoc_exe);

    // 基本参数
    cmd.arg(&options.input_file)
       .arg("-o")
       .arg(&output_path);

    if let Some(parent) = Path::new(&options.input_file).parent() {
        cmd.current_dir(parent);
    }
    
    // 参考文档（模板）
    let reference_doc_path = options.reference_doc.clone();
    if let Some(ref_doc) = &options.reference_doc {
        cmd.arg("--reference-doc").arg(ref_doc);
    }
    
    // 元数据文件
    if let Some(metadata) = &options.metadata_file {
        cmd.arg("--metadata-file").arg(metadata);
    }
    
    // 使用 crossref 过滤器
    if options.use_crossref {
        let crossref_exe = get_crossref_executable_path(app)?;
        if crossref_exe.exists() {
            cmd.arg("-F").arg(crossref_exe);
        }
    }
    
    // 执行转换
    let output = cmd.output()
        .map_err(|e| format!("Failed to execute pandoc: {}", e))?;
    
    if output.status.success() {
        // 转换成功后，删除 runtime 模板文件
        if let Some(ref_doc) = reference_doc_path {
            let ref_path = PathBuf::from(&ref_doc);
            if ref_path.exists() && ref_path.to_string_lossy().contains("runtime") {
                let _ = fs::remove_file(&ref_path); // 忽略删除错误
            }
        }
        
        // 清理旧的 session 目录（只保留最新5个）
        cleanup_old_sessions(app);
        
        Ok(output_path.to_string_lossy().to_string())
    } else {
        let error = String::from_utf8_lossy(&output.stderr);
        Err(format!("Pandoc conversion failed: {}", error))
    }
}

fn resolve_output_path(options: &ConvertOptions) -> PathBuf {
    let input_path = PathBuf::from(&options.input_file);

    let default_dir = input_path
        .parent()
        .map(|p| p.to_path_buf())
        .unwrap_or_else(|| PathBuf::from("."));

    let target_dir = options
        .source_dir
        .as_ref()
        .map(PathBuf::from)
        .filter(|p| p.exists())
        .unwrap_or_else(|| default_dir.clone());

    let base_stem = options
        .source_name
        .as_ref()
        .and_then(|n| Path::new(n).file_stem().map(|s| s.to_string_lossy().to_string()))
        .or_else(|| input_path.file_stem().map(|s| s.to_string_lossy().to_string()))
        .unwrap_or_else(|| "document".to_string());

    if let Some(provided) = &options.output_file {
        let candidate = PathBuf::from(provided);
        let stem = candidate
            .file_stem()
            .map(|s| s.to_string_lossy().to_string())
            .unwrap_or_else(|| base_stem.clone());
        let ext = candidate
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("docx");
        return make_unique_with_ext(candidate.parent().unwrap_or(&target_dir), &stem, ext);
    }

    make_unique_with_ext(&target_dir, &base_stem, "docx")
}

fn make_unique_with_ext(dir: &Path, stem: &str, ext: &str) -> PathBuf {
    let mut counter = 0;
    loop {
        let suffix = if counter == 0 {
            "_格式匠".to_string()
        } else {
            format!("_格式匠_{}", counter)
        };
        let file_name = format!("{}{}.{}", stem, suffix, ext);
        let candidate = dir.join(&file_name);
        if !candidate.exists() {
            return candidate;
        }
        counter += 1;
    }
}

pub fn check_pandoc_installed(app: &AppHandle) -> bool {
    get_pandoc_executable_path(app)
        .map(|path| path.exists())
        .unwrap_or(false)
}

pub fn check_crossref_installed(app: &AppHandle) -> bool {
    get_crossref_executable_path(app)
        .map(|path| path.exists())
        .unwrap_or(false)
}

pub fn get_pandoc_version(app: &AppHandle) -> Result<String, String> {
    let pandoc_exe = get_pandoc_executable_path(app)?;
    
    if !pandoc_exe.exists() {
        return Err("Pandoc not installed".to_string());
    }
    
    let output = Command::new(&pandoc_exe)
        .arg("--version")
        .output()
        .map_err(|e| format!("Failed to get version: {}", e))?;
    
    if output.status.success() {
        let version_str = String::from_utf8_lossy(&output.stdout);
        Ok(version_str.lines().next().unwrap_or("Unknown").to_string())
    } else {
        Err("Failed to get Pandoc version".to_string())
    }
}

/// 将元数据注入到 Markdown 文件的 YAML frontmatter
fn inject_metadata_to_markdown(file_path: &str, metadata: &Value) -> Result<(), String> {
    // 读取原始 Markdown 内容
    let content = fs::read_to_string(file_path)
        .map_err(|e| format!("Failed to read markdown file: {}", e))?;
    
    // 检查是否已有 YAML frontmatter
    let has_frontmatter = content.starts_with("---");
    
    // 构建 YAML frontmatter
    let yaml_lines: Vec<String> = metadata
        .as_object()
        .ok_or("Metadata is not an object")?  
        .iter()
        .filter_map(|(key, value)| {
            // 转换 JSON 值为 YAML 格式
            match value {
                Value::String(s) => Some(format!("{}: {}", key, s)),
                Value::Bool(b) => Some(format!("{}: {}", key, b)),
                Value::Number(n) => Some(format!("{}: {}", key, n)),
                Value::Array(arr) => {
                    // 数组转换为 YAML 列表
                    let items: Vec<String> = arr.iter()
                        .filter_map(|v| v.as_str().map(|s| format!("  - {}", s)))
                        .collect();
                    if items.is_empty() {
                        None
                    } else {
                        Some(format!("{}:\n{}", key, items.join("\n")))
                    }
                }
                _ => None,
            }
        })
        .collect();
    
    if yaml_lines.is_empty() {
        return Ok(()); // 没有有效的元数据，不需要修改文件
    }
    
    // 构建新内容
    let new_content = if has_frontmatter {
        // 替换现有的 frontmatter
        if let Some(end_pos) = content[3..].find("---") {
            let body = &content[end_pos + 6..]; // 跳过第二个 "---" 和换行符
            format!("---\n{}\n---\n{}", yaml_lines.join("\n"), body.trim_start())
        } else {
            // frontmatter 格式不正确，在前面添加
            format!("---\n{}\n---\n\n{}", yaml_lines.join("\n"), content)
        }
    } else {
        // 添加新的 frontmatter
        format!("---\n{}\n---\n\n{}", yaml_lines.join("\n"), content)
    };
    
    // 写回文件
    fs::write(file_path, new_content)
        .map_err(|e| format!("Failed to write markdown file: {}", e))?;
    
    Ok(())
}

/// 清理旧的 session 目录，只保留最新的5个
fn cleanup_old_sessions(app: &AppHandle) {
    if let Ok(cache_dir) = app.path().cache_dir() {
        let sessions_dir = cache_dir.join("format-tools").join("sessions");
        if !sessions_dir.exists() {
            return;
        }
        
        // 读取所有 session 目录
        let mut sessions: Vec<(PathBuf, SystemTime)> = Vec::new();
        if let Ok(entries) = fs::read_dir(&sessions_dir) {
            for entry in entries.flatten() {
                if let Ok(metadata) = entry.metadata() {
                    if metadata.is_dir() {
                        if let Ok(modified) = metadata.modified() {
                            sessions.push((entry.path(), modified));
                        }
                    }
                }
            }
        }
        
        // 按修改时间排序（最新的在前）
        sessions.sort_by(|a, b| b.1.cmp(&a.1));
        
        // 删除超过5个的旧 session
        if sessions.len() > 5 {
            for (path, _) in sessions.iter().skip(5) {
                let _ = fs::remove_dir_all(path); // 忽略删除错误
            }
        }
    }
}
