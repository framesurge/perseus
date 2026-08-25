use crate::server::errors::ServiceError;
use aes_gcm::aead::{generic_array::GenericArray, Aead, Payload};
use aes_gcm::Aes256Gcm;
use aes_gcm::{AeadInPlace, KeyInit};
use argon2::password_hash::rand_core::RngCore;
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use base64::{engine::general_purpose, Engine as _};
use bcrypt::{hash, verify, DEFAULT_COST};
use unicode_normalization::UnicodeNormalization;

pub(crate) const NONCE_LEN: usize = 12;
pub(crate) const TAG_LEN: usize = 16;

/// Encrypts the plain text with authenticated encryption providing
/// confidentiality, integrity, and authenticity.
pub fn encrypt(plain_text: &str, aad: &str, secret_key: &[u8]) -> Result<String, ServiceError> {
    let val = plain_text.as_bytes();
    let mut data = vec![0; NONCE_LEN + val.len() + TAG_LEN];

    let (nonce, in_out) = data.split_at_mut(NONCE_LEN);
    let (in_out, tag) = in_out.split_at_mut(val.len());
    in_out.copy_from_slice(val);

    OsRng.fill_bytes(nonce);
    let nonce = GenericArray::clone_from_slice(nonce);

    let aad = aad.as_bytes();
    let aead = Aes256Gcm::new(GenericArray::from_slice(secret_key));
    let aad_tag = aead
        .encrypt_in_place_detached(&nonce, aad, in_out)
        .map_err(|e| ServiceError::FaultySetup(format!("Encryption failure: {}", e)))?;

    tag.copy_from_slice(&aad_tag);

    Ok(general_purpose::STANDARD.encode(&data))
}

/// Given an encrypted value and an aad, verifies and decrypts the sealed value.
pub fn decrypt(cipher: &str, aad: &str, secret_key: &[u8]) -> Result<String, ServiceError> {
    let data = general_purpose::STANDARD
        .decode(cipher)
        .map_err(|_| ServiceError::FaultySetup("bad base64 value".into()))?;

    if data.len() <= NONCE_LEN {
        return Err(ServiceError::FaultySetup(
            "length of decoded data is <= NONCE_LEN".into(),
        ));
    }

    let (nonce, cipher) = data.split_at(NONCE_LEN);
    let payload = Payload {
        msg: cipher,
        aad: aad.as_bytes(),
    };

    let aead = Aes256Gcm::new(GenericArray::from_slice(secret_key));
    let decrypted = aead
        .decrypt(GenericArray::from_slice(nonce), payload)
        .map_err(|e| ServiceError::FaultySetup(e.to_string()))?;

    let decrypted =
        String::from_utf8(decrypted).map_err(|e| ServiceError::FaultySetup(e.to_string()))?;

    Ok(decrypted)
}

/// Hash a password using either bcrypt or argon2
pub async fn password_hash(password: &str, use_bcrypt: bool) -> Result<String, ServiceError> {
    let normalised_password = password.nfkc().collect::<String>();

    let hashed_password = if use_bcrypt {
        hash(&normalised_password, DEFAULT_COST).map_err(|_| ServiceError::Unauthorized("Password hashing failed".to_string()))?
    } else {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        let vec_bytes = normalised_password.into_bytes();

        argon2
            .hash_password(&vec_bytes, &salt)
            .map_err(|e| ServiceError::FaultySetup(e.to_string()))?
            .to_string()
    };

    Ok(hashed_password)
}

/// Verify a password against a hash
pub async fn verify_hash(
    password: &str,
    hashed_password: &str,
    use_bcrypt: bool,
) -> Result<bool, ServiceError> {
    let normalised_password = password.nfkc().collect::<String>();

    let is_valid = if use_bcrypt {
        verify(&normalised_password, hashed_password).map_err(|_| ServiceError::Unauthorized("Password verification failed".to_string()))?
    } else {
        let argon2 = Argon2::default();
        let parsed_hash = PasswordHash::new(hashed_password)
            .map_err(|e| ServiceError::FaultySetup(e.to_string()))?;
        let vec_bytes = normalised_password.into_bytes();
        argon2.verify_password(&vec_bytes, &parsed_hash).is_ok()
    };

    Ok(is_valid)
}

/// Convert hex string to bytes
pub fn hex_to_bytes(hex: &str) -> Result<Vec<u8>, std::num::ParseIntError> {
    (0..hex.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&hex[i..i + 2], 16))
        .collect()
}

/// Generate a random OTP code
pub fn generate_otp() -> u32 {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    rng.gen_range(10000..99999)
}

/// Generate random bytes for session tokens
pub fn generate_random_bytes_32() -> [u8; 32] {
    use rand_core::RngCore;
    let mut bytes = [0u8; 32];
    rand_core::OsRng.fill_bytes(&mut bytes);
    bytes
}

/// Generate 8 random bytes (for selectors)
pub fn generate_random_bytes_8() -> [u8; 8] {
    use rand_core::RngCore;
    let mut bytes = [0u8; 8];
    rand_core::OsRng.fill_bytes(&mut bytes);
    bytes
}

/// Generate 24 random bytes (for verifiers)
pub fn generate_random_bytes_24() -> [u8; 24] {
    use rand_core::RngCore;
    let mut bytes = [0u8; 24];
    rand_core::OsRng.fill_bytes(&mut bytes);
    bytes
}

/// Hash bytes using SHA256
pub fn hash_sha256(bytes: &[u8]) -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    hex::encode(hasher.finalize())
}

/// Compare hashes in constant time to prevent timing attacks
pub fn constant_time_compare(a: &str, b: &str) -> bool {
    a.bytes()
        .zip(b.bytes())
        .fold(0, |acc, (a, b)| acc | (a ^ b))
        == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_encryption() {
        let random_bytes = generate_random_bytes_32();
        let cipher_text = encrypt("Hello World", "AAD", &random_bytes).unwrap();
        let plain_text = decrypt(&cipher_text, "AAD", &random_bytes).unwrap();
        assert_eq!(plain_text, "Hello World");
    }

    #[tokio::test]
    async fn test_password_argon() {
        let password = "test_password123";
        let hash = password_hash(password, false).await.unwrap();
        let verified = verify_hash(password, &hash, false).await.unwrap();
        assert!(verified);
    }

    #[tokio::test]
    async fn test_password_bcrypt() {
        let password = "test_password123";
        let hash = password_hash(password, true).await.unwrap();
        let verified = verify_hash(password, &hash, true).await.unwrap();
        assert!(verified);
    }
}
