use serde::{Serialize, Deserialize};
use std::fs;
use std::path::Path;

#[derive(Serialize, Deserialize)]
pub struct UserRecord {
    pub username: String,
    pub pub_key: [u8; 32],
}

fn user_exists(path: &str) -> bool {
    Path::new(&path).exists()
}

pub fn store_user_record(record: &UserRecord) {
    let dir = "server/data";
    let path = format!("{}/{}.json", dir, record.username);
    fs::create_dir_all(dir).expect("failed to create the data directory");
    if user_exists(&record.username) {
        panic!("Username already exists");
    }
    let json = serde_json::to_string_pretty(record).expect("Failed to serialize the record");
    fs::write(path, json).expect("Failed to write the user record");
}
