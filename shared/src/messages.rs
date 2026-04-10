use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct RegisterRequest {
    pub username: String,
    pub y: [u8; 32],
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LoginStartRequest {
    pub username: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ClientMessage {
    Register(RegisterRequest),
    LoginStart(LoginStartRequest),
}
