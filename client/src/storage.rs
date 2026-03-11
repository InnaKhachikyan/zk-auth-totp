use serde::{Serialize, Deserialize};
use std::fs;
use std::path::Path;

#[derive(Serialize, Deserialize)]

pub struct LocalUserRecord {
    pub username: String,
    pub salt: [u8; 16],
    pub nonce: [u8; 12],
    pub enc_x: Vec<u8>,
}

pub fn store_local_user(record: &LocalUserRecord) {
    let dir = "../data";
    let path = format!("{}/{}.json", dir, record.username);
    fs::create_dir_all(dir).expect("Failed to create the data directory");
    let json = serde_json::to_string_pretty(record).expect("Failed to serialize the record");
    fs::write(path, json).expect("Failed to write the user record");
}

