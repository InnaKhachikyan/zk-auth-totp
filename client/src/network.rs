use std::io::Write;
use std::net::TcpStream;
use shared::messages::RegisterRequest;

pub fn send_register_request(username: String, y: [u8; 32]) {
    let request = RegisterRequest {username, y};
    let json = serde_json::to_string(&request).expect("Failed to serialize RegisterRequest");
    let mut stream = TcpStream::connect("127.0.0.1:7878").expect("Failed to connect to the server");
    stream.write_all(json.as_bytes()).expect("Failed to send the request");
    stream.write_all(b"\n").expect("Failed to send the newline");
    println!("Registration request sent");
}
