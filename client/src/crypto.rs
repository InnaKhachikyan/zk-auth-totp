use rand_core::{OsRng, RngCore};

pub fn salt_gen() -> [u8; 16] {
    let mut salt = [0u8; 16];
    OsRng.fill_bytes(&mut salt);
    salt
}


