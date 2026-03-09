mod auth;

use shared::crypto::schnorr::keypair_gen;
use shared::crypto::kdf::salt_gen;
use shared::crypto::kdf::derive_key_from_password;
use auth::read_user_password;
use auth::read_username;

fn main() {

    let username = read_username();
    println!("Username stored: {} ", username);

    let password = read_user_password();
    println!("Password stored: {:?} ", password);
    
    let salt = salt_gen();
    println!("Salt is: {:?}", salt);
    
    let key = derive_key_from_password(&password, &salt).expect("KDF failed");
    println!("Key is: {:?} ", key);
    
    let (x, y) = keypair_gen();

    println!("Secret x: {:?}", x.to_bytes());
    println!("Public Y: {:?}", y.compress().to_bytes());

}
