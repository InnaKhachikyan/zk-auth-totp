use shared::messages::{RegisterRequest, LoginStartRequest, LoginProofRequest, RegisterResponse, LoginStartResponse, LoginResult, ServerMessage};
use shared::crypto::dh::{dh_gen, derive_dh_key};
use shared::crypto::schnorr::verify_proof;
use shared::crypto::totp::generate_totp;
use crate::storage::{UserRecord, store_user_record, user_exists, load_user};
use rand_core::{RngCore, OsRng};
use curve25519_dalek::ristretto::{RistrettoPoint, CompressedRistretto};
use curve25519_dalek::scalar::Scalar;
use std::time::{SystemTime, UNIX_EPOCH};

pub struct LoginSession {
    pub y: RistrettoPoint,
    pub a_bytes: [u8;32],
    pub b_secret: Scalar,
    pub b_bytes: [u8;32],
    pub nonce: [u8;16],
}

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

pub fn handle_login_start(request: LoginStartRequest) -> (ServerMessage, Option<LoginSession>) {
    println!("Received login request");
    let username = &request.username;
    let path = format!("server/data/{}.json", username);
    if !user_exists(&path) {
        return (
            ServerMessage::LoginStart(LoginStartResponse::Failure {
                message: "User does not exist".to_string(),
            }),
            None,
        );
    }
    let user = load_user(username).expect("Faied to load user");
    let (secret_local, pub_local) = dh_gen();
    let nonce = generate_nonce();
    let pub_local_bytes = pub_local.compress().to_bytes();
    let compressed_pub_key = CompressedRistretto(user.pub_key).decompress().expect("Invalid Ristretto Point");
    let session = LoginSession {
        y: compressed_pub_key,
        a_bytes: request.client_pub_dh,
        b_secret: secret_local,
        b_bytes : pub_local_bytes,
        nonce,
    };
    (
        ServerMessage::LoginStart(LoginStartResponse::Success {
            nonce,
            pub_dh: pub_local_bytes,
        }),
        Some(session),
    )
}

pub fn handle_login_proof(request: LoginProofRequest, y: &RistrettoPoint, a_bytes: &[u8;32], b_secret: &Scalar, b_bytes: &[u8;32], nonce: &[u8;16]) -> ServerMessage {
    let client_dh = CompressedRistretto(*a_bytes).decompress();
    let dh_key: RistrettoPoint = derive_dh_key(*b_secret, client_dh);
    let key_bytes: [u8;32] = dh_key.compress().to_bytes();
    let current_time = SystemTime::now().duration_since(UNIX_EPOCH).expect("Time extraction failed").as_secs();
    let totp = generate_totp(&key_bytes, current_time, 60, 6);
    let auth = verify_proof(&request.proof, y, a_bytes, b_bytes, totp, nonce);
    if auth {
            ServerMessage::LoginResult(LoginResult::Success)
    }
    else {
            ServerMessage::LoginResult(LoginResult::Failure { message: "Authentication failed".to_string() })
    }
}
