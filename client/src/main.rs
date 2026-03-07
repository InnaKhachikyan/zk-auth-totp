mod crypto;
mod auth;

use shared::crypto::schnorr::keypair_gen;
use crypto::salt_gen;
use auth::read_user_password;

fn main() {
    let (x, y) = keypair_gen();

    println!("Secret x: {:?}", x.to_bytes());
    println!("Public Y: {:?}", y.compress().to_bytes());

    let salt = salt_gen();

    println!("Salt is: {:?}", salt);
    
    let password = read_user_password();
    println!("Password stored: {:?} ", password);
}
