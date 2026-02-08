use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tauri::{AppHandle, Manager};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PandocConfig {
    pub version: String,
    pub crossref_version: String,
}

impl Default for PandocConfig {
    fn default() -> Self {
        Self {
            version: "3.8.3".to_string(),
            crossref_version: "0.3.22b".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownloadUrls {
    pub primary: String,
    pub mirror: String,
}

pub fn get_platform_info() -> (String, String) {
    let os = std::env::consts::OS;
    let arch = std::env::consts::ARCH;
    (os.to_string(), arch.to_string())
}

pub fn get_pandoc_download_urls(config: &PandocConfig) -> DownloadUrls {
    let (os, arch) = get_platform_info();
    let version = &config.version;

    let filename = match (os.as_str(), arch.as_str()) {
        ("windows", "x86_64") => format!("pandoc-{}-windows-x86_64.zip", version),
        ("linux", "x86_64") => format!("pandoc-{}-linux-amd64.tar.gz", version),
        ("linux", "aarch64") => format!("pandoc-{}-linux-arm64.tar.gz", version),
        ("macos", "x86_64") => format!("pandoc-{}-x86_64-macOS.zip", version),
        ("macos", "aarch64") => format!("pandoc-{}-arm64-macOS.zip", version),
        _ => panic!("Unsupported platform: {} {}", os, arch),
    };

    let base_url = format!(
        "https://github.com/jgm/pandoc/releases/download/{}",
        version
    );
    let mirror_url = format!(
        "https://ghfast.top/https://github.com/jgm/pandoc/releases/download/{}",
        version
    );

    DownloadUrls {
        primary: format!("{}/{}", base_url, filename),
        mirror: format!("{}/{}", mirror_url, filename),
    }
}

pub fn get_crossref_download_urls(config: &PandocConfig) -> DownloadUrls {
    let (os, arch) = get_platform_info();
    let version = &config.crossref_version;

    let filename = match (os.as_str(), arch.as_str()) {
        ("windows", "x86_64") => "pandoc-crossref-Windows-X64.7z".to_string(),
        ("linux", "x86_64") => "pandoc-crossref-Linux-X64.tar.xz".to_string(),
        ("linux", "aarch64") => "pandoc-crossref-Linux-ARM64.tar.xz".to_string(),
        ("macos", "x86_64") => "pandoc-crossref-macOS-X64.tar.xz".to_string(),
        ("macos", "aarch64") => "pandoc-crossref-macOS-ARM64.tar.xz".to_string(),
        _ => panic!("Unsupported platform: {} {}", os, arch),
    };

    let base_url = format!(
        "https://github.com/lierdakil/pandoc-crossref/releases/download/v{}",
        version
    );
    let mirror_url = format!(
        "https://ghfast.top/https://github.com/lierdakil/pandoc-crossref/releases/download/v{}",
        version
    );

    DownloadUrls {
        primary: format!("{}/{}", base_url, filename),
        mirror: format!("{}/{}", mirror_url, filename),
    }
}

pub fn get_install_dir(app: &AppHandle) -> Result<PathBuf, String> {
    let app_data_dir = app
        .path()
        .app_data_dir()
        .map_err(|e| format!("Failed to get app data directory: {}", e))?;

    let install_dir = app_data_dir.join("pandoc");
    Ok(install_dir)
}

pub fn get_pandoc_executable_path(app: &AppHandle) -> Result<PathBuf, String> {
    let install_dir = get_install_dir(app)?;
    let exe_path = if cfg!(windows) {
        install_dir.join("pandoc.exe")
    } else {
        install_dir.join("pandoc")
    };
    Ok(exe_path)
}

pub fn get_crossref_executable_path(app: &AppHandle) -> Result<PathBuf, String> {
    let install_dir = get_install_dir(app)?;
    let exe_path = if cfg!(windows) {
        install_dir.join("pandoc-crossref.exe")
    } else {
        install_dir.join("pandoc-crossref")
    };
    Ok(exe_path)
}
