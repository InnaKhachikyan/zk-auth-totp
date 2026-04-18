use serde::{Serialize, Deserialize};
use std::fs;
use std::path::Path;

#[derive(Serialize, Deserialize, Debug)]
pub struct UserRecord {
    pub username: String,
    pub pub_key: [u8; 32],
}

pub fn user_exists(path: &str) -> bool {
    Path::new(&path).exists()
}

pub fn store_user_record(record: &UserRecord) -> Result<(), String> {
    let dir = "server/data";
    let path = format!("{}/{}.json", dir, record.username);
    fs::create_dir_all(dir).map_err(|e| format!("Failed to create data directory: {}", e))?;
    if user_exists(&path) {
        return Err("Username already exists".to_string());
    }
    let json = serde_json::to_string_pretty(record).map_err(|e| format!("Failed to serialize record: {}", e))?;
    fs::write(path, json).map_err(|e| format!("Failed to write user record: {}", e))?;
    Ok(())
}

pub fn load_user(username: &str) -> Result<UserRecord, Box<dyn std::error::Error>> {
    let path = format!("server/data/{}.json", username);
    let data = std::fs::read_to_string(&path)?;
    let user = serde_json::from_str(&data)?;
    Ok(user)
}
