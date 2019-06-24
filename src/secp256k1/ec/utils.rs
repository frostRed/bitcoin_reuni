use num_bigint::BigUint;
use num_integer::{div_rem, Integer};
use num_traits::identities::One;
use num_traits::ToPrimitive;
use rand::Rng;
use ripemd160::Ripemd160;
use sha2::{Digest, Sha256};

construct_uint! {
    pub struct U256(4);
}

construct_uint! {
    pub struct U512(8);
}

pub fn u256_is_even(v: U256) -> bool {
    v % U256::from(2u8) == U256::from(0u8)
}

pub fn u256_to_big_uint(v: U256) -> BigUint {
    let mut u256_bytes = [0u8; 32];
    v.to_little_endian(&mut u256_bytes);
    BigUint::from_bytes_le(&u256_bytes)
}

pub fn big_uint_to_u256(v: &BigUint) -> U256 {
    let big_uint_bytes = v.to_bytes_le();
    U256::from_little_endian(&big_uint_bytes)
}

/////////////
pub fn big_uint_to_u512(v: &BigUint) -> U512 {
    let big_uint_bytes = v.to_bytes_le();
    U512::from_little_endian(&big_uint_bytes)
}

pub fn u512_to_big_uint(v: U512) -> BigUint {
    let mut u512_bytes = [0u8; 64];
    v.to_little_endian(&mut u512_bytes);
    BigUint::from_bytes_le(&u512_bytes)
}

//////////////
pub fn u512_to_u256(v: U512) -> U256 {
    let mut u512_bytes = [0u8; 64];
    v.to_little_endian(&mut u512_bytes);
    U256::from_little_endian(&u512_bytes[0..32])
}

pub fn u256_to_u512(v: U256) -> U512 {
    let mut u512_bytes = [0u8; 64];
    v.to_little_endian(&mut u512_bytes[0..32]);
    U512::from_little_endian(&u512_bytes)
}

///////////
pub fn pow(value: BigUint, exp: BigUint) -> BigUint {
    if exp.is_one() {
        return value;
    }
    if exp.is_odd() {
        return value.clone() * pow(value.clone() * value.clone(), exp / BigUint::from(2u32));
    }
    return pow(value.clone() * value.clone(), exp / BigUint::from(2u32));
}

pub fn u256_modpow(value: U256, exp: U256, modulus: U256) -> U256 {
    let value = u256_to_big_uint(value);
    let exp = u256_to_big_uint(exp);
    let modulus = u256_to_big_uint(modulus);

    big_uint_to_u256(&value.modpow(&exp, &modulus))
}

pub fn u256_mul(lhs: U256, rhs: U256) -> U256 {
    let lhs = u256_to_big_uint(lhs);
    let rhs = u256_to_big_uint(rhs);

    big_uint_to_u256(&(lhs * rhs))
}

pub fn u256_modmul(lhs: U256, rhs: U256, modulus: U256) -> U256 {
    let lhs = u256_to_big_uint(lhs);
    let rhs = u256_to_big_uint(rhs);
    let modulus = u256_to_big_uint(modulus);

    big_uint_to_u256(&(lhs * rhs % modulus))
}

pub fn u256_parse_str(str: &[u8], radix: u32) -> U256 {
    let v = BigUint::parse_bytes(str, radix).expect("literal number convert to BigUint failed");
    big_uint_to_u256(&v)
}

pub fn u256_random() -> U256 {
    let mut rng = rand::thread_rng();
    let n1 = rng.gen::<u128>();
    let n2 = rng.gen::<u128>();
    let ret =
        BigUint::from(n1) * pow(BigUint::from(2u32), BigUint::from(128u32)) + BigUint::from(n2);
    big_uint_to_u256(&ret)
}

////////////////////////////////////
pub fn hash256(bytes: &[u8]) -> Vec<u8> {
    let hash = Sha256::digest(bytes);
    hash[0..32].iter().map(|i| *i).collect()
}

pub fn hash160(bytes: &[u8]) -> Vec<u8> {
    let hash = Ripemd160::digest(&Sha256::digest(bytes));
    hash[0..20].iter().map(|i| *i).collect()
}

pub fn sha256_to_u256(str: &[u8]) -> U256 {
    /// tow rounds of sha256
    let e = Sha256::digest(&Sha256::digest(str));
    /// U256 parse by big endian
    let e = e[0..32].iter().rev().map(|i| *i).collect::<Vec<u8>>();

    U256::from_little_endian(&e[0..32])
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
    use super::{encode_base58, U256};

    #[test]
    fn test_encode_base58() {
        let v = [1u8];
        assert_eq!(encode_base58(&v), "2".to_string());
    }
}
