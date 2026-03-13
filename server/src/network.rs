use std::io::{BufRead, BufReader};
use std::net::{TcpListener, TcpStream};
use shared::messages::RegisterRequest;

pub fn run_server() {
    let listener = TcpListener::bind("127.0.0.1:7878").expect("Failed to bind server");
    println!("Server listening on localhost");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => handle_client(stream),Err(e) => eprintln!("Connection failed: {}", e),
        }
    }
}

fn handle_client(stream: TcpStream) {
    let mut reader = BufReader::new(stream);
    let mut message = String::new();
    reader.read_line(&mut message).expect("Failed to read from client");
    let request: RegisterRequest = serde_json::from_str(&message).expect("Failed to deserialize RegisterRequest");

    println!("Received registration request:");
    println!("username = {}", request.username);
    println!("y = {:?}", request.y);
}
