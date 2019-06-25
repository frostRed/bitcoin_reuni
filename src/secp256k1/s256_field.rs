use num_bigint::{BigInt, BigUint, Sign};
use num_traits::{one, zero};
use std::fmt::{self, Display};
use std::ops::{Add, Div, Mul, Sub};

use super::ec::field_element::FieldElementError;
use super::ec::utils::{U256, U512};

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
        let num = Into::<BigUint>::into(self.num);
        let prime = Into::<BigUint>::into(self.prime);

        let mut exp = BigInt::from(exp);
        while exp < zero() {
            exp = exp + BigInt::from_biguint(Sign::Plus, prime.clone() - BigUint::from(1u32));
        }
        let mut e = exp.to_biguint().expect("BigInt convert to BigUint failed");
        // fast very big exp calculate
        e = e % (prime.clone() - BigUint::from(1u32));
        let num: BigUint = num.modpow(&e, &prime);

        S256Field::new(num)
    }

    pub fn prime() -> U256 {
        let p = U512::from(2u32).pow(U512::from(256u32))
            - U512::from(2u32).pow(U512::from(32u32))
            - U512::from(977u32);
        p.into()
    }

    pub fn sqrt(&self) -> Self {
        let prime = Into::<BigUint>::into(self.prime);
        let power = (prime.clone() + BigUint::from(1u8)) / BigUint::from(4u8);
        let new_num = Into::<BigUint>::into(self.num).modpow(&power, &prime);
        S256Field {
            num: new_num.into(),
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

        let num = Into::<BigUint>::into(self.num);
        let rhs_num = Into::<BigUint>::into(rhs.num);
        let prime = Into::<BigUint>::into(self.prime);
        let num: BigUint = (num + rhs_num) % prime;

        S256Field::new(num)
    }
}

impl<T> Add<T> for S256Field
where
    T: Into<U256>,
{
    type Output = S256Field;

    fn add(self, rhs: T) -> Self::Output {
        let num = Into::<BigUint>::into(self.num);
        let rhs_num = Into::<BigUint>::into(rhs.into());
        let prime = Into::<BigUint>::into(self.prime);
        let num: BigUint = (num + rhs_num) % prime;

        S256Field::new(num)
    }
}

impl Add<S256Field> for U256 {
    type Output = S256Field;

    fn add(self, rhs: S256Field) -> Self::Output {
        let num = Into::<BigUint>::into(self);
        let rhs_num = Into::<BigUint>::into(rhs.num);
        let prime = Into::<BigUint>::into(rhs.prime);
        let num: BigUint = (num + rhs_num) % prime;

        S256Field::new(num)
    }
}

impl Sub<Self> for S256Field {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        if self.prime != rhs.prime {
            panic!("{}", FieldElementError::NotSamePrime);
        }

        let self_num = Into::<BigUint>::into(self.num);
        let self_prime = Into::<BigUint>::into(self.prime);
        let rhs_num = Into::<BigUint>::into(rhs.num);

        let mut num: BigInt = zero();
        if self.num >= rhs.num {
            num = BigInt::from_biguint(Sign::Plus, (self_num - rhs_num) % self_prime.clone());
        } else {
            num = BigInt::from_biguint(Sign::Minus, (rhs_num - self_num) % self_prime.clone());
        }
        while num < zero() {
            num = num + BigInt::from_biguint(Sign::Plus, self_prime.clone());
        }
        S256Field::new(num.to_biguint().expect("BigInt convert to BigUint failed"))
    }
}

impl<T> Sub<T> for S256Field
where
    T: Into<U256>,
{
    type Output = Self;

    fn sub(self, rhs: T) -> Self::Output {
        let self_num = Into::<BigUint>::into(self.num);
        let rhs_num = Into::<BigUint>::into(rhs.into());
        let self_prime = Into::<BigUint>::into(self.prime);

        let mut num: BigInt = zero();
        if self_num >= rhs_num {
            num = BigInt::from_biguint(Sign::Plus, (self_num - rhs_num) % self_prime.clone());
        } else {
            num = BigInt::from_biguint(Sign::Minus, (rhs_num - self_num) % self_prime.clone());
        }
        while num < zero() {
            num = num + BigInt::from_biguint(Sign::Plus, self_prime.clone());
        }

        S256Field::new(num.to_biguint().expect("BigInt convert to BigUint failed"))
    }
}

impl Mul<Self> for S256Field {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        if self.prime != rhs.prime {
            panic!("{}", FieldElementError::NotSamePrime);
        }

        let self_num = Into::<BigUint>::into(self.num);
        let rhs_num = Into::<BigUint>::into(rhs.num);
        let self_prime = Into::<BigUint>::into(self.prime);
        let num: BigUint = (self_num * rhs_num) % self_prime;

        S256Field::new(num)
    }
}

impl<T> Mul<T> for S256Field
where
    T: Into<U256>,
{
    type Output = S256Field;
    fn mul(self, rhs: T) -> Self::Output {
        let self_num = Into::<BigUint>::into(self.num);
        let rhs_num = Into::<BigUint>::into(rhs.into());
        let self_prime = Into::<BigUint>::into(self.prime);
        let num: BigUint = (self_num * rhs_num) % self_prime;

        S256Field::new(num)
    }
}

impl Mul<S256Field> for U256 {
    type Output = S256Field;
    fn mul(self, rhs: S256Field) -> Self::Output {
        let self_num = Into::<BigUint>::into(self);
        let rhs_num = Into::<BigUint>::into(rhs.num);
        let prime = Into::<BigUint>::into(rhs.prime);
        let num: BigUint = (self_num * rhs_num) % prime;

        S256Field::new(num)
    }
}

impl Div<Self> for S256Field {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        let t = Into::<BigUint>::into(self.prime - 2);
        let num: BigUint = (Into::<BigUint>::into(self.num)
            * Into::<BigUint>::into(rhs.num).modpow(&t, &Into::<BigUint>::into(self.prime)))
            % Into::<BigUint>::into(self.prime);

        S256Field::new(num)
    }
}

impl Div<U256> for S256Field {
    type Output = Self;

    fn div(self, rhs: U256) -> Self::Output {
        let t: BigUint = (self.prime - 2).into();
        let num: BigUint = (Into::<BigUint>::into(self.num)
            * Into::<BigUint>::into(rhs).modpow(&t, &Into::<BigUint>::into(self.prime)))
            % Into::<BigUint>::into(self.prime);

        S256Field::new(num)
    }
}

impl Display for S256Field {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.num)
    }
}
