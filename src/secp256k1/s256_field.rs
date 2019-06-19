use num_bigint::{BigInt, BigUint, Sign};
use num_traits::{one, zero};
use std::fmt::{self, Display};
use std::ops::{Add, Div, Mul, Sub};

use super::ec::field_element::FieldElementError;
use super::ec::utils::{big_uint_to_u256, u256_to_big_uint, u512_to_u256, U256, U512};

/// Secp256k1 Finite field element
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct S256Field {
    /// Secp256k1 Finite field element number value
    pub num: U256,
    /// Secp256k1 Finite field prime, finite field F = {0 , 1, 2, ..., p-1}
    pub prime: U256,
}

impl Copy for S256Field {}

impl S256Field {
    pub fn new<T: Into<U256>>(num: T) -> Self {
        S256Field {
            num: num.into(),
            prime: Self::prime(),
        }
    }

    pub fn pow(self, exp: i32) -> Self {
        let num = u256_to_big_uint(self.num);
        let prime = u256_to_big_uint(self.prime);

        let mut exp = BigInt::from(exp);
        while exp < zero() {
            exp = exp + BigInt::from_biguint(Sign::Plus, prime.clone() - BigUint::from(1u32));
        }
        let mut e = exp.to_biguint().expect("BigInt convert to BigUint failed");
        // fast very big exp calculate
        e = e % (prime.clone() - BigUint::from(1u32));
        let num = num.modpow(&e, &prime);

        S256Field::new(big_uint_to_u256(&num))
    }

    pub fn prime() -> U256 {
        let p = U512::from(2u32).pow(U512::from(256u32))
            - U512::from(2u32).pow(U512::from(32u32))
            - U512::from(977u32);
        u512_to_u256(p)
    }

    pub fn sqrt(&self) -> Self {
        let prime = u256_to_big_uint(self.prime);
        let power = (prime.clone() + BigUint::from(1u8)) / BigUint::from(4u8);
        let new_num = u256_to_big_uint(self.num).modpow(&power, &prime);
        S256Field {
            num: big_uint_to_u256(&new_num),
            prime: self.prime,
        }
    }
}

impl<T> From<T> for S256Field
where
    T: Into<U256>,
{
    fn from(num: T) -> Self {
        S256Field::new(num.into())
    }
}

impl Add<Self> for S256Field {
    type Output = S256Field;

    fn add(self, rhs: Self) -> Self::Output {
        if self.prime != rhs.prime {
            panic!("{}", FieldElementError::NotSamePrime);
        }

        let num = u256_to_big_uint(self.num);
        let rhs_num = u256_to_big_uint(rhs.num);
        let prime = u256_to_big_uint(self.prime);
        let num = (num + rhs_num) % prime;

        S256Field::new(big_uint_to_u256(&num))
    }
}

impl<T> Add<T> for S256Field
where
    T: Into<U256>,
{
    type Output = S256Field;

    fn add(self, rhs: T) -> Self::Output {
        let num = u256_to_big_uint(self.num);
        let rhs_num = u256_to_big_uint(rhs.into());
        let prime = u256_to_big_uint(self.prime);
        let num = (num + rhs_num) % prime;

        S256Field::new(big_uint_to_u256(&num))
    }
}

impl Add<S256Field> for U256 {
    type Output = S256Field;

    fn add(self, rhs: S256Field) -> Self::Output {
        let num = u256_to_big_uint(self);
        let rhs_num = u256_to_big_uint(rhs.num);
        let prime = u256_to_big_uint(rhs.prime);
        let num = (num + rhs_num) % prime;

        S256Field::new(big_uint_to_u256(&num))
    }
}

impl Sub<Self> for S256Field {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        if self.prime != rhs.prime {
            panic!("{}", FieldElementError::NotSamePrime);
        }

        let self_num = u256_to_big_uint(self.num);
        let self_prime = u256_to_big_uint(self.prime);
        let rhs_num = u256_to_big_uint(rhs.num);

        let mut num: BigInt = zero();
        if self.num >= rhs.num {
            num = BigInt::from_biguint(Sign::Plus, (self_num - rhs_num) % self_prime.clone());
        } else {
            num = BigInt::from_biguint(Sign::Minus, (rhs_num - self_num) % self_prime.clone());
        }
        while num < zero() {
            num = num + BigInt::from_biguint(Sign::Plus, self_prime.clone());
        }
        S256Field::new(big_uint_to_u256(
            &num.to_biguint().expect("BigInt convert to BigUint failed"),
        ))
    }
}

impl<T> Sub<T> for S256Field
where
    T: Into<U256>,
{
    type Output = Self;

    fn sub(self, rhs: T) -> Self::Output {
        let self_num = u256_to_big_uint(self.num);
        let rhs_num = u256_to_big_uint(rhs.into());
        let self_prime = u256_to_big_uint(self.prime);

        let mut num: BigInt = zero();
        if self_num >= rhs_num {
            num = BigInt::from_biguint(Sign::Plus, (self_num - rhs_num) % self_prime.clone());
        } else {
            num = BigInt::from_biguint(Sign::Minus, (rhs_num - self_num) % self_prime.clone());
        }
        while num < zero() {
            num = num + BigInt::from_biguint(Sign::Plus, self_prime.clone());
        }

        S256Field::new(big_uint_to_u256(
            &num.to_biguint().expect("BigInt convert to BigUint failed"),
        ))
    }
}

impl Mul<Self> for S256Field {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        if self.prime != rhs.prime {
            panic!("{}", FieldElementError::NotSamePrime);
        }

        let self_num = u256_to_big_uint(self.num);
        let rhs_num = u256_to_big_uint(rhs.num);
        let self_prime = u256_to_big_uint(self.prime);
        let num = (self_num * rhs_num) % self_prime;

        S256Field::new(big_uint_to_u256(&num))
    }
}

impl<T> Mul<T> for S256Field
where
    T: Into<U256>,
{
    type Output = S256Field;
    fn mul(self, rhs: T) -> Self::Output {
        let self_num = u256_to_big_uint(self.num);
        let rhs_num = u256_to_big_uint(rhs.into());
        let self_prime = u256_to_big_uint(self.prime);
        let num = (self_num * rhs_num) % self_prime;

        S256Field::new(big_uint_to_u256(&num))
    }
}

impl Mul<S256Field> for U256 {
    type Output = S256Field;
    fn mul(self, rhs: S256Field) -> Self::Output {
        let self_num = u256_to_big_uint(self);
        let rhs_num = u256_to_big_uint(rhs.num);
        let prime = u256_to_big_uint(rhs.prime);
        let num = (self_num * rhs_num) % prime;

        S256Field::new(big_uint_to_u256(&num))
    }
}

impl Div<Self> for S256Field {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        let t = u256_to_big_uint(self.prime - 2);
        let num = (u256_to_big_uint(self.num)
            * u256_to_big_uint(rhs.num).modpow(&t, &u256_to_big_uint(self.prime)))
            % u256_to_big_uint(self.prime);

        S256Field::new(big_uint_to_u256(&num))
    }
}

impl Div<U256> for S256Field {
    type Output = Self;

    fn div(self, rhs: U256) -> Self::Output {
        let t = u256_to_big_uint(self.prime - 2);
        let num = (u256_to_big_uint(self.num)
            * u256_to_big_uint(rhs).modpow(&t, &u256_to_big_uint(self.prime)))
            % u256_to_big_uint(self.prime);

        S256Field::new(big_uint_to_u256(&num))
    }
}

impl Display for S256Field {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.num)
    }
}
