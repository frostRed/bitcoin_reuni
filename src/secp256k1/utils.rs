use num_bigint::BigUint;
use num_integer::{div_rem, Integer};
use num_traits::identities::One;
use num_traits::ToPrimitive;
use ripemd160::Ripemd160;
use sha2::{Digest, Sha256};

pub fn hash256(bytes: &[u8]) -> Vec<u8> {
    let hash = Sha256::digest(bytes);
    hash[0..32].iter().map(|i| *i).collect()
}

pub fn hash160(bytes: &[u8]) -> Vec<u8> {
    let hash = Ripemd160::digest(&Sha256::digest(bytes));
    hash[0..20].iter().map(|i| *i).collect()
}

pub fn encode_base58(bytes: &[u8]) -> String {
    let base58_alphabet = "123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz";

    let mut prefix = "".to_string();
    for i in bytes.iter() {
        if *i == 0u8 {
            prefix = prefix + "1"
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

mod test {
    use super::encode_base58;

    #[test]
    fn test_encode_base58() {
        let v = [1u8];
        assert_eq!(encode_base58(&v), "2".to_string());
    }
}
