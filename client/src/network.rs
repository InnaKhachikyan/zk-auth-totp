use std::io::Write;
use std::net::TcpStream;
use std::io::{BufRead, BufReader};
use shared::messages::{RegisterRequest, LoginStartRequest, ClientMessage, LoginStartResponse};

pub fn start_connection() -> TcpStream {
    let stream = TcpStream::connect("127.0.0.1:7878").expect("Failed to connect to the server");
    stream
}

pub fn send_register_request(username: String, y: [u8; 32], stream: &mut TcpStream) {
    let request = ClientMessage::Register(RegisterRequest {username, y,});
    let json = serde_json::to_string(&request).expect("Failed to serialize RegisterRequest");
    stream.write_all(json.as_bytes()).expect("Failed to send the request");
    stream.write_all(b"\n").expect("Failed to send the newline");
    println!("Registration request sent");
}

pub fn send_login_request(username: String, A: [u8; 32], stream: &mut TcpStream) {
    let request = ClientMessage::LoginStart(LoginStartRequest {username, A,});
    let json = serde_json::to_string(&request).expect("Failed to serialize the LoginStartRequest");
    stream.write_all(json.as_bytes()).expect("failed to send the request");
    stream.write_all(b"\n").expect("Failed to send the newline");
    println!("Login request sent");
}

pub fn read_login_start_response(stream: &mut TcpStream) -> LoginStartResponse {
    let mut reader = BufReader::new(stream);
    let mut response = String::new();
    reader.read_line(&mut response).expect("Failed to read response");
    let reply: LoginStartResponse = serde_json::from_str(&response).expect("Failed to deserialize the response");
    reply
}
