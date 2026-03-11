use rpassword::read_password;
use std::io::{self,Write};
use shared::crypto::schnorr::keypair_gen;
use shared::crypto::kdf::{salt_gen, derive_key_from_password};
use shared::crypto::aes::encrypt_secret_x;

pub fn read_username() -> String {
    let mut username = String::new();
    print!("Enter Username: ");
    io::stdout().flush().expect("Failed to flush stdout");
    io::stdin().read_line(&mut username).expect("Failed to read username");
    let username = username.trim();
    let username = username.to_string();
    username
}

pub fn read_user_password() -> String {
    print!("Enter Password: ");
    io::stdout().flush().unwrap();

    let password = read_password().expect("Failed to read password");

    password
}

pub fn register_user() {

    let username = read_username();
    let password = read_user_password();

    let (x,y) = keypair_gen();
    let x = x.to_bytes();
    let salt = salt_gen();

    let key = derive_key_from_password(&password, &salt).expect("Failed to derive the key");

    let (ciphertext, nonce) = encrypt_secret_x(&key, &x);
    //store username : salt : enc_x on disk
    //send to server username : pub_key
}
