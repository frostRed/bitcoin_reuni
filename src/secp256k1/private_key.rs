use super::ec::utils::U256;
use super::s256_point::{S256Point, Secp256K1EllipticCurve};
use super::signature::Signature;
use crate::secp256k1::ec::utils::{u256_modmul, u256_random};
use rand::Rng;

pub struct PrivateKey {
    secret: U256,
    point: S256Point,
}

impl PrivateKey {
    pub fn new(secret: U256, point: S256Point) -> Self {
        PrivateKey {
            secret,
            point: S256Point::gen_point() * secret,
        }
    }

    pub fn sig(self, z: U256) -> Signature {
        let n = Secp256K1EllipticCurve::n();
        let mut k = u256_random();
        while k > n {
            k = u256_random();
        }

        let gen_point = S256Point::gen_point();
        let r = (gen_point * k).coordinate().unwrap().0;
        let k_inv = u256_modmul(k, n - U256::from(2u32), n);
        let mut s = u256_modmul(z + r * self.secret, k_inv, n);
        if s > n / U256::from(2u32) {
            s = n - s;
        }
        Signature::new(r, s)
    }
}
