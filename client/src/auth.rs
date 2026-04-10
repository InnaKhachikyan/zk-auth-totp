use rpassword::read_password;
use curve25519_dalek::ristretto::{CompressedRistretto, RistrettoPoint};
use std::io::{self,Write};
use shared::crypto::schnorr::keypair_gen;
use shared::crypto::kdf::{salt_gen, derive_key_from_password};
use shared::crypto::aes::encrypt_secret_x;
use shared::crypto::dh::{dh_gen, derive_dh_key};
use crate::storage::{LocalUserRecord, store_local_user};
use crate::network::{start_connection, send_register_request, send_login_request, read_login_start_response};
use shared::messages::LoginStartResponse;

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
    let mut stream = start_connection();
    send_register_request(username, y_bytes, &mut stream);
}

pub fn login() {
    let username = read_username();
    let (secret_local, pub_local) = dh_gen();
    let pub_local_bytes: [u8; 32] = pub_local.compress().to_bytes();

    let mut stream = start_connection();
    send_login_request(username, pub_local_bytes, &mut stream); 
    let server_response = read_login_start_response(&mut stream);
    let (server_nonc, server_dh_compressed) = match server_response {
        LoginStartResponse::Success {nonce, pub_dh} => (nonce, pub_dh),
        LoginStartResponse::Failure { message } => panic!("Login failed: {}", message),
    };
    let compressed = CompressedRistretto(server_dh_compressed);
    let server_dh: RistrettoPoint = compressed.decompress().expect("Invalid Ristretto point");
    let dh_key: RistrettoPoint = derive_dh_key(secret_local, server_dh);

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

