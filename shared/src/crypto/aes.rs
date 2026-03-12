use aes_gcm::{aead::{Aead, KeyInit}, Aes256Gcm, Nonce,};
use rand_core::{RngCore, OsRng};

pub fn encrypt_secret_x(key: &[u8; 32], x: &[u8; 32]) -> (Vec<u8>, [u8; 12]) {
    let cipher = Aes256Gcm::new_from_slice(key).expect("Failed to initialize AES-256-GCM cipher");

    let mut nonce_bytes = [0u8; 12];
    OsRng.fill_bytes(&mut nonce_bytes);

    let nonce = Nonce::from_slice(&nonce_bytes);
    let ciphertext = cipher.encrypt(nonce, x.as_ref()).expect("Failed to encrypt secret x");

    (ciphertext, nonce_bytes)
}        

pub fn decrypt_secret_x(key: &[u8; 32], nonce: &[u8; 12], ciphertext: Vec<u8>) -> [u8; 32] {
    let cipher = Aes256Gcm::new_from_slice(key).expect("Failed to initialize AES-256-GCM cipher");
    let nonce = Nonce::from_slice(nonce);
    let plaintext = cipher.decrypt(nonce, ciphertext.as_ref()).expect("Failed to decrypt secret x");
    if plaintext.len() != 32 {
        panic!("The decrypted x has an invalid length");
    }
    let mut x = [0u8; 32];
    x.copy_from_slice(&plaintext);
    x
}

