use chacha20poly1305::{
    aead::{Aead, KeyInit, OsRng},
    ChaCha20Poly1305, Nonce,
};
use rand::RngCore;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum EncryptionError {
    #[error("Encryption failed: {0}")]
    EncryptionFailed(String),

    #[error("Decryption failed: {0}")]
    DecryptionFailed(String),

    #[error("Invalid key length: expected 32 bytes, got {0}")]
    InvalidKeyLength(usize),

    #[error("Invalid ciphertext format")]
    InvalidFormat,

    #[error("Base64 decode error: {0}")]
    Base64Error(#[from] base64::DecodeError),
}

const NONCE_SIZE: usize = 12; // 96 bits for ChaCha20Poly1305

/// Encrypt plaintext using ChaCha20Poly1305 with a 256-bit key
/// Returns base64-encoded: [nonce || ciphertext || tag]
pub fn encrypt(plaintext: &[u8], key: &[u8]) -> Result<String, EncryptionError> {
    if key.len() != 32 {
        return Err(EncryptionError::InvalidKeyLength(key.len()));
    }

    // Create cipher instance
    let cipher = ChaCha20Poly1305::new_from_slice(key)
        .map_err(|e| EncryptionError::EncryptionFailed(e.to_string()))?;

    // Generate random nonce
    let mut nonce_bytes = [0u8; NONCE_SIZE];
    OsRng.fill_bytes(&mut nonce_bytes);
    let nonce = Nonce::from_slice(&nonce_bytes);

    // Encrypt
    let ciphertext = cipher
        .encrypt(nonce, plaintext)
        .map_err(|e| EncryptionError::EncryptionFailed(e.to_string()))?;

    // Combine: nonce || ciphertext (ciphertext already includes the auth tag)
    let mut combined = Vec::with_capacity(NONCE_SIZE + ciphertext.len());
    combined.extend_from_slice(&nonce_bytes);
    combined.extend_from_slice(&ciphertext);

    // Encode to base64
    Ok(base64::encode(&combined))
}

/// Decrypt base64-encoded ciphertext
/// Expected format: base64([nonce || ciphertext || tag])
pub fn decrypt(ciphertext_b64: &str, key: &[u8]) -> Result<Vec<u8>, EncryptionError> {
    if key.len() != 32 {
        return Err(EncryptionError::InvalidKeyLength(key.len()));
    }

    // Decode base64
    let combined = base64::decode(ciphertext_b64)?;

    // Extract nonce and ciphertext
    if combined.len() < NONCE_SIZE {
        return Err(EncryptionError::InvalidFormat);
    }

    let (nonce_bytes, ciphertext) = combined.split_at(NONCE_SIZE);
    let nonce = Nonce::from_slice(nonce_bytes);

    // Create cipher and decrypt
    let cipher = ChaCha20Poly1305::new_from_slice(key)
        .map_err(|e| EncryptionError::DecryptionFailed(e.to_string()))?;

    let plaintext = cipher
        .decrypt(nonce, ciphertext)
        .map_err(|e| EncryptionError::DecryptionFailed(e.to_string()))?;

    Ok(plaintext)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encrypt_decrypt_roundtrip() {
        let key = [0u8; 32]; // Test key
        let plaintext = b"Hello, World! This is a secret message.";

        let encrypted = encrypt(plaintext, &key).expect("Encryption failed");
        let decrypted = decrypt(&encrypted, &key).expect("Decryption failed");

        assert_eq!(plaintext, decrypted.as_slice());
    }

    #[test]
    fn test_different_keys_fail() {
        let key1 = [0u8; 32];
        let key2 = [1u8; 32];
        let plaintext = b"Secret";

        let encrypted = encrypt(plaintext, &key1).expect("Encryption failed");
        let result = decrypt(&encrypted, &key2);

        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_key_length() {
        let short_key = [0u8; 16];
        let plaintext = b"Test";

        let result = encrypt(plaintext, &short_key);
        assert!(result.is_err());
    }
}
