use super::ec::utils::U256;
use super::s256_point::{S256Point, Secp256K1EllipticCurve};
use super::signature::Signature;
use super::utils::encode_base58_checksum;
use crate::secp256k1::utils::hmac_sha256_digest;
use bytes::{BufMut, BytesMut};
use num_bigint::BigUint;
use rand::Rng;
use sha2::Sha256;

pub struct PrivateKey {
    secret: U256,
    pub point: S256Point,
}

impl PrivateKey {
    pub fn new(secret: U256) -> Self {
        PrivateKey {
            secret,
            point: S256Point::gen_point() * secret,
        }
    }

    pub fn hex(&self) -> String {
        self.secret.hex()
    }

    pub fn sign(&self, z: U256) -> Signature {
        let n = Secp256K1EllipticCurve::n();
        let mut k = self.deterministic_k(z);
        while k > n {
            k = U256::from_random();
        }

        let gen_point = S256Point::gen_point();
        let r = (gen_point * k).coordinate().unwrap().0;
        let k_inv = k.modpow(n - U256::from(2u32), n);

        // let mut s = u256_modmul(z + r * self.secret, k_inv, n);
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

    /// RFC 6979 use *secret* and *z* to create a unique, deterministic **K** every time
    fn deterministic_k(&self, z: U256) -> U256 {
        let n: U256 = Secp256K1EllipticCurve::n();

        let mut k = vec![b'\x00'; 32];
        let mut v = vec![b'\x01'; 32];
        let mut z = z;
        if z > n {
            z = z - n;
        }

        let mut z_bytes = [0u8; 32];
        z.to_big_endian(&mut z_bytes);
        let mut secret_bytes = [0u8; 32];
        self.secret.to_big_endian(&mut secret_bytes);

        let mut buf = BytesMut::with_capacity(256);

        buf.put(&v[..]);
        buf.put(&b"\x00"[..]);
        buf.put(&secret_bytes[..]);
        buf.put(&z_bytes[..]);
        let data = buf.take();
        k = hmac_sha256_digest(&k, &data[..]);
        v = hmac_sha256_digest(&k, &v);

        buf.put(&v[..]);
        buf.put(&b"\x01"[..]);
        buf.put(&secret_bytes[..]);
        buf.put(&z_bytes[..]);
        let data = buf.take();
        k = hmac_sha256_digest(&k, &data[..]);
        v = hmac_sha256_digest(&k, &v);

        loop {
            v = hmac_sha256_digest(&k, &v);
            let candidate = U256::from_big_endian(&v);
            if candidate >= U256::from(1u8) && candidate < n {
                return candidate;
            }

            buf.put(&v[..]);
            buf.put(&b"\x00"[..]);
            let data = buf.take();
            k = hmac_sha256_digest(&k, &data[..]);
            v = hmac_sha256_digest(&k, &v);
        }
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

mod test {
    use crate::secp256k1::ec::utils::{pow, sha256_to_u256, U256};
    use crate::secp256k1::private_key::PrivateKey;
    use crate::secp256k1::s256_point::S256Point;
    use num_bigint::BigUint;

    #[test]
    fn test_wif() {
        let point = S256Point::gen_point();

        let secret: BigUint = pow(BigUint::from(2u8), BigUint::from(256u16))
            - pow(BigUint::from(2u8), BigUint::from(199u8));
        let secret: U256 = secret.into();
        let p = PrivateKey::new(secret);
        assert_eq!(
            "L5oLkpV3aqBJ4BgssVAsax1iRa77G5CVYnv9adQ6Z87te7TyUdSC".to_string(),
            p.wif(true, false)
        );

        let secret: BigUint = pow(BigUint::from(2u8), BigUint::from(256u16))
            - pow(BigUint::from(2u8), BigUint::from(201u8));
        let secret: U256 = secret.into();
        let p = PrivateKey::new(secret);
        assert_eq!(
            "93XfLeifX7Jx7n7ELGMAf1SUR6f9kgQs8Xke8WStMwUtrDucMzn".to_string(),
            p.wif(false, true)
        );

        let p = PrivateKey::new(U256::from_hex(
            b"0dba685b4511dbd3d368e5c4358a1277de9486447af7b3604a69b8d9d8b7889d",
        ));
        assert_eq!(
            "5HvLFPDVgFZRK9cd4C5jcWki5Skz6fmKqi1GQJf5ZoMofid2Dty".to_string(),
            p.wif(false, false)
        );

        let p = PrivateKey::new(U256::from_hex(
            b"1cca23de92fd1862fb5b76e5f4f50eb082165e5191e116c18ed1a6b24be6a53f",
        ));
        assert_eq!(
            "cNYfWuhDpbNM1JWc3c6JTrtrFVxU4AGhUKgw5f93NP2QaBqmxKkg".to_string(),
            p.wif(true, true)
        );
    }

    #[test]
    fn test_address() {
        let point = S256Point::gen_point();

        let secret: BigUint = pow(BigUint::from(888u16), BigUint::from(3u8));
        let secret: U256 = secret.into();
        let p = PrivateKey::new(secret);

        assert_eq!(
            "148dY81A9BmdpMhvYEVznrM45kWN32vSCN".to_string(),
            p.point.address(true, false)
        );
        assert_eq!(
            "mieaqB68xDCtbUBYFoUNcmZNwk74xcBfTP".to_string(),
            p.point.address(true, true)
        );
    }

    #[test]
    fn test_sig() {
        //        let pk = PrivateKey::new(U256::from_random());
        //        let z = U256::from_random();
        let pk = PrivateKey::new(U256::from(333u16));
        let z = U256::from(999u16);
        let sig = pk.sign(z);
        assert_eq!(pk.point.verify(z, sig), true);
    }
}
