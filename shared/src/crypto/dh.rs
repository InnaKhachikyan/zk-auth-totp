use curve25519_dalek::scalar::Scalar;
use curve25519_dalek::constants::RISTRETTO_BASEPOINT_POINT;
use curve25519_dalek::ristretto::RistrettoPoint;
use rand_core::OsRng;

pub fn dh_gen() -> (Scalar, RistrettoPoint) {
    let mut rng = OsRng;
    let priv_local: Scalar = Scalar::random(&mut rng);
    let pub_local: RistrettoPoint = RISTRETTO_BASEPOINT_POINT * priv_local;
    (priv_local, pub_local)
}

pub fn derive_dh_key(priv_local: Scalar, pub_peer: RistrettoPoint) -> RistrettoPoint {
    priv_local * pub_peer
}
