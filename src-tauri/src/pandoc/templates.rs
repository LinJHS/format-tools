use serde::{Serialize, Deserialize};
use std::fs;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};
use tauri::{AppHandle, Manager};
use aes::Aes256;
use cbc::Decryptor;
use cipher::{KeyIvInit, block_padding::Pkcs7, BlockDecryptMut};
use sha2::{Digest, Sha256};

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

pub fn prepare_template(app_handle: &AppHandle, template_name: &str) -> Result<TemplateInfo, String> {
    // Try to find template in resources
    let resource = find_template_resource(template_name)?;

    let cache_root = app_handle
        .path()
        .cache_dir()
        .map_err(|e| format!("Failed to get cache dir: {}", e))?;

    let runtime_dir = cache_root
        .join("format-tools")
        .join("templates")
        .join("runtime");
    fs::create_dir_all(&runtime_dir)
        .map_err(|e| format!("Failed to create runtime dir: {}", e))?;

    let millis = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|e| format!("Time error: {}", e))?
        .as_millis();

    let runtime_docx = runtime_dir.join(format!("{}-{}.docx", template_name, millis));
    if resource.encrypted {
        let encrypted = fs::read(&resource.path)
            .map_err(|e| format!("Failed to read protected template: {}", e))?;
        let decrypted = decrypt_bytes_aes_cbc(&encrypted)?;
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

fn find_template_resource(template_name: &str) -> Result<TemplateResource, String> {
    // 1. Try to find template in the current executable directory
    if let Ok(exe_path) = std::env::current_exe() {
        if let Some(exe_dir) = exe_path.parent() {
            let resource_path = exe_dir.join("resources").join("templates").join(template_name);
            if resource_path.exists() {
                return Ok(TemplateResource { path: resource_path, encrypted: false });
            }

            let protected_path = exe_dir.join("resources").join("auth-private").join("templates").join(template_name);
            if protected_path.exists() {
                return Ok(TemplateResource { path: protected_path, encrypted: true });
            }
        }
    }

    // 2. Try to find template relative to project root (for development)
    let candidates = [
        "src-tauri/resources/templates",
        "resources/templates",
        "../resources/templates",
    ];

    for base in &candidates {
        let path = Path::new(base).join(template_name);
        if path.exists() {
            if let Ok(canonical) = path.canonicalize() {
                return Ok(TemplateResource { path: canonical, encrypted: false });
            }
            return Ok(TemplateResource { path, encrypted: false });
        }
    }

    let protected_candidates = [
        "src/auth-private/templates",
        "auth-private/templates",
        "../auth-private/templates",
    ];

    for base in &protected_candidates {
        let path = Path::new(base).join(template_name);
        if path.exists() {
            if let Ok(canonical) = path.canonicalize() {
                return Ok(TemplateResource { path: canonical, encrypted: true });
            }
            return Ok(TemplateResource { path, encrypted: true });
        }
    }

    Err(format!("Template '{}' not found in resources", template_name))
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

pub fn list_templates(_app_handle: &AppHandle) -> Result<Vec<TemplateMeta>, String> {
    // Locate templates metadata file in resources/templates/templates.json
    let metadata_candidates = [
        // exe dir resources
        std::env::current_exe()
            .ok()
            .and_then(|p| p.parent().map(|d| d.join("resources").join("templates").join("templates.json"))),
        // dev paths
        Some(PathBuf::from("src-tauri/resources/templates/templates.json")),
        Some(PathBuf::from("resources/templates/templates.json")),
        Some(PathBuf::from("../resources/templates/templates.json")),
    ];

    let protected_metadata_candidates = [
        std::env::current_exe()
            .ok()
            .and_then(|p| p.parent().map(|d| d.join("resources").join("auth-private").join("templates").join("templates.json.enc"))),
        Some(PathBuf::from("src/auth-private/templates/templates.json.enc")),
        Some(PathBuf::from("auth-private/templates/templates.json.enc")),
        Some(PathBuf::from("../auth-private/templates/templates.json.enc")),
    ];

    let mut all_templates: Vec<TemplateMeta> = vec![];

    for opt_path in metadata_candidates.iter().flatten() {
        if opt_path.exists() {
            let content = fs::read_to_string(opt_path)
                .map_err(|e| format!("Failed to read metadata: {}", e))?;
            let parsed: serde_json::Value = serde_json::from_str(&content)
                .map_err(|e| format!("Invalid JSON in metadata: {}", e))?;
            let templates_val = if parsed.is_object() {
                parsed.get("templates").cloned().unwrap_or(serde_json::Value::Array(vec![]))
            } else {
                parsed
            };
            let mut list: Vec<TemplateMeta> = serde_json::from_value(templates_val)
                .map_err(|e| format!("Failed to parse templates: {}", e))?;
            all_templates.append(&mut list);
            break;
        }
    }

    for opt_path in protected_metadata_candidates.iter().flatten() {
        if opt_path.exists() {
            let encrypted = match fs::read(opt_path) {
                Ok(bytes) => bytes,
                Err(_) => continue,
            };

            let decrypted = match decrypt_bytes_aes_cbc(&encrypted) {
                Ok(bytes) => bytes,
                Err(_) => continue,
            };

            let content = match String::from_utf8(decrypted) {
                Ok(text) => text,
                Err(_) => continue,
            };

            let parsed: serde_json::Value = match serde_json::from_str(&content) {
                Ok(val) => val,
                Err(_) => continue,
            };

            let templates_val = if parsed.is_object() {
                parsed.get("templates").cloned().unwrap_or(serde_json::Value::Array(vec![]))
            } else {
                parsed
            };

            let mut list: Vec<TemplateMeta> = match serde_json::from_value(templates_val) {
                Ok(v) => v,
                Err(_) => continue,
            };

            all_templates.append(&mut list);
            break;
        }
    }

    if all_templates.is_empty() {
        Err("No templates metadata found in resources".to_string())
    } else {
        Ok(all_templates)
    }
}

fn decrypt_bytes_aes_cbc(data: &[u8]) -> Result<Vec<u8>, String> {
    if data.len() < 16 {
        return Err("Encrypted content is too short".to_string());
    }

    let key_string = std::env::var("TEMPLATE_ENCRYPTION_KEY")
        .or_else(|_| std::env::var("VITE_TEMPLATE_ENCRYPTION_KEY"))
        .or_else(|_| std::env::var("VITE_ENCRYPTION_KEY"))
        .map_err(|_| "Encryption key not configured in environment".to_string())?;

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
