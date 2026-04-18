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

//maybe will modify to return Result<> instead of crashing with panic
//might also modify to store base64 encoding instead of the bytes format

fn username_exists(path: &str) -> bool {
    Path::new(&path).exists()
}

pub fn store_local_user(record: &LocalUserRecord) -> Result<(), String> {
    let dir = "client/data";
    fs::create_dir_all(dir).map_err(|e| format!("Failed to create the data directory: {}", e))?;
    let path = format!("{}/{}.json", dir, record.username);
    if username_exists(&path) {
        return Err("Username already exists locally".to_string());
    }
    let json = serde_json::to_string_pretty(record).map_err(|e| format!("Failed to serialize the record: {}", e))?;
    fs::write(path, json).map_err(|e| format!("Failed to write the user record: {}", e))?;
    Ok(())
}

