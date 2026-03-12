use curve25519_dalek::scalar::Scalar;
use curve25519_dalek::constants::RISTRETTO_BASEPOINT_POINT;
use curve25519_dalek::ristretto::RistrettoPoint;
use rand_core::OsRng;

pub fn keypair_gen() -> (Scalar, RistrettoPoint) {
    let mut rng = OsRng;
    let x: Scalar = Scalar::random(&mut rng);
    let y: RistrettoPoint = RISTRETTO_BASEPOINT_POINT * x;
    (x,y)
}
