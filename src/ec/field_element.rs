use num_bigint::{BigInt, BigUint, Sign};
use num_traits::{zero, One, Zero};

use std::fmt::{self, Display};
use std::ops::{Add, Div, Mul, Sub};

/// Finite field element
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FieldElement {
    /// Finite field element number value
    pub num: BigUint,
    /// Finite field prime, finite field F = {0 , 1, 2, ..., p-1}
    pub prime: BigUint,
}

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
    pub fn new<T: Into<BigUint>>(num: T, prime: T) -> Self {
        FieldElement {
            num: num.into(),
            prime: prime.into(),
        }
    }

    pub fn pow(&self, exp: isize) -> Self {
        let mut exp = BigInt::from(exp);
        while exp < zero() {
            exp = exp + BigInt::from_biguint(Sign::Plus, self.prime.clone() - BigUint::from(1u32));
        }
        let mut e = exp.to_biguint().expect("BigInt convert to BigUint failed");
        // fast very big exp calculate
        let e = e % (self.prime.clone() - BigUint::from(1u32));
        FieldElement::new(self.num.modpow(&e, &self.prime), self.prime.clone())
    }

    pub fn prime(&self) -> BigUint {
        self.prime.clone()
    }
}

impl Add<Self> for FieldElement {
    type Output = FieldElement;

    fn add(self, rhs: Self) -> Self::Output {
        if self.prime != rhs.prime {
            panic!("{}", FieldElementError::NotSamePrime);
        }
        FieldElement::new(
            (self.num + rhs.num) % self.prime.clone(),
            self.prime.clone(),
        )
    }
}

impl<T> Add<T> for FieldElement
where
    T: Into<BigUint>,
{
    type Output = FieldElement;

    fn add(self, rhs: T) -> Self::Output {
        FieldElement::new(
            (self.num + rhs.into()) % self.prime.clone(),
            self.prime.clone(),
        )
    }
}

impl Add<FieldElement> for BigUint {
    type Output = FieldElement;

    fn add(self, rhs: FieldElement) -> Self::Output {
        FieldElement::new((rhs.num + self) % rhs.prime.clone(), rhs.prime.clone())
    }
}

impl Sub<Self> for FieldElement {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        if self.prime != rhs.prime {
            panic!("{}", FieldElementError::NotSamePrime);
        }
        let mut num: BigInt = zero();
        if self.num >= rhs.num {
            num = BigInt::from_biguint(Sign::Plus, (self.num - rhs.num) % self.prime.clone());
        } else {
            num = BigInt::from_biguint(Sign::Minus, (rhs.num - self.num) % self.prime.clone());
        }
        while num < zero() {
            num = num + BigInt::from_biguint(Sign::Plus, self.prime.clone());
        }
        FieldElement::new(
            num.to_biguint().expect("BigInt convert to BigUint failed"),
            self.prime.clone(),
        )
    }
}

impl<T> Sub<T> for FieldElement
where
    T: Into<BigUint>,
{
    type Output = Self;

    fn sub(self, rhs: T) -> Self::Output {
        FieldElement::new(
            (self.num - rhs.into()) % self.prime.clone(),
            self.prime.clone(),
        )
    }
}

impl Mul<Self> for FieldElement {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        if self.prime != rhs.prime {
            panic!("{}", FieldElementError::NotSamePrime);
        }
        FieldElement::new(
            (self.num * rhs.num) % self.prime.clone(),
            self.prime.clone(),
        )
    }
}

impl<T> Mul<T> for FieldElement
where
    T: Into<BigUint>,
{
    type Output = FieldElement;
    fn mul(self, rhs: T) -> Self::Output {
        FieldElement::new(
            (self.num * rhs.into()) % self.prime.clone(),
            self.prime.clone(),
        )
    }
}

impl Mul<FieldElement> for BigUint {
    type Output = FieldElement;
    fn mul(self, rhs: FieldElement) -> Self::Output {
        FieldElement::new((rhs.num * self) % rhs.prime.clone(), rhs.prime.clone())
    }
}

impl Div<Self> for FieldElement {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        let t = self.prime.clone() - BigUint::from(2u32);
        let num = (self.num * rhs.num.modpow(&t, &self.prime)) % self.prime.clone();
        FieldElement::new(num, self.prime.clone())
    }
}

impl<T> Div<T> for FieldElement
where
    T: Into<BigUint>,
{
    type Output = Self;

    fn div(self, rhs: T) -> Self::Output {
        let t = self.prime.clone() - BigUint::from(2u32);
        let num = (self.num * rhs.into().modpow(&t, &self.prime)) % self.prime.clone();
        FieldElement::new(num, self.prime.clone())
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
        let a = FieldElement::new(1u32, 3u32);
        assert_eq!("FieldElement_1(3)", format!("{}", a));
    }

    #[test]
    fn test_eq() {
        let a = FieldElement::new(7u32, 13u32);
        let b = FieldElement::new(6u32, 13u32);

        assert_ne!(a, b);
        assert_eq!(a, a);
    }

    #[test]
    fn test_add() {
        let a = FieldElement::new(7u32, 13u32);
        let b = FieldElement::new(12u32, 13u32);
        let c = FieldElement::new(6u32, 13u32);
        assert_eq!(a + b, c);
    }

    #[test]
    #[should_panic(expected = "NotSamePrime Error")]
    fn test_add_panic() {
        let a = FieldElement::new(7u32, 13u32);
        let b = FieldElement::new(6u32, 17u32);
        a + b;
    }

    #[test]
    fn test_sub() {
        let e1 = FieldElement::new(2u32, 3u32);
        let e2 = FieldElement::new(1u32, 3u32);
        let e3 = FieldElement::new(1u32, 5u32);
        assert_eq!(e1 - e2, FieldElement::new(1u32, 3u32));
    }

    #[test]
    #[should_panic(expected = "NotSamePrime Error")]
    fn test_sub_panic() {
        let e1 = FieldElement::new(2u32, 3u32);
        let e3 = FieldElement::new(1u32, 5u32);
        e1 - e3;
    }

    #[test]
    fn test_mul() {
        let a = FieldElement::new(7u32, 13u32);
        let b = FieldElement::new(12u32, 13u32);
        let c = FieldElement::new(6u32, 13u32);

        assert_eq!(a * b, c);
    }

    #[test]
    #[should_panic(expected = "NotSamePrime Error")]
    fn test_mul_panic() {
        let a = FieldElement::new(7u32, 13u32);
        let d = FieldElement::new(6u32, 17u32);
        a * d;
    }

    #[test]
    fn test_exp() {
        let a = FieldElement::new(3u32, 13u32);
        let b = FieldElement::new(1u32, 13u32);

        assert_eq!(a.pow(3), b);

        let a = FieldElement::new(7u32, 13u32);
        let b = FieldElement::new(8u32, 13u32);
        assert_eq!(a.pow(-3), b);
    }

    #[test]
    fn test_div() {
        let e1 = FieldElement::new(2u32, 19u32);
        let e2 = FieldElement::new(7u32, 19u32);
        let e3 = FieldElement::new(5u32, 19u32);

        assert_eq!(e1 / e2.clone(), FieldElement::new(3u32, 19u32));
        assert_eq!(e2 / e3, FieldElement::new(9u32, 19u32));
    }
}
