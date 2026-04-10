use shared::messages::{RegisterRequest, LoginStartRequest, LoginProofRequest, RegisterResponse, LoginStartResponse, LoginResult, ServerMessage};
use shared::crypto::dh::dh_gen;
use crate::storage::{UserRecord, store_user_record, user_exists};
use rand_core::{RngCore, OsRng};

fn generate_nonce() -> [u8;16] {
    let mut nonce = [0u8;16];
    OsRng.fill_bytes(&mut nonce);
    nonce
}

pub fn handle_register(request: RegisterRequest) -> ServerMessage {
    println!("Received register request");
    let record = UserRecord {username: request.username, pub_key: request.y};
    store_user_record(&record);

    ServerMessage::Register(RegisterResponse::Success)
}

pub fn handle_login_start(request: LoginStartRequest) -> ServerMessage {
    println!("Received login request");
    let username = &request.username;
    let path = format!("server/data/{}.json", username);
    if !user_exists(&path) {
        return ServerMessage::LoginStart(LoginStartResponse::Failure {
            message: "User does not exist".to_string(),
        });
    }
    let (secret_local, pub_local) = dh_gen();
    let nonce = generate_nonce();

    ServerMessage::LoginStart(LoginStartResponse::Success {
        nonce, 
        pub_dh: pub_local.compress().to_bytes(),
    })
}

pub fn handle_login_proof(request: LoginProofRequest) -> ServerMessage {
    ServerMessage::LoginStart(LoginStartResponse::Failure {
        message: "not implemented".to_string(),
    })
}
