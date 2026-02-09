use aes::Aes256;
use cbc::Decryptor;
use cipher::{block_padding::Pkcs7, BlockDecryptMut, KeyIvInit};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::fs;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};
use tauri::path::BaseDirectory;
use tauri::{AppHandle, Manager};

#[derive(Debug, Clone, Serialize)]
pub struct TemplateInfo {
    pub reference_doc: String,
    pub protected_path: String,
}

#[derive(Debug, Clone)]
struct TemplateResource {
    path: PathBuf,
    encrypted: bool,
}

pub fn prepare_template(
    app_handle: &AppHandle,
    template_name: &str,
    encrypted: bool,
    key_string: String,
) -> Result<TemplateInfo, String> {
    // Try to find template in resources
    let resource = find_template_resource(app_handle, template_name, encrypted)?;

    let cache_root = app_handle
        .path()
        .cache_dir()
        .map_err(|e| format!("Failed to get cache dir: {}", e))?;

    let runtime_dir = cache_root
        .join("format-tools")
        .join("templates")
        .join("runtime");
    fs::create_dir_all(&runtime_dir).map_err(|e| format!("Failed to create runtime dir: {}", e))?;

    let millis = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|e| format!("Time error: {}", e))?
        .as_millis();

    let runtime_docx = runtime_dir.join(format!("{}-{}.docx", template_name, millis));
    if resource.encrypted {
        let encrypted_bytes = fs::read(&resource.path)
            .map_err(|e| format!("Failed to read protected template: {}", e))?;
        let decrypted = decrypt_bytes_aes_cbc(&encrypted_bytes, key_string)?;
        fs::write(&runtime_docx, decrypted)
            .map_err(|e| format!("Failed to stage protected template: {}", e))?;
    } else {
        fs::copy(&resource.path, &runtime_docx)
            .map_err(|e| format!("Failed to stage template: {}", e))?;
    }

    Ok(TemplateInfo {
        reference_doc: runtime_docx.to_string_lossy().to_string(),
        protected_path: resource.path.to_string_lossy().to_string(),
    })
}

fn find_template_resource(
    app_handle: &AppHandle,
    template_id: &str,
    encrypted: bool,
) -> Result<TemplateResource, String> {
    let filename = template_id.to_string();

    // 1. Development Environment: Check project root relative paths
    // Unified directory: resources/templates
    let dev_candidates = [
        PathBuf::from("src-tauri/resources/templates").join(&filename),
        PathBuf::from("resources/templates").join(&filename),
    ];

    for path in &dev_candidates {
        if path.exists() {
            return Ok(TemplateResource {
                path: path.clone(),
                encrypted,
            });
        }
    }

    // 2. Production Environment: Check Resource directory
    // In production, check both 'templates' and 'resources/templates' subdirectories
    let candidates = [
        Path::new("templates").join(&filename),
        Path::new("resources").join("templates").join(&filename),
    ];

    for candidate in &candidates {
        if let Ok(path) = app_handle
            .path()
            .resolve(candidate, BaseDirectory::Resource)
        {
            if path.exists() {
                return Ok(TemplateResource { path, encrypted });
            }
        }
    }

    Err(format!(
        "Template '{}' not found in resources/templates",
        template_id
    ))
}

#[derive(Debug, Clone, Serialize)]
pub struct TemplateListResponse {
    pub templates: Vec<TemplateMeta>,
    pub has_premium: bool,
}

#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateMeta {
    pub id: String,
    pub name: String,
    pub description: String,
    #[serde(default)]
    pub category: String,
    #[serde(default)]
    pub member: bool,
    #[serde(default)]
    pub defaultPreset: serde_json::Value,
}

pub fn list_templates(_app_handle: &AppHandle) -> Result<TemplateListResponse, String> {
    // DEPRECATED: Metadata is now handled by the frontend importing templates.ts directly.
    // This function returns empty list to avoid breaking if called,
    // but should be avoided in favor of frontend config.
    Ok(TemplateListResponse {
        templates: vec![],
        has_premium: false,
    })
}

fn decrypt_bytes_aes_cbc(data: &[u8], key_string: String) -> Result<Vec<u8>, String> {
    if data.len() < 16 {
        return Err("Encrypted content is too short".to_string());
    }

    let mut hasher = Sha256::new();
    hasher.update(key_string.as_bytes());
    let key_bytes = hasher.finalize();

    let (iv, ciphertext) = data.split_at(16);
    let mut buffer = ciphertext.to_vec();
    let decrypted = Decryptor::<Aes256>::new((&key_bytes).into(), iv.into())
        .decrypt_padded_mut::<Pkcs7>(&mut buffer)
        .map_err(|_| "Failed to decrypt protected content".to_string())?;

    Ok(decrypted.to_vec())
}
