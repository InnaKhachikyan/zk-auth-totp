mod crypto;

use shared::crypto::schnorr::keypair_gen;
use crypto::salt_gen;

fn main() {
    let (x, y) = keypair_gen();

    println!("Secret x: {:?}", x.to_bytes());
    println!("Public Y: {:?}", y.compress().to_bytes());

    let salt = salt_gen();

    println!("Salt is: {:?}", salt);
}
