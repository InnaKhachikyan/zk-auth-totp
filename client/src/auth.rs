use rpassword::read_password;
use std::io::{self,Write};
use shared::crypto::schnorr::keypair_gen;
use shared::crypto::kdf::{salt_gen, derive_key_from_password};
use shared::crypto::aes::encrypt_secret_x;
use crate::storage::{LocalUserRecord, store_local_user};
use crate::network::{send_register_request, send_login_request};

fn read_username() -> String {
    let mut username = String::new();
    print!("Enter Username: ");
    io::stdout().flush().expect("Failed to flush stdout");
    io::stdin().read_line(&mut username).expect("Failed to read username");
    let username = username.trim();
    let username = username.to_string();
    username
}

fn read_user_password() -> String {
    print!("Enter Password: ");
    io::stdout().flush().unwrap();

    let password = read_password().expect("Failed to read password");

    password
}

pub fn register() {

    let username = read_username();
    let password = read_user_password();

    let (x,y) = keypair_gen();
    let x = x.to_bytes();
    let salt = salt_gen();

    let key = derive_key_from_password(&password, &salt).expect("Failed to derive the key");

    let (ciphertext, nonce) = encrypt_secret_x(&key, &x);
    let record = LocalUserRecord {username: username.clone(), salt, nonce, enc_x: ciphertext};
    store_local_user(&record);
    let y_bytes: [u8; 32] = y.compress().to_bytes();
    send_register_request(username, y_bytes);
}

pub fn login() {
    let username = read_username();
    //generate DH a
    //send DH A and username
    //receive server_nonce and DH B
    //derive DH key
    let password = read_user_password();
    //find the user's file
    //extract salt and derive the key with kdf
    //decrypt x from enc_x with the key
    //derive DH key, prepare TOTP
    //prepare schnorr proof
    //send the proof to the server
    //receive response from the server
    //give access or reject
}

