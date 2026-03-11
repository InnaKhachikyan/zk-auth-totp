use serde::{Serialize, Deserialize};
use std::fs;

#[derive(Serialize, Deserialize)]

pub struct LocalUserRecord {
    pub username: String,
    pub salt: [u8; 16],
    pub nonce: [u8; 12],
    pub enc_x: Vec<u8>,
}

//maybe will modify to return Result<> instead of crashing with panic
//might also modify to store base64 encoding instead of the bytes format

pub fn store_local_user(record: &LocalUserRecord) {
    let dir = "client/data";
    let path = format!("{}/{}.json", dir, record.username);
    fs::create_dir_all(dir).expect("Failed to create the data directory");
    if std::path::Path::new(&path).exists() {
        panic!("Username already exists");
    }
    let json = serde_json::to_string_pretty(record).expect("Failed to serialize the record");
    fs::write(path, json).expect("Failed to write the user record");
}

