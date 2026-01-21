use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use serde::{Deserialize, Serialize};
use tauri::AppHandle;

use super::config::{get_pandoc_executable_path, get_crossref_executable_path};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConvertOptions {
    pub input_file: String,
    pub output_file: Option<String>,
    pub source_dir: Option<String>,
    pub source_name: Option<String>,
    pub reference_doc: Option<String>,
    pub metadata_file: Option<String>,
    pub use_crossref: bool,
}

pub async fn convert_md_to_docx(app: &AppHandle, options: ConvertOptions) -> Result<String, String> {
    let pandoc_exe = get_pandoc_executable_path(app)?;
    
    if !pandoc_exe.exists() {
        return Err("Pandoc not installed. Please install it first.".to_string());
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
