pub mod config;
pub mod downloader;
pub mod converter;
pub mod input;
pub mod templates;

use tauri::{command, Window, AppHandle};

use config::{PandocConfig, get_pandoc_download_urls, get_crossref_download_urls, get_install_dir};
use downloader::{download_with_fallback, extract_archive, find_executable_in_dir};
use converter::{ConvertOptions, convert_md_to_docx, check_pandoc_installed, check_crossref_installed, get_pandoc_version, delete_all_sessions};
use input::{InputSource, PreparedInput, prepare_input};
use templates::{TemplateInfo, TemplateListResponse, prepare_template, list_templates as list_templates_impl};

#[command]
pub async fn install_pandoc(window: Window, app_handle: AppHandle) -> Result<String, String> {
    let config = PandocConfig::default();
    let urls = get_pandoc_download_urls(&config);
    let install_dir = get_install_dir(&app_handle)?;
    
    // 创建临时下载目录
    let temp_dir = std::env::temp_dir().join("pandoc_download");
    std::fs::create_dir_all(&temp_dir)
        .map_err(|e| format!("Failed to create temp directory: {}", e))?;
    
    let archive_name = urls.primary.split('/').last().unwrap();
    let archive_path = temp_dir.join(archive_name);
    
    // 下载
    download_with_fallback(&urls, &archive_path, window.clone(), "pandoc-download-progress").await?;
    
    // 解压
    let extract_dir = temp_dir.join("extracted");
    extract_archive(&archive_path, &extract_dir).await?;
    
    // 查找并移动可执行文件
    std::fs::create_dir_all(&install_dir)
        .map_err(|e| format!("Failed to create install directory: {}", e))?;
    
    let exe_name = if cfg!(windows) { "pandoc.exe" } else { "pandoc" };
    if let Some(exe_path) = find_executable_in_dir(&extract_dir, exe_name) {
        let dest_path = install_dir.join(exe_name);
        std::fs::copy(&exe_path, &dest_path)
            .map_err(|e| format!("Failed to copy executable: {}", e))?;
        
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mut perms = std::fs::metadata(&dest_path)
                .map_err(|e| format!("Failed to get file metadata: {}", e))?
                .permissions();
            perms.set_mode(0o755);
            std::fs::set_permissions(&dest_path, perms)
                .map_err(|e| format!("Failed to set permissions: {}", e))?;
        }
    } else {
        return Err("Pandoc executable not found in archive".to_string());
    }
    
    // 清理临时文件
    let _ = std::fs::remove_dir_all(&temp_dir);
    
    Ok("Pandoc installed successfully".to_string())
}

#[command]
pub async fn install_crossref(window: Window, app_handle: AppHandle) -> Result<String, String> {
    let config = PandocConfig::default();
    let urls = get_crossref_download_urls(&config);
    let install_dir = get_install_dir(&app_handle)?;
    
    // 创建临时下载目录
    let temp_dir = std::env::temp_dir().join("crossref_download");
    std::fs::create_dir_all(&temp_dir)
        .map_err(|e| format!("Failed to create temp directory: {}", e))?;
    
    let archive_name = urls.primary.split('/').last().unwrap();
    let archive_path = temp_dir.join(archive_name);
    
    // 下载
    download_with_fallback(&urls, &archive_path, window.clone(), "crossref-download-progress").await?;
    
    // 解压
    let extract_dir = temp_dir.join("extracted");
    extract_archive(&archive_path, &extract_dir).await?;
    
    // 查找并移动可执行文件
    let exe_name = if cfg!(windows) { "pandoc-crossref.exe" } else { "pandoc-crossref" };
    if let Some(exe_path) = find_executable_in_dir(&extract_dir, exe_name) {
        let dest_path = install_dir.join(exe_name);
        std::fs::copy(&exe_path, &dest_path)
            .map_err(|e| format!("Failed to copy executable: {}", e))?;
        
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mut perms = std::fs::metadata(&dest_path)
                .map_err(|e| format!("Failed to get file metadata: {}", e))?
                .permissions();
            perms.set_mode(0o755);
            std::fs::set_permissions(&dest_path, perms)
                .map_err(|e| format!("Failed to set permissions: {}", e))?;
        }
    } else {
        return Err("Pandoc-crossref executable not found in archive".to_string());
    }
    
    // 清理临时文件
    let _ = std::fs::remove_dir_all(&temp_dir);
    
    Ok("Pandoc-crossref installed successfully".to_string())
}

#[command]
pub fn is_pandoc_installed(app_handle: AppHandle) -> bool {
    check_pandoc_installed(&app_handle)
}

#[command]
pub fn is_crossref_installed(app_handle: AppHandle) -> bool {
    check_crossref_installed(&app_handle)
}

#[command]
pub fn pandoc_version(app_handle: AppHandle) -> Result<String, String> {
    get_pandoc_version(&app_handle)
}

#[command]
pub async fn convert_markdown(app_handle: AppHandle, options: ConvertOptions) -> Result<String, String> {
    convert_md_to_docx(&app_handle, options).await
}

#[command]
pub async fn prepare_input_payload(app_handle: AppHandle, source: InputSource) -> Result<PreparedInput, String> {
    prepare_input(&app_handle, source).await
}

#[allow(non_snake_case)]
#[command]
pub fn prepare_template_protected(app_handle: AppHandle, templateName: String, isMember: bool, key: String) -> Result<TemplateInfo, String> {
    // Tauri v2 expects camelCase param names; use `templateName` here
    prepare_template(&app_handle, &templateName, isMember, key)
}

#[command]
pub fn list_templates(app_handle: AppHandle) -> Result<TemplateListResponse, String> {
    list_templates_impl(&app_handle)
}

#[command]
pub async fn clear_sessions(app_handle: AppHandle) -> Result<(), String> {
    delete_all_sessions(&app_handle)
}

