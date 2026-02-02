use serde::{Serialize, Deserialize};
use std::fs;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};
use tauri::{AppHandle, Manager};
use tauri::path::BaseDirectory;
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
    let resource = find_template_resource(app_handle, template_name)?;

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

fn find_template_resource(app_handle: &AppHandle, template_id: &str) -> Result<TemplateResource, String> {
    // Determine the expected filename (assuming .docx extension)
    let filename = template_id.to_string();

    // 1. Development Environment: Check project root relative paths
    // Unified directory: resources/templates
    let dev_candidates = [
        PathBuf::from("src-tauri/resources/templates").join(&filename),
        PathBuf::from("resources/templates").join(&filename),
    ];

    for path in &dev_candidates {
        if path.exists() {
            let encrypted = get_template_member_flag(app_handle, template_id).unwrap_or(false);
            return Ok(TemplateResource { path: path.clone(), encrypted });
        }
    }

    // 2. Production Environment: Check Resource directory
    // In production, everything is in the 'templates' subdirectory of resources
    let resource_path = app_handle
        .path()
        .resolve(Path::new("templates").join(&filename), BaseDirectory::Resource)
        .ok();
        
    if let Some(path) = resource_path {
        if path.exists() {
            let encrypted = get_template_member_flag(app_handle, template_id).unwrap_or(false);
            return Ok(TemplateResource { path, encrypted });
        }
    }

    Err(format!("Template '{}' not found in resources/templates", template_id))
}

fn parse_ts_object(content: &str) -> Result<serde_json::Value, String> {
    let start = content.find('{').ok_or("No JSON object start found in metadata")?;
    let end = content.rfind('}').ok_or("No JSON object end found in metadata")?;
    if end <= start {
        return Err("Invalid metadata object boundaries".to_string());
    }
    let json_like = &content[start..=end];
    serde_json::from_str(json_like)
        .map_err(|e| format!("Invalid JSON content in metadata: {}", e))
}

fn load_metadata_from_candidates(candidates: &[PathBuf]) -> Result<Vec<TemplateMeta>, String> {
    for path in candidates {
        if path.exists() {
            let content = fs::read_to_string(path)
                .map_err(|e| format!("Failed to read metadata: {}", e))?;
            let parsed = parse_ts_object(&content)?;
            let templates_val = if parsed.is_object() {
                parsed.get("templates").cloned().unwrap_or(serde_json::Value::Array(vec![]))
            } else {
                parsed
            };
            let list: Vec<TemplateMeta> = serde_json::from_value(templates_val)
                .map_err(|e| format!("Failed to parse templates: {}", e))?;
            return Ok(list);
        }
    }

    Ok(vec![])
}

fn get_template_member_flag(app_handle: &AppHandle, template_name: &str) -> Option<bool> {
    let candidates = [
        PathBuf::from("src-tauri/resources/templates/templates.ts"),
        PathBuf::from("resources/templates/templates.ts"),
        app_handle
            .path()
            .resolve("templates/templates.ts", BaseDirectory::Resource)
            .unwrap_or_default(),
    ];

    if let Ok(list) = load_metadata_from_candidates(&candidates) {
        return list
            .into_iter()
            .find(|t| t.id == template_name)
            .map(|t| t.member);
    }

    None
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

#[derive(Debug, Clone, Serialize)]
pub struct TemplateListResponse {
    pub templates: Vec<TemplateMeta>,
    pub has_premium: bool,
}

pub fn list_templates(app_handle: &AppHandle) -> Result<TemplateListResponse, String> {
    let mut all_templates: Vec<TemplateMeta> = vec![];

    let free_candidates = [
        PathBuf::from("src-tauri/resources/templates/templates.ts"),
        PathBuf::from("resources/templates/templates.ts"),
        app_handle.path().resolve("templates/templates.ts", BaseDirectory::Resource).unwrap_or_default(),
    ];

    if let Ok(mut list) = load_metadata_from_candidates(&free_candidates) {
        all_templates.append(&mut list);
    }

    let has_premium = all_templates.iter().any(|t| t.member);

    if all_templates.is_empty() {
        Err("No templates metadata found in resources".to_string())
    } else {
        Ok(TemplateListResponse {
            templates: all_templates,
            has_premium,
        })
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
