use std::io::{BufRead, BufReader};
use std::thread;
use std::net::{TcpListener, TcpStream};
use shared::messages::{ClientMessage, ServerMessage};
use crate::auth::{handle_register, handle_login_start, handle_login_proof};

pub fn run_server() {
    let listener = TcpListener::bind("127.0.0.1:7878").expect("Failed to bind server");
    println!("Server listening on localhost");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("Client connected");
                thread::spawn(move || {
                    handle_client(stream);
                });
            }
            Err(e) => eprintln!("Connection failed: {}", e),
        }
    }
}

fn handle_client(mut stream: TcpStream) {
    let reader_stream = match stream.try_clone() {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Failed to clone stream: {}", e);
            return;
        }
    };

    let mut reader = BufReader::new(reader_stream);
    loop {
        let mut message = String::new();

        match reader.read_line(&mut message) {
            Ok(0) => {
                println!("Clinet disconnected");
                break;
            }
            Ok(_) => {}
            Err(e) => {
                eprintln!("Failed to read from client: {}", e);
                break;
            }
        }
        let request: ClientMessage = match serde_json::from_str(&message) {
            Ok(req) => req,
            Err(e) => {
                eprintln!("Failed to deserialize client message: {}", e);
                continue;
            }
        };
        let response = match request {
            ClientMessage::Register(request) => handle_register(request),
            ClientMessage::LoginStart(request) => handle_login_start(request),
            ClientMessage::LoginProof(request) => handle_login_proof(request),
        };
        if let Err(e) = send_response(&mut stream, &response) {
            eprintln!("Failed to send response {}", e);
            break;
        }
    }
}

fn send_response(stream: &mut TcpStream, response: &ServerMessage) -> Result<(), Box<dyn std::error::Error>> {
    return Ok(())
}


