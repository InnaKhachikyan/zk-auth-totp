use rand_core::{OsRng, RngCore};
use argon2::{Algorithm, Argon2, Params, Version};

pub fn salt_gen() -> [u8; 16] {
    let mut salt = [0u8; 16];
    OsRng.fill_bytes(&mut salt);
    salt
}

pub fn derive_key_from_password(password: &str, salt: &[u8]) -> Result<[u8; 32], argon2::Error> {
    let params = Params::new(19456, 2, 1, Some(32))?; //mem=~19MB, iterations=2, parallelism=1, output length 32 bytes
    let argon2 = Argon2::new(Algorithm::Argon2id, Version::V0x13, params);
    let mut output_key = [0u8; 32];

    argon2.hash_password_into(password.as_bytes(), salt, &mut output_key)?;

    Ok(output_key)
}
