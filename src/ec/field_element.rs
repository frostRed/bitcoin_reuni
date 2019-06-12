use num_bigint::{BigInt, BigUint, Sign};
use num_traits::zero;
use std::fmt::{self, Display};
use std::ops::{Add, Div, Mul, Sub};

construct_uint! {
    pub struct U256(4);
}

construct_uint! {
    pub struct U512(8);
}

/// Finite field element
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FieldElement {
    /// Finite field element number value
    pub num: U256,
    /// Finite field prime, finite field F = {0 , 1, 2, ..., p-1}
    pub prime: U256,
}

impl Copy for FieldElement {}

/// The Error of FieldElement operate
#[derive(Debug, Eq, PartialEq)]
pub enum FieldElementError {
    NotSamePrime,
}

impl fmt::Display for FieldElementError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            FieldElementError::NotSamePrime => write!(f, "NotSamePrime Error"),
        }
    }
}

impl std::error::Error for FieldElementError {
    fn description(&self) -> &str {
        match self {
            FieldElementError::NotSamePrime => "The FieldElements NotSamePrime",
        }
    }
}

impl FieldElement {
    pub fn new<T: Into<U256>>(num: T, prime: T) -> Self {
        FieldElement {
            num: num.into(),
            prime: prime.into(),
        }
    }

    pub fn pow(self, exp: i32) -> Self {
        let mut positive_exp: U256 = 0.into();
        if exp < 0 {
            let t: U256 = self.prime - U256::from(1);
            positive_exp = t - (U256::from(-exp as u32) % t);
        } else {
            positive_exp = U256::from(exp as u32);
        }
        // fast very big exp calculate
        let exp: U256 = positive_exp % (self.prime - U256::from(1));

        let num = self.num.pow(exp) % self.prime;
        FieldElement::new(num, self.prime)
    }

    pub fn prime(&self) -> U256 {
        self.prime
    }
}

impl Add<Self> for FieldElement {
    type Output = FieldElement;

    fn add(self, rhs: Self) -> Self::Output {
        if self.prime != rhs.prime {
            panic!("{}", FieldElementError::NotSamePrime);
        }
        FieldElement::new((self.num + rhs.num) % self.prime, self.prime)
    }
}

impl<T> Add<T> for FieldElement
where
    T: Into<U256>,
{
    type Output = FieldElement;

    fn add(self, rhs: T) -> Self::Output {
        FieldElement::new((self.num + rhs.into()) % self.prime, self.prime)
    }
}

impl Add<FieldElement> for U256 {
    type Output = FieldElement;

    fn add(self, rhs: FieldElement) -> Self::Output {
        FieldElement::new((rhs.num + self) % rhs.prime, rhs.prime)
    }
}

impl Sub<Self> for FieldElement {
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
        FieldElement::new(
            big_uint_to_u256(&num.to_biguint().expect("BigInt convert to BigUint failed")),
            self.prime,
        )
    }
}

impl<T> Sub<T> for FieldElement
where
    T: Into<U256>,
{
    type Output = Self;

    fn sub(self, rhs: T) -> Self::Output {
        FieldElement::new((self.num - rhs.into()) % self.prime, self.prime)
    }
}

impl Mul<Self> for FieldElement {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        if self.prime != rhs.prime {
            panic!("{}", FieldElementError::NotSamePrime);
        }
        FieldElement::new((self.num * rhs.num) % self.prime, self.prime)
    }
}

impl<T> Mul<T> for FieldElement
where
    T: Into<U256>,
{
    type Output = FieldElement;
    fn mul(self, rhs: T) -> Self::Output {
        FieldElement::new((self.num * rhs.into()) % self.prime, self.prime)
    }
}

impl Mul<FieldElement> for U256 {
    type Output = FieldElement;
    fn mul(self, rhs: FieldElement) -> Self::Output {
        FieldElement::new((rhs.num * self) % rhs.prime, rhs.prime)
    }
}

