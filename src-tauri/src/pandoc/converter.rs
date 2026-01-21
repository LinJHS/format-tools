use std::path::Path;
use std::process::Command;
use serde::{Deserialize, Serialize};
use tauri::AppHandle;

use super::config::{get_pandoc_executable_path, get_crossref_executable_path};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConvertOptions {
    pub input_file: String,
    pub output_file: String,
    pub reference_doc: Option<String>,
    pub metadata_file: Option<String>,
    pub use_crossref: bool,
}

pub async fn convert_md_to_docx(app: &AppHandle, options: ConvertOptions) -> Result<String, String> {
    let pandoc_exe = get_pandoc_executable_path(app)?;
    
    if !pandoc_exe.exists() {
        return Err("Pandoc not installed. Please install it first.".to_string());
    }
    
    let mut cmd = Command::new(&pandoc_exe);
    
    // 基本参数
    cmd.arg(&options.input_file)
       .arg("-o")
       .arg(&options.output_file);

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
        Ok(options.output_file)
    } else {
        let error = String::from_utf8_lossy(&output.stderr);
        Err(format!("Pandoc conversion failed: {}", error))
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
