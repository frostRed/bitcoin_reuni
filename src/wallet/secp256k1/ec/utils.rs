use num_bigint::BigUint;
use num_integer::Integer;
use num_traits::identities::One;
use rand::Rng;
use sha2::{Digest, Sha256};

construct_uint! {
    pub struct U256(4);
}

construct_uint! {
    pub struct U512(8);
}

// todo
// U256 is used by default for calculation. If there is an overflow panic, use BigUint.
impl U256 {
    pub fn is_even(&self) -> bool {
        self % U256::from(2u8) == U256::from(0u8)
    }

    pub fn to_big_uint(self) -> BigUint {
        let mut u256_bytes = [0u8; 32];
        self.to_little_endian(&mut u256_bytes);
        BigUint::from_bytes_le(&u256_bytes)
    }

    pub fn modpow(self, exp: U256, modulus: U256) -> U256 {
        let value: BigUint = self.into();
        let exp: BigUint = exp.into();
        let modulus: BigUint = modulus.into();

        value.modpow(&exp, &modulus).into()
    }

    pub fn mul(self, rhs: U256) -> U256 {
        let lhs: BigUint = self.into();
        let rhs: BigUint = rhs.into();

        (lhs * rhs).into()
    }

    pub fn modmul(self, rhs: U256, modulus: U256) -> U256 {
        let lhs: BigUint = self.into();
        let rhs: BigUint = rhs.into();
        let modulus: BigUint = modulus.into();

        (lhs * rhs % modulus).into()
    }

    pub fn from_hex(str: &[u8]) -> U256 {
        let v = BigUint::parse_bytes(str, 16u32).expect("literal number convert to BigUint failed");
        v.into()
    }

    pub fn from_random() -> U256 {
        let mut rng = rand::thread_rng();
        let n1 = rng.gen::<u128>();
        let n2 = rng.gen::<u128>();
        let ret =
            BigUint::from(n1) * pow(BigUint::from(2u32), BigUint::from(128u32)) + BigUint::from(n2);
        ret.into()
    }

    // todo
    // Abstract to Hex trait
    pub fn hex(&self) -> String {
        let string = format!("{:x}", self);
        if string.len() < 64 {
            std::iter::repeat("0")
                .take(64 - string.len())
                .collect::<String>()
                + &string
        } else {
            string
        }
    }
}

impl From<BigUint> for U256 {
    fn from(v: BigUint) -> Self {
        let big_uint_bytes = v.to_bytes_le();
        U256::from_little_endian(&big_uint_bytes)
    }
}

impl Into<BigUint> for U256 {
    fn into(self) -> BigUint {
        let mut u256_bytes = [0u8; 32];
        self.to_little_endian(&mut u256_bytes);
        BigUint::from_bytes_le(&u256_bytes)
    }
}

impl From<BigUint> for U512 {
    fn from(v: BigUint) -> Self {
        let big_uint_bytes = v.to_bytes_le();
        U512::from_little_endian(&big_uint_bytes)
    }
}

impl Into<BigUint> for U512 {
    fn into(self) -> BigUint {
        let mut u512_bytes = [0u8; 64];
        self.to_little_endian(&mut u512_bytes);
        BigUint::from_bytes_le(&u512_bytes)
    }
}

impl From<U512> for U256 {
    fn from(v: U512) -> Self {
        let mut u512_bytes = [0u8; 64];
        v.to_little_endian(&mut u512_bytes);
        U256::from_little_endian(&u512_bytes[0..32])
    }
}

impl Into<U512> for U256 {
    fn into(self) -> U512 {
        let mut u512_bytes = [0u8; 64];
        self.to_little_endian(&mut u512_bytes[0..32]);
        U512::from_little_endian(&u512_bytes)
    }
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

////////////////////////////////////
pub fn sha256_to_u256(str: &[u8]) -> U256 {
    // two rounds of sha256
    let e = Sha256::digest(&Sha256::digest(str));
    // U256 parse by big endian
    let e = e[0..32].iter().rev().map(|i| *i).collect::<Vec<u8>>();

    U256::from_little_endian(&e[0..32])
}
