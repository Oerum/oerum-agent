use aes_gcm::{
    aead::{Aead, KeyInit},
    Aes256Gcm, Nonce,
};
use anyhow::{anyhow, bail, Result};
use argon2::{Algorithm, Argon2, Params, Version};
use rand::{rngs::OsRng, RngCore};

/// On-wire layout:
///   byte 0:           VERSION (currently 1)
///   bytes 1..17:      salt (16 bytes)
///   bytes 17..29:     nonce (12 bytes)
///   bytes 29..:       AES-256-GCM ciphertext (includes 16-byte tag)
///
/// Version 1 uses Argon2id (m=64 MiB, t=3, p=1) for key derivation.
const VERSION: u8 = 1;
const SALT_LEN: usize = 16;
const NONCE_LEN: usize = 12;
const KEY_LEN: usize = 32;

/// Argon2id parameters tuned to current OWASP guidance:
///   memory: 64 MiB
///   iterations: 3
///   parallelism: 1
const ARGON2_MEMORY_KIB: u32 = 64 * 1024;
const ARGON2_ITERATIONS: u32 = 3;
const ARGON2_PARALLELISM: u32 = 1;

fn derive_key(passphrase: &str, salt: &[u8]) -> Result<[u8; KEY_LEN]> {
    let params = Params::new(
        ARGON2_MEMORY_KIB,
        ARGON2_ITERATIONS,
        ARGON2_PARALLELISM,
        Some(KEY_LEN),
    )
    .map_err(|e| anyhow!("invalid argon2 params: {e}"))?;
    let argon = Argon2::new(Algorithm::Argon2id, Version::V0x13, params);
    let mut key = [0u8; KEY_LEN];
    argon
        .hash_password_into(passphrase.as_bytes(), salt, &mut key)
        .map_err(|e| anyhow!("argon2 key derivation failed: {e}"))?;
    Ok(key)
}

pub fn encrypt(payload: &[u8], passphrase: &str) -> Result<Vec<u8>> {
    if passphrase.is_empty() {
        bail!("passphrase must not be empty");
    }

    let mut salt = [0u8; SALT_LEN];
    OsRng.fill_bytes(&mut salt);
    let mut nonce_bytes = [0u8; NONCE_LEN];
    OsRng.fill_bytes(&mut nonce_bytes);

    let key = derive_key(passphrase, &salt)?;
    let cipher =
        Aes256Gcm::new_from_slice(&key).map_err(|e| anyhow!("invalid key length: {e}"))?;
    let ciphertext = cipher
        .encrypt(Nonce::from_slice(&nonce_bytes), payload)
        .map_err(|e| anyhow!("encryption failed: {e}"))?;

    let mut out = Vec::with_capacity(1 + SALT_LEN + NONCE_LEN + ciphertext.len());
    out.push(VERSION);
    out.extend_from_slice(&salt);
    out.extend_from_slice(&nonce_bytes);
    out.extend_from_slice(&ciphertext);
    Ok(out)
}

pub fn decrypt(payload: &[u8], passphrase: &str) -> Result<Vec<u8>> {
    if payload.len() < 1 + SALT_LEN + NONCE_LEN {
        bail!("payload too short");
    }
    let version = payload[0];
    if version != VERSION {
        bail!("unsupported brain-sync payload version: {version}");
    }

    let salt = &payload[1..1 + SALT_LEN];
    let nonce_bytes = &payload[1 + SALT_LEN..1 + SALT_LEN + NONCE_LEN];
    let ciphertext = &payload[1 + SALT_LEN + NONCE_LEN..];

    let key = derive_key(passphrase, salt)?;
    let cipher =
        Aes256Gcm::new_from_slice(&key).map_err(|e| anyhow!("invalid key length: {e}"))?;

    cipher
        .decrypt(Nonce::from_slice(nonce_bytes), ciphertext)
        .map_err(|e| anyhow!("decryption failed (wrong passphrase or corrupt payload): {e}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roundtrip_recovers_plaintext() {
        let plaintext = b"hello, brain";
        let ct = encrypt(plaintext, "correct horse battery staple").expect("encrypt");
        let pt = decrypt(&ct, "correct horse battery staple").expect("decrypt");
        assert_eq!(pt, plaintext);
    }

    #[test]
    fn wrong_passphrase_fails() {
        let ct = encrypt(b"secret", "right passphrase").expect("encrypt");
        assert!(decrypt(&ct, "wrong passphrase").is_err());
    }

    #[test]
    fn rejects_unknown_version() {
        let mut ct = encrypt(b"data", "p").expect("encrypt");
        ct[0] = 99;
        assert!(decrypt(&ct, "p").is_err());
    }

    #[test]
    fn rejects_truncated_payload() {
        assert!(decrypt(b"\x01short", "p").is_err());
    }

    #[test]
    fn rejects_empty_passphrase() {
        assert!(encrypt(b"data", "").is_err());
    }
}
