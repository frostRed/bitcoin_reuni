use super::ec::utils::U256;
use super::s256_point::{S256Point, Secp256K1EllipticCurve};
use super::signature::Signature;
use super::utils::encode_base58_checksum;
use num_bigint::BigUint;
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
        let mut k = U256::from_random();
        while k > n {
            k = U256::from_random();
        }

        let gen_point = S256Point::gen_point();
        let r = (gen_point * k).coordinate().unwrap().0;
        let k_inv = k.modpow(n - U256::from(2u32), n);

        //        let mut s = u256_modmul(z + r * self.secret, k_inv, n);
        let mut s = (Into::<BigUint>::into(z)
            + Into::<BigUint>::into(r) * Into::<BigUint>::into(self.secret))
            * Into::<BigUint>::into(k_inv);
        s = s % Into::<BigUint>::into(n);
        let mut s: U256 = s.into();
        // It turns out that using the low-s value will get nodes to relay our transactions.
        // This is for malleability reasons.
        if s > n / U256::from(2u32) {
            s = n - s;
        }

        Signature::new(r, s)
    }

    pub fn wif(&self, compressed: bool, testnet: bool) -> String {
        let mut secret_bytes = [0u8; 32];
        self.secret.to_big_endian(&mut secret_bytes);

        let prefix = if testnet {
            vec![b'\xef']
        } else {
            vec![b'\x80']
        };

        let suffix = if compressed { vec![b'\x01'] } else { vec![] };

        let all_bytes = [&prefix[..], &secret_bytes[..], &suffix[..]].concat();
        encode_base58_checksum(&all_bytes)
    }
}
