use std::io::{BufRead, BufReader};
use std::net::{TcpListener, TcpStream};
use shared::messages::{RegisterRequest, LoginStartRequest, ClientMessage};
use crate::storage::{UserRecord, store_user_record};

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
    let request: ClientMessage = serde_json::from_str(&message).expect("Failed to deserialize client message");
    match request {
        ClientMessage::Register(request) => handle_register(request),
        ClientMessage::LoginStart(request) => handle_login_start(request),
    }
}

fn handle_register(request: RegisterRequest) {
    println!("Received register request");
    let path = format!("server/data/{}.json", request.username);
    if std::path::Path::new(&path).exists() {
        panic!("Username already exists");
    }
    let record = UserRecord {username: request.username, pub_key: request.y};
    store_user_record(&record);
}

fn handle_login_start(request: LoginStartRequest) {
    println!("Received login request");
    let path = format!("server/data/{}.json", request.username);
    if !std::path::Path::new(&path).exists() {
        panic!("User does not exist");
    }
}
