use std::fs;
use std::io::Write;
use std::path::Path;

use aes_gcm::aead::{Aead, KeyInit, OsRng};
use aes_gcm::{Aes256Gcm, Key, Nonce};
use base64::engine::general_purpose::STANDARD;
use base64::Engine;
use rand::RngCore;

use crate::domain::errors::DomainError;
use crate::domain::ports::SecretCipher;

const KEY_LEN: usize = 32;
const NONCE_LEN: usize = 12;

pub struct AesGcmSecretCipher {
    cipher: Aes256Gcm,
}

impl AesGcmSecretCipher {
    pub fn from_bytes(key_bytes: [u8; KEY_LEN]) -> Self {
        let key = Key::<Aes256Gcm>::from_slice(&key_bytes);
        Self {
            cipher: Aes256Gcm::new(key),
        }
    }

    pub fn load_or_create(
        env_key: Option<&str>,
        key_file_path: &Path,
    ) -> Result<Self, DomainError> {
        if let Some(raw) = env_key {
            let bytes = decode_key_material(raw)?;
            return Ok(Self::from_bytes(bytes));
        }

        if key_file_path.exists() {
            let content = fs::read(key_file_path)
                .map_err(|e| DomainError::Internal(format!("failed to read master key: {e}")))?;
            let bytes = if content.len() == KEY_LEN {
                let mut arr = [0u8; KEY_LEN];
                arr.copy_from_slice(&content);
                arr
            } else {
                let text = String::from_utf8(content).map_err(|_| {
                    DomainError::Internal("master key file has invalid format".to_string())
                })?;
                decode_key_material(&text)?
            };
            return Ok(Self::from_bytes(bytes));
        }

        if let Some(parent) = key_file_path.parent() {
            fs::create_dir_all(parent).map_err(|e| {
                DomainError::Internal(format!("failed to create key directory: {e}"))
            })?;
        }

        let mut key_bytes = [0u8; KEY_LEN];
        OsRng.fill_bytes(&mut key_bytes);
        write_restricted_key_file(key_file_path, &key_bytes)?;
        Ok(Self::from_bytes(key_bytes))
    }
}

impl SecretCipher for AesGcmSecretCipher {
    fn encrypt(&self, plaintext: &str) -> Result<String, DomainError> {
        let mut nonce_bytes = [0u8; NONCE_LEN];
        OsRng.fill_bytes(&mut nonce_bytes);
        let nonce = Nonce::from_slice(&nonce_bytes);
        let ciphertext = self
            .cipher
            .encrypt(nonce, plaintext.as_bytes())
            .map_err(|_| DomainError::Internal("encryption failure".to_string()))?;

        let mut combined = Vec::with_capacity(NONCE_LEN + ciphertext.len());
        combined.extend_from_slice(&nonce_bytes);
        combined.extend_from_slice(&ciphertext);
        Ok(STANDARD.encode(combined))
    }

    fn decrypt(&self, encoded: &str) -> Result<String, DomainError> {
        let combined = STANDARD
            .decode(encoded.trim())
            .map_err(|_| DomainError::Internal("invalid encrypted payload".to_string()))?;
        if combined.len() <= NONCE_LEN {
            return Err(DomainError::Internal(
                "encrypted payload is malformed".to_string(),
            ));
        }
        let (nonce_bytes, ciphertext) = combined.split_at(NONCE_LEN);
        let nonce = Nonce::from_slice(nonce_bytes);
        let plaintext = self
            .cipher
            .decrypt(nonce, ciphertext)
            .map_err(|_| DomainError::Internal("decryption failure".to_string()))?;
        String::from_utf8(plaintext)
            .map_err(|_| DomainError::Internal("decrypted payload is not valid UTF-8".to_string()))
    }
}

fn decode_key_material(raw: &str) -> Result<[u8; KEY_LEN], DomainError> {
    let trimmed = raw.trim();
    if trimmed.len() == KEY_LEN * 2 {
        let decoded = hex::decode(trimmed)
            .map_err(|_| DomainError::Validation("invalid hex master key".to_string()))?;
        return decoded
            .try_into()
            .map_err(|_| DomainError::Validation("master key must be 32 bytes".to_string()));
    }
    let decoded = STANDARD
        .decode(trimmed)
        .map_err(|_| DomainError::Validation("invalid base64 master key".to_string()))?;
    decoded
        .try_into()
        .map_err(|_| DomainError::Validation("master key must be 32 bytes".to_string()))
}

#[cfg(unix)]
fn write_restricted_key_file(path: &Path, key_bytes: &[u8; KEY_LEN]) -> Result<(), DomainError> {
    use std::os::unix::fs::OpenOptionsExt;

    let mut file = fs::OpenOptions::new()
        .create_new(true)
        .write(true)
        .mode(0o600)
        .open(path)
        .map_err(|e| DomainError::Internal(format!("failed to create master key file: {e}")))?;
    file.write_all(key_bytes)
        .map_err(|e| DomainError::Internal(format!("failed to write master key file: {e}")))?;
    Ok(())
}

#[cfg(not(unix))]
fn write_restricted_key_file(path: &Path, key_bytes: &[u8; KEY_LEN]) -> Result<(), DomainError> {
    let mut file = fs::OpenOptions::new()
        .create_new(true)
        .write(true)
        .open(path)
        .map_err(|e| DomainError::Internal(format!("failed to create master key file: {e}")))?;
    file.write_all(key_bytes)
        .map_err(|e| DomainError::Internal(format!("failed to write master key file: {e}")))?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::AesGcmSecretCipher;
    use crate::domain::ports::SecretCipher;

    #[test]
    fn round_trips_encryption_and_uses_unique_nonces() {
        let cipher = AesGcmSecretCipher::from_bytes([7u8; 32]);
        let secret = "{\"webhook_url\":\"https://discord.com/api/webhooks/1/abc\"}";

        let c1 = cipher.encrypt(secret).expect("encrypt 1");
        let c2 = cipher.encrypt(secret).expect("encrypt 2");
        assert_ne!(c1, c2);

        let p1 = cipher.decrypt(&c1).expect("decrypt 1");
        let p2 = cipher.decrypt(&c2).expect("decrypt 2");
        assert_eq!(p1, secret);
        assert_eq!(p2, secret);
    }

    #[test]
    fn rejects_tampered_ciphertext() {
        let cipher = AesGcmSecretCipher::from_bytes([9u8; 32]);
        let encrypted = cipher.encrypt("token-123").expect("encrypt");
        let mut bytes = encrypted.into_bytes();
        bytes[0] = if bytes[0] == b'A' { b'B' } else { b'A' };
        let tampered = String::from_utf8(bytes).expect("utf8");
        assert!(cipher.decrypt(&tampered).is_err());
    }
}
