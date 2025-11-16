use crate::security::{decrypt, encrypt, get_master_key};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),

    #[error("Encryption error: {0}")]
    EncryptionError(#[from] crate::security::encryption::EncryptionError),

    #[error("Keychain error: {0}")]
    KeychainError(#[from] crate::security::keychain::KeychainError),

    #[error("Provider '{0}' not found")]
    ProviderNotFound(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderConfig {
    pub provider_id: String,
    pub api_key: String, // Encrypted when stored, decrypted when loaded
    #[serde(default)]
    pub base_url: Option<String>,
    #[serde(default)]
    pub default_model: Option<String>,
    #[serde(default)]
    pub enabled: bool,
}

impl ProviderConfig {
    /// Create a masked version for safe frontend display
    pub fn masked(&self) -> MaskedProviderConfig {
        MaskedProviderConfig {
            provider_id: self.provider_id.clone(),
            has_api_key: !self.api_key.is_empty(),
            base_url: self.base_url.clone(),
            default_model: self.default_model.clone(),
            enabled: self.enabled,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MaskedProviderConfig {
    pub provider_id: String,
    pub has_api_key: bool,
    pub base_url: Option<String>,
    pub default_model: Option<String>,
    pub enabled: bool,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct AppConfig {
    pub providers: HashMap<String, ProviderConfig>,

    #[serde(default)]
    pub general: GeneralConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneralConfig {
    #[serde(default)]
    pub theme: String, // "light" or "dark"

    #[serde(default)]
    pub default_provider: Option<String>,
}

impl Default for GeneralConfig {
    fn default() -> Self {
        Self {
            theme: "light".to_string(),
            default_provider: None,
        }
    }
}

pub struct ConfigStore {
    config_path: PathBuf,
    master_key: Vec<u8>,
}

impl ConfigStore {
    /// Create a new ConfigStore with the app config directory
    pub fn new(app_data_dir: PathBuf) -> Result<Self, ConfigError> {
        // Ensure config directory exists
        fs::create_dir_all(&app_data_dir)?;

        let config_path = app_data_dir.join("config.enc");

        // Get or create master key from OS keychain
        let master_key = get_master_key()?;

        Ok(Self {
            config_path,
            master_key,
        })
    }

    /// Load config from disk, or create default if doesn't exist
    pub fn load(&self) -> Result<AppConfig, ConfigError> {
        if !self.config_path.exists() {
            tracing::info!("Config file not found, creating default");
            return Ok(AppConfig::default());
        }

        let encrypted_data = fs::read_to_string(&self.config_path)?;
        let decrypted_bytes = decrypt(&encrypted_data, &self.master_key)?;
        let config: AppConfig = serde_json::from_slice(&decrypted_bytes)?;

        tracing::info!("Loaded config with {} providers", config.providers.len());
        Ok(config)
    }

    /// Save config to disk (encrypted)
    pub fn save(&self, config: &AppConfig) -> Result<(), ConfigError> {
        let json = serde_json::to_string_pretty(config)?;
        let encrypted = encrypt(json.as_bytes(), &self.master_key)?;
        fs::write(&self.config_path, encrypted)?;

        tracing::info!("Saved config with {} providers", config.providers.len());
        Ok(())
    }

    /// Update or add a provider configuration
    pub fn update_provider(
        &self,
        provider_id: String,
        api_key: Option<String>,
        base_url: Option<String>,
        default_model: Option<String>,
        enabled: Option<bool>,
    ) -> Result<(), ConfigError> {
        let mut config = self.load()?;

        let provider_config = config
            .providers
            .entry(provider_id.clone())
            .or_insert_with(|| ProviderConfig {
                provider_id: provider_id.clone(),
                api_key: String::new(),
                base_url: None,
                default_model: None,
                enabled: false,
            });

        // Update fields
        if let Some(key) = api_key {
            provider_config.api_key = key;
        }
        if let Some(url) = base_url {
            provider_config.base_url = Some(url);
        }
        if let Some(model) = default_model {
            provider_config.default_model = Some(model);
        }
        if let Some(en) = enabled {
            provider_config.enabled = en;
        }

        self.save(&config)?;
        Ok(())
    }

    /// Get a specific provider's config
    pub fn get_provider(&self, provider_id: &str) -> Result<ProviderConfig, ConfigError> {
        let config = self.load()?;
        config
            .providers
            .get(provider_id)
            .cloned()
            .ok_or_else(|| ConfigError::ProviderNotFound(provider_id.to_string()))
    }

    /// Get all providers (masked for frontend)
    pub fn get_all_providers_masked(&self) -> Result<Vec<MaskedProviderConfig>, ConfigError> {
        let config = self.load()?;
        Ok(config
            .providers
            .values()
            .map(|p| p.masked())
            .collect())
    }

    /// Delete a provider
    pub fn delete_provider(&self, provider_id: &str) -> Result<(), ConfigError> {
        let mut config = self.load()?;
        config.providers.remove(provider_id);
        self.save(&config)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_config_roundtrip() {
        let temp_dir = TempDir::new().unwrap();
        let store = ConfigStore::new(temp_dir.path().to_path_buf()).unwrap();

        // Create config
        let mut config = AppConfig::default();
        config.providers.insert(
            "test".to_string(),
            ProviderConfig {
                provider_id: "test".to_string(),
                api_key: "secret123".to_string(),
                base_url: Some("https://api.example.com".to_string()),
                default_model: Some("model-1".to_string()),
                enabled: true,
            },
        );

        // Save and reload
        store.save(&config).unwrap();
        let loaded = store.load().unwrap();

        assert_eq!(loaded.providers.len(), 1);
        let provider = loaded.providers.get("test").unwrap();
        assert_eq!(provider.api_key, "secret123");
        assert_eq!(provider.base_url.as_deref(), Some("https://api.example.com"));
    }
}
