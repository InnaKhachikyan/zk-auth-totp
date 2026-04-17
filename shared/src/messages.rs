use serde::{Serialize, Deserialize};
use crate::crypto::schnorr::{SchnorrProof};

#[derive(Serialize, Deserialize, Debug)]
pub struct RegisterRequest {
    pub username: String,
    pub y: [u8; 32],
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LoginStartRequest {
    pub username: String,
    pub client_pub_dh: [u8; 32],
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LoginProofRequest {
    pub proof: SchnorrProof, 
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ClientMessage {
    Register(RegisterRequest),
    LoginStart(LoginStartRequest),
    LoginProof(LoginProofRequest),
}

#[derive(Serialize, Deserialize, Debug)]
pub enum RegisterResponse {
    Success,
    Failure {message: String},
}

#[derive(Serialize, Deserialize, Debug)]
pub enum LoginStartResponse {
    Success {
        nonce: [u8; 16], 
        pub_dh: [u8; 32],
    },
    Failure {message: String},
}

#[derive(Serialize, Deserialize, Debug)]
pub enum LoginResult {
    Success,
    Failure {message: String},
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ServerMessage {
    Register(RegisterResponse),
    LoginStart(LoginStartResponse),
    LoginResult(LoginResult),
}
