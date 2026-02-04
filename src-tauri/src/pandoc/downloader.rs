use reqwest;
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;
use tauri::{Emitter, Window};

use super::config::DownloadUrls;

#[derive(Clone, serde::Serialize)]
pub struct DownloadProgress {
    pub downloaded: u64,
    pub total: u64,
    pub percentage: f64,
}

pub async fn download_with_fallback(
    urls: &DownloadUrls,
    dest_path: &Path,
    window: Window,
    event_name: &str,
) -> Result<(), String> {
    // 先尝试主地址
    match download_file(&urls.primary, dest_path, window.clone(), event_name).await {
        Ok(_) => Ok(()),
        Err(_) => {
            // 如果主地址失败，尝试镜像地址
            download_file(&urls.mirror, dest_path, window, event_name).await
        }
    }
}

async fn download_file(
    url: &str,
    dest_path: &Path,
    window: Window,
    event_name: &str,
) -> Result<(), String> {
    let client = reqwest::Client::new();
    let response = client
        .get(url)
        .send()
        .await
        .map_err(|e| format!("Failed to download: {}", e))?;

    let total_size = response.content_length().unwrap_or(0);

    // 创建目标目录
    if let Some(parent) = dest_path.parent() {
        fs::create_dir_all(parent).map_err(|e| format!("Failed to create directory: {}", e))?;
    }

    let mut file = File::create(dest_path).map_err(|e| format!("Failed to create file: {}", e))?;

    let mut downloaded: u64 = 0;
    let mut stream = response.bytes_stream();

    use futures::StreamExt;
    while let Some(chunk) = stream.next().await {
        let chunk = chunk.map_err(|e| format!("Failed to read chunk: {}", e))?;
        file.write_all(&chunk)
            .map_err(|e| format!("Failed to write chunk: {}", e))?;

        downloaded += chunk.len() as u64;
        let percentage = if total_size > 0 {
            (downloaded as f64 / total_size as f64) * 100.0
        } else {
            0.0
        };

        let progress = DownloadProgress {
            downloaded,
            total: total_size,
            percentage,
        };

        // 发送进度更新
        let _ = window.emit(event_name, &progress);
    }

    Ok(())
}

pub async fn extract_archive(archive_path: &Path, extract_to: &Path) -> Result<(), String> {
    let archive_path_str = archive_path.to_str().unwrap();

    if archive_path_str.ends_with(".zip") {
        extract_zip(archive_path, extract_to)?
    } else if archive_path_str.ends_with(".tar.gz") {
        extract_tar_gz(archive_path, extract_to)?
    } else if archive_path_str.ends_with(".tar.xz") {
        extract_tar_xz(archive_path, extract_to)?
    } else if archive_path_str.ends_with(".7z") {
        extract_7z(archive_path, extract_to)?
    } else {
        return Err(format!(
            "Unsupported archive format for file: {}",
            archive_path_str
        ));
    }

    Ok(())
}

fn extract_zip(archive_path: &Path, extract_to: &Path) -> Result<(), String> {
    let file = File::open(archive_path).map_err(|e| format!("Failed to open archive: {}", e))?;

    let mut archive =
        zip::ZipArchive::new(file).map_err(|e| format!("Failed to read zip: {}", e))?;

    for i in 0..archive.len() {
        let mut file = archive
            .by_index(i)
            .map_err(|e| format!("Failed to read file from zip: {}", e))?;

        let outpath = extract_to.join(file.name());

        if file.is_dir() {
            fs::create_dir_all(&outpath)
                .map_err(|e| format!("Failed to create directory: {}", e))?;
        } else {
            if let Some(parent) = outpath.parent() {
                fs::create_dir_all(parent)
                    .map_err(|e| format!("Failed to create parent directory: {}", e))?;
            }

            let mut outfile =
                File::create(&outpath).map_err(|e| format!("Failed to create file: {}", e))?;

            std::io::copy(&mut file, &mut outfile)
                .map_err(|e| format!("Failed to extract file: {}", e))?;
        }
    }

    Ok(())
}

fn extract_tar_gz(archive_path: &Path, extract_to: &Path) -> Result<(), String> {
    let tar_gz = File::open(archive_path).map_err(|e| format!("Failed to open archive: {}", e))?;

    let tar = flate2::read::GzDecoder::new(tar_gz);
    let mut archive = tar::Archive::new(tar);

    archive
        .unpack(extract_to)
        .map_err(|e| format!("Failed to extract tar.gz: {}", e))?;

    Ok(())
}

fn extract_tar_xz(archive_path: &Path, extract_to: &Path) -> Result<(), String> {
    let xz_file = File::open(archive_path).map_err(|e| format!("Failed to open archive: {}", e))?;

    let tar = xz2::read::XzDecoder::new(xz_file);
    let mut archive = tar::Archive::new(tar);

    archive
        .unpack(extract_to)
        .map_err(|e| format!("Failed to extract tar.xz: {}", e))?;

    Ok(())
}

fn extract_7z(archive_path: &Path, extract_to: &Path) -> Result<(), String> {
    fs::create_dir_all(extract_to)
        .map_err(|e| format!("Failed to create extract directory: {}", e))?;

    sevenz_rust::decompress_file(archive_path, extract_to)
        .map_err(|e| format!("Failed to extract 7z archive: {}", e))?;

    Ok(())
}
pub fn find_executable_in_dir(dir: &Path, exe_name: &str) -> Option<std::path::PathBuf> {
    // 递归查找可执行文件
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_file() {
                if let Some(filename) = path.file_name() {
                    if filename.to_str().unwrap_or("").contains(exe_name) {
                        return Some(path);
                    }
                }
            } else if path.is_dir() {
                if let Some(found) = find_executable_in_dir(&path, exe_name) {
                    return Some(found);
                }
            }
        }
    }
    None
}
