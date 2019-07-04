use num_bigint::BigUint;
use num_integer::div_rem;
use num_traits::ToPrimitive;
use ripemd160::Ripemd160;
use sha2::{Digest, Sha256};
use std::ops::Deref;

pub fn encode_base58(bytes: &[u8]) -> String {
    let base58_alphabet = "123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz";

    let mut prefix = "".to_string();
    for i in bytes.iter() {
        if *i == 0u8 {
            prefix = prefix + "1";
        } else {
            break;
        }
    }

    let mut v = BigUint::from_bytes_be(bytes);
    let mut ret = "".to_string();
    while v > BigUint::from(0u8) {
        let (quotient, remainder) = div_rem(v, BigUint::from(58u8));
        v = quotient;
        ret.push(
            base58_alphabet
                .chars()
                .nth(remainder.to_usize().unwrap())
                .unwrap(),
        )
    }

    ret = ret + &prefix;
    ret.chars().rev().collect()
}

pub fn encode_base58_checksum(bytes: &[u8]) -> String {
    let hash = hash256(bytes);
    let mut bytes = bytes.to_vec();
    bytes.extend_from_slice(&hash[0..4]);
    encode_base58(&bytes)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Hash256([u8; 32]);

impl Copy for Hash256 {}

impl Hash256 {
    pub fn to_vec(&self) -> Vec<u8> {
        self.0.to_vec()
    }
}

impl Deref for Hash256 {
    type Target = [u8];
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Hash160([u8; 20]);
impl Copy for Hash160 {}

impl Hash160 {
    pub fn to_vec(&self) -> Vec<u8> {
        self.0.to_vec()
    }
}

impl Deref for Hash160 {
    type Target = [u8];
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub fn hash160(bytes: &[u8]) -> Hash160 {
    let hash = Ripemd160::digest(&Sha256::digest(bytes));
    let mut buf: [u8; 20] = Default::default();
    buf.copy_from_slice(&hash[0..20]);
    Hash160(buf)
}

pub fn hash256(bytes: &[u8]) -> Hash256 {
    // tow rounds of sha256
    let hash = Sha256::digest(&Sha256::digest(bytes));
    let mut buf: [u8; 32] = Default::default();
    buf.copy_from_slice(&hash[0..32]);
    Hash256(buf)
}

mod test {
    use super::{encode_base58, encode_base58_checksum, hash160, hash256, Hash160, Hash256};

    #[test]
    fn test_hash160() {
        let v = b"1";
        assert_eq!(
            Hash160([
                67, 30, 206, 201, 78, 10, 146, 10, 121, 114, 176, 132, 220, 250, 187, 214, 159, 97,
                105, 18
            ]),
            hash160(v)
        );
    }

    #[test]
    fn test_encode_base58() {
        let v = hash256(b"1");
        assert_eq!(
            encode_base58(&v),
            "BWfYz3GXAHhqpwCKmzEviyajcVR9ou1XT2HS1fDxvyuZ".to_string()
        );
    }

    #[test]
    fn test_hash256() {
        let v = b"1";
        assert_eq!(
            Hash256([
                156, 46, 77, 143, 233, 125, 136, 20, 48, 222, 78, 117, 75, 66, 5, 185, 194, 124,
                233, 103, 21, 35, 28, 255, 196, 51, 115, 64, 203, 17, 2, 128
            ]),
            hash256(v)
        );
    }

    #[test]
    fn test_encode_base58_checksum() {
        let v = hash256(b"1");
        assert_eq!(
            encode_base58_checksum(&v),
            "2BnRyzAHqgBgec9ahUkMZ1uchLFa5Dha2BLTuzCS1orPri4j2f".to_string()
        );
    }
}
