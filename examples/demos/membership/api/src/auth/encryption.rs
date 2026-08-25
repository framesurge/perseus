use crate::errors::ServiceError;
use aes_gcm::aead::{generic_array::GenericArray, Aead, Payload};
use aes_gcm::Aes256Gcm; // Or `Aes128Gcm`
use aes_gcm::{AeadInPlace, KeyInit};
use argon2::password_hash::rand_core::RngCore;
use argon2::{
    password_hash::{
        rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, Salt, SaltString,
    },
    Argon2,
};
use bcrypt::{hash, verify, DEFAULT_COST};
use unicode_normalization::UnicodeNormalization;

pub(crate) const NONCE_LEN: usize = 12;
pub(crate) const TAG_LEN: usize = 16;

/// Encrypts the plain text with authenticated encryption providing
/// confidentiality, integrity, and authenticity.
pub fn encrypt(plain_text: &str, aad: &str, secret_key: &[u8]) -> Result<String, ServiceError> {
    // Create a vec to hold the [nonce | cookie value | tag].
    let val = plain_text.as_bytes();
    let mut data = vec![0; NONCE_LEN + val.len() + TAG_LEN];

    // Split data into three: nonce, input/output, tag. Copy input.
    let (nonce, in_out) = data.split_at_mut(NONCE_LEN);
    let (in_out, tag) = in_out.split_at_mut(val.len());
    in_out.copy_from_slice(val);

    // Fill nonce piece with random data.
    OsRng.fill_bytes(nonce);
    let nonce = GenericArray::clone_from_slice(nonce);

    // Perform the actual sealing operation, using aad as
    // associated data to prevent value swapping.
    let aad = aad.as_bytes();
    let aead = Aes256Gcm::new(GenericArray::from_slice(secret_key));
    let aad_tag = aead
        .encrypt_in_place_detached(&nonce, aad, in_out)
        .expect("encryption failure!");

    // Copy the tag into the tag piece.
    tag.copy_from_slice(&aad_tag);

    // Base64 encode [nonce | encrypted value | tag].
    Ok(base64::encode(&data))
}

/// Given a encrypted value `str` and an aad, where the nonce is
/// prepended to the original value and then both are Base64 encoded,
/// verifies and decrypts the sealed value and returns it. If there's a
/// problem, returns an `Err` with a string describing the issue.
pub fn decrypt(cipher: &str, aad: &str, secret_key: &[u8]) -> Result<String, ServiceError> {
    let data =
        base64::decode(cipher).map_err(|_| ServiceError::FaultySetup("bad base64 value".into()))?;
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

// Derive a key from the master password hash and encrypt a protected key one more time.
pub async fn kdf_and_wrap(data: &str, password: &str, aead: &str) -> Result<String, ServiceError> {
    let mut bytes = [0u8; Salt::RECOMMENDED_LENGTH];
    OsRng.fill_bytes(&mut bytes);

    let salt =
        SaltString::b64_encode(&bytes).map_err(|e| ServiceError::FaultySetup(e.to_string()))?;

    let stretched_password = stretch_password(password, &salt).await?;

    if let Ok(cipher) = encrypt(data, aead, &stretched_password) {
        return Ok(format!("{}:{}", hex::encode(bytes), cipher));
    }

    Err(ServiceError::FaultySetup(
        "Problem encryptin protected key".into(),
    ))
}

async fn stretch_password(password: &str, salt: &SaltString) -> Result<Vec<u8>, ServiceError> {
    // Argon2 with default params (Argon2id v19)
    let argon2 = Argon2::default();
    let vec_bytes = password.as_bytes();

    let argoned: PasswordHash = argon2
        .hash_password(vec_bytes, salt)
        .map_err(|e| ServiceError::FaultySetup(e.to_string()))?;

    if let Some(hash) = argoned.hash {
        let stretched_key = hash.as_bytes().to_vec();
        return Ok(stretched_key);
    }

    Err(ServiceError::FaultySetup(
        "Problem stretching password".into(),
    ))
}

// Derive a key from the master password hash and encrypt a protected key one more time.
pub async fn kdf_and_unwrap(
    wrapped_data: &str,
    password: &str,
    aead: &str,
) -> Result<String, ServiceError> {
    let split: Vec<&str> = wrapped_data.split(':').collect();

    if split.len() == 2 {
        let decode_bytes =
            hex::decode(split[0]).map_err(|e| ServiceError::FaultySetup(e.to_string()))?;

        let salt = SaltString::b64_encode(&decode_bytes)
            .map_err(|e| ServiceError::FaultySetup(e.to_string()))?;

        let stretched_password = stretch_password(password, &salt).await?;

        return decrypt(split[1], aead, &stretched_password);
    }

    Err(ServiceError::FaultySetup("Problem with decryption".into()))
}

pub async fn password_hash(password: &str, use_bcrypt: bool) -> Result<String, ServiceError> {
    let normalised_password = password.nfkc().collect::<String>();

    let hashed_password = if use_bcrypt {
        hash(&normalised_password, DEFAULT_COST).map_err(|_| ServiceError::Unauthorized)?
    } else {
        let salt = SaltString::generate(&mut OsRng);

        // Argon2 with default params (Argon2id v19)
        let argon2 = Argon2::default();
        let vec_bytes = normalised_password.into_bytes();

        // Hash password to PHC string ($argon2id$v=19$...)
        argon2
            .hash_password(&vec_bytes, &salt)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))?
            .to_string()
    };

    Ok(hashed_password)
}

pub async fn verify_hash(
    password: &str,
    hashed_password: &str,
    use_bcrypt: bool,
) -> Result<bool, ServiceError> {
    let normalised_password = password.nfkc().collect::<String>();

    let is_valid = if use_bcrypt {
        verify(&normalised_password, hashed_password).map_err(|_| ServiceError::Unauthorized)?
    } else {
        // Argon2 with default params (Argon2id v19)
        let argon2 = Argon2::default();
        let parsed_hash = PasswordHash::new(hashed_password)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))?;
        let vec_bytes = normalised_password.into_bytes();
        argon2.verify_password(&vec_bytes, &parsed_hash).is_ok()
    };

    Ok(is_valid)
}

pub fn hex_to_bytes(hex: &str) -> Result<Vec<u8>, std::num::ParseIntError> {
    (0..hex.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&hex[i..i + 2], 16))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::Rng;

    #[tokio::test]
    async fn test_encryption() {
        let random_bytes = rand::thread_rng().gen::<[u8; 32]>();

        let cipher_text = encrypt("Hello World", "AAD", &random_bytes).unwrap();

        let plain_text = decrypt(&cipher_text, "AAD", &random_bytes).unwrap();

        assert_eq!(plain_text, "Hello World");
    }

    #[tokio::test]
    async fn test_wrapping() {
        let wrapped = kdf_and_wrap("Hello World", "password1234", "")
            .await
            .unwrap();

        let un_wrapped = kdf_and_unwrap(&wrapped, "password1234", "").await.unwrap();

        assert_eq!(un_wrapped, "Hello World");
    }
}
