use rpassword::read_password;
use curve25519_dalek::ristretto::{CompressedRistretto, RistrettoPoint};
use std::io::{self,Write};
use shared::crypto::schnorr::keypair_gen;
use shared::crypto::kdf::{salt_gen, derive_key_from_password};
use shared::crypto::aes::{encrypt_secret_x, decrypt_secret_x};
use shared::crypto::dh::{dh_gen, derive_dh_key};
use crate::storage::{LocalUserRecord, store_local_user};
use crate::network::{start_connection, send_register_request, send_login_request, read_register_response, read_login_start_response};
use shared::messages::{ClientMessage, RegisterRequest, LoginStartRequest, LoginProofRequest, ServerMessage, RegisterResponse, LoginStartResponse, LoginResult};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct UserData {
    pub username: String,
    pub salt: [u8; 16],
    pub nonce: [u8; 12],
    pub enc_x: Vec<u8>,
}

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

fn load_user(username: &str) -> Result<UserData, Box<dyn std::error::Error>> {
    let path = format!("client/data/{}.json", username);
    let data = std::fs::read_to_string(&path)?;
    let user = serde_json::from_str(&data)?;

    Ok(user)
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
    let y_bytes: [u8; 32] = y.compress().to_bytes();
    let mut client_connection = start_connection();
    send_register_request(&mut client_connection, username.clone(), y_bytes);
    let response = read_register_response(&mut client_connection);

    match response {
        RegisterResponse::Success => { 
            store_local_user(&record);
            println!("User '{}' successfully reigstered", username);
        }
        RegisterResponse::Failure {message: _ } => {
            println!("Registration failed");
        }
    }
}

pub fn login() {
    let username = read_username();
    let password = read_user_password();
    let user = match load_user(&username) {
        Ok(u) => u,
        Err(e) => {
            eprintln!("Failed to load user: {}", e);
            return;
        }
    };

    let key = derive_key_from_password(&password, &user.salt).expect("Failed to derive the key");
    let secret_x = decrypt_secret_x(&key, &user.nonce, user.enc_x);

    let (secret_local, pub_local) = dh_gen();
    let pub_local_bytes: [u8; 32] = pub_local.compress().to_bytes();

    let mut client_connection = start_connection();
    send_login_request(&mut client_connection, username, pub_local_bytes); 
    let server_response = read_login_start_response(&mut client_connection);
    let (server_nonce, server_dh_compressed) = match server_response {
        LoginStartResponse::Success {nonce, pub_dh} => (nonce, pub_dh),
        LoginStartResponse::Failure { message } => panic!("Login failed: {}", message),
    };
    let compressed = CompressedRistretto(server_dh_compressed);
    let server_dh: RistrettoPoint = compressed.decompress().expect("Invalid Ristretto point");
    let dh_key: RistrettoPoint = derive_dh_key(secret_local, server_dh);

    //prepare TOTP
    //prepare schnorr proof
    //send the proof to the server
    //receive response from the server
    //give access or reject
}

