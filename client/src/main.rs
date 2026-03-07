mod auth;

use shared::crypto::schnorr::keypair_gen;
use shared::crypto::kdf::salt_gen;
use shared::crypto::kdf::derive_key_from_password;
use auth::read_user_password;

fn main() {
    let (x, y) = keypair_gen();

    println!("Secret x: {:?}", x.to_bytes());
    println!("Public Y: {:?}", y.compress().to_bytes());

    let salt = salt_gen();

    println!("Salt is: {:?}", salt);
    
    let password = read_user_password();
    println!("Password stored: {:?} ", password);
    let key = derive_key_from_password(&password, &salt).expect("KDF failed");
    println!("Key is: {:?} ", key);
}
