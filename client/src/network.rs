use std::io::Write;
use std::net::TcpStream;
use std::io::{BufRead, BufReader};
use shared::crypto::schnorr::{SchnorrProof};
use shared::messages::{RegisterRequest, LoginStartRequest, LoginProofRequest, ClientMessage, LoginStartResponse, RegisterResponse, LoginResult, ServerMessage};

pub struct ClientConnection {
    writer: TcpStream,
    reader: BufReader<TcpStream>,
}

pub fn start_connection() -> ClientConnection {
    let writer = TcpStream::connect("127.0.0.1:7878").expect("Failed to connect to the server");
    let reader_stream = writer.try_clone().expect("Failed to clone the stream");

    let reader = BufReader::new(reader_stream);
    ClientConnection {writer,reader }
}

pub fn send_register_request(c_con: &mut ClientConnection, username: String, y: [u8; 32]) {
    let request = ClientMessage::Register(RegisterRequest {username, y,});
    let json = serde_json::to_string(&request).expect("Failed to serialize RegisterRequest");
    c_con.writer.write_all(json.as_bytes()).expect("Failed to send the request");
    c_con.writer.write_all(b"\n").expect("Failed to send the newline");
    println!("Registration request sent");
}

pub fn send_login_request(c_con: &mut ClientConnection, username: String, client_pub_dh: [u8; 32]) {
    let request = ClientMessage::LoginStart(LoginStartRequest {username, client_pub_dh,});
    let json = serde_json::to_string(&request).expect("Failed to serialize the LoginStartRequest");
    c_con.writer.write_all(json.as_bytes()).expect("failed to send the request");
    c_con.writer.write_all(b"\n").expect("Failed to send the newline");
    println!("Login request sent");
}

pub fn send_login_proof(c_con: &mut ClientConnection, proof: SchnorrProof) {
    let request = ClientMessage::LoginProof(LoginProofRequest {proof});
    let json = serde_json::to_string(&request).expect("Failed to serialize the LoginProofRequest");
    c_con.writer.write_all(json.as_bytes()).expect("Failed to send the request");
    c_con.writer.write_all(b"\n").expect("Failed to send the newline");
    println!("Login proof sent");
}

pub fn read_register_response(c_con: &mut ClientConnection) -> RegisterResponse {
    let mut response = String::new();
    c_con.reader.read_line(&mut response).expect("Failed to read response");
    let reply: ServerMessage = serde_json::from_str(&response).expect("Failed to deserialize the response");
    match reply {
        ServerMessage::Register(register_response) => register_response,
        _ => panic!("Expected Register response from server"),
    }
}

pub fn read_login_start_response(c_con: &mut ClientConnection) -> LoginStartResponse {
    let mut response = String::new();
    c_con.reader.read_line(&mut response).expect("Failed to read response");
    let reply: ServerMessage = serde_json::from_str(&response).expect("Failed to deserialize the response");
    match reply {
        ServerMessage::LoginStart(login_start_response) => login_start_response,
        _ => panic!("Expected LoginStart response from server"),
    }
}

pub fn read_login_result_response(c_con: &mut ClientConnection) -> LoginResult {
    let mut response = String::new();
    c_con.reader.read_line(&mut response).expect("Failed to read response");
    let reply: ServerMessage = serde_json::from_str(&response).expect("Failed to deserialize the response");
    match reply {
        ServerMessage::LoginResult(login_result) => login_result,
        _ => panic!("Expected LoginResult response from server"),
    }
}
