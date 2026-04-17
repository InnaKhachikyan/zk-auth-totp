use curve25519_dalek::scalar::Scalar;
use curve25519_dalek::constants::RISTRETTO_BASEPOINT_POINT;
use curve25519_dalek::ristretto::{CompressedRistretto, RistrettoPoint};
use rand_core::OsRng;
use sha2::{Digest, Sha512};
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SchnorrProof {
    pub t: [u8;32],
    pub s: [u8;32],
}

//e = SHA-256("ZK_TOTP_AUTH" || t || Y || A || B || TOTP || nonce)
fn compute_fiat_shamir_challenge(t: &RistrettoPoint, y: &RistrettoPoint, a_bytes: &[u8;32], b_bytes: &[u8;32], totp: u32, nonce: &[u8;16]) -> Scalar {
    let mut hasher = Sha512::new();
    hasher.update(b"ZK_AUTH_TOTP");
    hasher.update(t.compress().as_bytes());
    hasher.update(y.compress().as_bytes());
    hasher.update(a_bytes);
    hasher.update(b_bytes);
    hasher.update(totp.to_be_bytes());
    hasher.update(nonce);
    let hash = hasher.finalize();

    let mut scalar = [0u8;64];
    scalar.copy_from_slice(&hash);
    Scalar::from_bytes_mod_order_wide(&scalar)
}

pub fn generate_proof(x: &Scalar, y: &RistrettoPoint, a_bytes: &[u8;32], b_bytes: &[u8;32], totp: u32, server_nonce: &[u8;16]) -> SchnorrProof {
    let mut rng = OsRng;
    let r = Scalar::random(&mut rng);
    let t_point = RISTRETTO_BASEPOINT_POINT * r;
    let e = compute_fiat_shamir_challenge(&t_point, y, a_bytes, b_bytes, totp, server_nonce);
    let s_point = r + e * x;

    SchnorrProof {
        t: t_point.compress().to_bytes(),
        s: s_point.to_bytes(),
    }
}

pub fn verify_proof(proof: &SchnorrProof, y: &RistrettoPoint, a_bytes: &[u8;32], b_bytes: &[u8;32], totp: u32, nonce: &[u8;16]) -> bool {
    let t_point = match CompressedRistretto(proof.t).decompress() {
        Some(point) => point,
        None => return false,
    };

    let s_point = Scalar::from_bytes_mod_order(proof.s);
    let e = compute_fiat_shamir_challenge(&t_point, y, a_bytes, b_bytes, totp, nonce);
    let left_side = RISTRETTO_BASEPOINT_POINT * s_point;
    let right_side = t_point + (y * e);
    left_side == right_side
}

pub fn keypair_gen() -> (Scalar, RistrettoPoint) {
    let mut rng = OsRng;
    let x: Scalar = Scalar::random(&mut rng);
    let y: RistrettoPoint = RISTRETTO_BASEPOINT_POINT * x;
    (x,y)
}