fn u256_to_big_uint(v: U256) -> BigUint {
    let mut u256_bytes = [0u8; 32];
    v.to_little_endian(&mut u256_bytes);
    BigUint::from_bytes_le(&u256_bytes)
}

fn big_uint_to_u256(v: &BigUint) -> U256 {
    let big_uint_bytes = v.to_bytes_le();
    U256::from_little_endian(&big_uint_bytes)
}

impl Div<Self> for FieldElement {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        let t = u256_to_big_uint(self.prime - 2);
        let num = (u256_to_big_uint(self.num)
            * u256_to_big_uint(rhs.num).modpow(&t, &u256_to_big_uint(self.prime)))
            % u256_to_big_uint(self.prime);

        FieldElement::new(big_uint_to_u256(&num), self.prime)
    }
}

impl Div<U256> for FieldElement {
    type Output = Self;

    fn div(self, rhs: U256) -> Self::Output {
        let t = u256_to_big_uint(self.prime - 2);
        let num = (u256_to_big_uint(self.num)
            * u256_to_big_uint(rhs).modpow(&t, &u256_to_big_uint(self.prime)))
            % u256_to_big_uint(self.prime);

        FieldElement::new(big_uint_to_u256(&num), self.prime)
    }
}

impl Display for FieldElement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "FieldElement_{}({})", self.num, self.prime)
    }
}

mod test {
    use crate::ec::field_element::{FieldElement, FieldElementError};

    #[test]
    fn test_display() {
        let a = FieldElement::new(1, 3);
        assert_eq!("FieldElement_1(3)", format!("{}", a));
    }

    #[test]
    fn test_eq() {
        let a = FieldElement::new(7, 13);
        let b = FieldElement::new(6, 13);

        assert_ne!(a, b);
        assert_eq!(a, a);
    }

    #[test]
    fn test_add() {
        let a = FieldElement::new(7, 13);
        let b = FieldElement::new(12, 13);
        let c = FieldElement::new(6, 13);
        assert_eq!(a + b, c);
    }

    #[test]
    #[should_panic(expected = "NotSamePrime Error")]
    fn test_add_panic() {
        let a = FieldElement::new(7, 13);
        let b = FieldElement::new(6, 17);
        a + b;
    }

    #[test]
    fn test_sub() {
        let e1 = FieldElement::new(2, 3);
        let e2 = FieldElement::new(1, 3);
        let e3 = FieldElement::new(1, 5);
        assert_eq!(e1 - e2, FieldElement::new(1, 3));
    }

    #[test]
    #[should_panic(expected = "NotSamePrime Error")]
    fn test_sub_panic() {
        let e1 = FieldElement::new(2, 3);
        let e3 = FieldElement::new(1, 5);
        e1 - e3;
    }

    #[test]
    fn test_mul() {
        let a = FieldElement::new(7, 13);
        let b = FieldElement::new(12, 13);
        let c = FieldElement::new(6, 13);

        assert_eq!(a * b, c);
    }

    #[test]
    #[should_panic(expected = "NotSamePrime Error")]
    fn test_mul_panic() {
        let a = FieldElement::new(7, 13);
        let d = FieldElement::new(6, 17);
        a * d;
    }
    #[test]
    fn test_exp() {
        let a = FieldElement::new(3, 13);
        let b = FieldElement::new(1, 13);

        assert_eq!(a.pow(3), b);

        let a = FieldElement::new(7, 13);
        let b = FieldElement::new(8, 13);
        assert_eq!(a.pow(-3), b);
    }

    #[test]
    fn test_div() {
        let e1 = FieldElement::new(2, 19);
        let e2 = FieldElement::new(7, 19);
        let e3 = FieldElement::new(5, 19);

        assert_eq!(e1 / e2, FieldElement::new(3, 19));
        assert_eq!(e2 / e3, FieldElement::new(9, 19));
    }
}
