use keyring::Entry;
use rand::RngCore;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum KeychainError {
    #[error("Keychain error: {0}")]
    KeyringError(#[from] keyring::Error),

    #[error("Base64 decode error: {0}")]
    Base64Error(#[from] base64::DecodeError),

    #[error("Invalid key format: expected 32 bytes, got {0}")]
    InvalidKeyFormat(usize),
}

const SERVICE_NAME: &str = "llm_workbench_master_key";
const ACCOUNT_NAME: &str = "master";

/// Get or create the master encryption key from OS keychain
/// On first run, generates and stores a new 256-bit random key
pub fn get_master_key() -> Result<Vec<u8>, KeychainError> {
    let entry = Entry::new(SERVICE_NAME, ACCOUNT_NAME)?;

    match entry.get_password() {
        Ok(password) => {
            // Decode existing key from base64
            let key = base64::decode(password)?;
            if key.len() != 32 {
                return Err(KeychainError::InvalidKeyFormat(key.len()));
            }
            tracing::info!("Retrieved master key from OS keychain");
            Ok(key)
        }
        Err(keyring::Error::NoEntry) => {
            // First run: generate new key
            tracing::info!("Generating new master key (first run)");
            let key = generate_master_key()?;
            store_master_key(&key)?;
            Ok(key)
        }
        Err(e) => Err(KeychainError::KeyringError(e)),
    }
}

/// Store the master key in OS keychain
pub fn store_master_key(key: &[u8]) -> Result<(), KeychainError> {
    if key.len() != 32 {
        return Err(KeychainError::InvalidKeyFormat(key.len()));
    }

    let entry = Entry::new(SERVICE_NAME, ACCOUNT_NAME)?;
    let key_b64 = base64::encode(key);
    entry.set_password(&key_b64)?;

    tracing::info!("Stored master key in OS keychain");
    Ok(())
}

/// Generate a new random 256-bit master key
fn generate_master_key() -> Result<Vec<u8>, KeychainError> {
    let mut key = vec![0u8; 32];
    rand::rngs::OsRng.fill_bytes(&mut key);
    Ok(key)
}

/// Delete the master key from OS keychain (for testing or reset)
#[allow(dead_code)]
pub fn delete_master_key() -> Result<(), KeychainError> {
    let entry = Entry::new(SERVICE_NAME, ACCOUNT_NAME)?;
    entry.delete_password()?;
    tracing::info!("Deleted master key from OS keychain");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore] // Only run manually as it touches OS keychain
    fn test_keychain_operations() {
        // Clean up any existing key
        let _ = delete_master_key();

        // First call should generate a new key
        let key1 = get_master_key().expect("Failed to get master key");
        assert_eq!(key1.len(), 32);

        // Second call should retrieve the same key
        let key2 = get_master_key().expect("Failed to get master key");
        assert_eq!(key1, key2);

        // Clean up
        delete_master_key().expect("Failed to delete master key");
    }
}
