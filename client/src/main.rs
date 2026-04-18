mod auth;
mod storage;
mod network;

use auth::{register, login};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: client <register|login>");
        return;
    }

    match args[1].as_str() {
        "register" => register(),
        "login" => login(),
        _ => eprintln!("Unknown command. Use 'register' or 'login'."),
    }
}
