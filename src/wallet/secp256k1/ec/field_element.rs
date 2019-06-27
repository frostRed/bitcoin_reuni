use num_bigint::{BigInt, BigUint, Sign};
use num_traits::zero;
use std::fmt::{self, Display};
use std::ops::{Add, Div, Mul, Sub};

use super::utils::U256;

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
        let num: BigUint = self.num.into();
        let prime: BigUint = self.prime.into();

        let mut exp = BigInt::from(exp);
        while exp < zero() {
            exp = exp + BigInt::from_biguint(Sign::Plus, prime.clone() - BigUint::from(1u32));
        }
        let mut e = exp.to_biguint().expect("BigInt convert to BigUint failed");
        // fast very big exp calculate
        e = e % (prime.clone() - BigUint::from(1u32));
        let num = num.modpow(&e, &prime);

        FieldElement::new(num.into(), self.prime)
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

        let num: BigUint = self.num.into();
        let rhs_num: BigUint = rhs.num.into();
        let prime: BigUint = self.prime.into();
        let num = (num + rhs_num) % prime;

        FieldElement::new(num.into(), self.prime)
    }
}

impl<T> Add<T> for FieldElement
where
    T: Into<U256>,
{
    type Output = FieldElement;

    fn add(self, rhs: T) -> Self::Output {
        let num: BigUint = self.num.into();
        let rhs_num: BigUint = rhs.into().into();
        let prime: BigUint = self.prime.into();
        let num = (num + rhs_num) % prime;

        FieldElement::new(num.into(), self.prime)
    }
}

impl Add<FieldElement> for U256 {
    type Output = FieldElement;

    fn add(self, rhs: FieldElement) -> Self::Output {
        let num: BigUint = self.into();
        let rhs_num: BigUint = rhs.num.into();
        let prime: BigUint = rhs.prime.into();
        let num = (num + rhs_num) % prime;

        FieldElement::new(num.into(), rhs.prime)
    }
}

impl Sub<Self> for FieldElement {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        if self.prime != rhs.prime {
            panic!("{}", FieldElementError::NotSamePrime);
        }

        let self_num: BigUint = self.num.into();
        let self_prime: BigUint = self.prime.into();
        let rhs_num: BigUint = rhs.num.into();

        let mut num = if self.num >= rhs.num {
            BigInt::from_biguint(Sign::Plus, (self_num - rhs_num) % self_prime.clone())
        } else {
            BigInt::from_biguint(Sign::Minus, (rhs_num - self_num) % self_prime.clone())
        };
        while num < zero() {
            num = num + BigInt::from_biguint(Sign::Plus, self_prime.clone());
        }
        FieldElement::new(
            num.to_biguint()
                .expect("BigInt convert to BigUint failed")
                .into(),
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
        let self_num: BigUint = self.num.into();
        let rhs_num: BigUint = rhs.into().into();
        let self_prime: BigUint = self.prime.into();

        let mut num = if self_num >= rhs_num {
            BigInt::from_biguint(Sign::Plus, (self_num - rhs_num) % self_prime.clone())
        } else {
            BigInt::from_biguint(Sign::Minus, (rhs_num - self_num) % self_prime.clone())
        };
        while num < zero() {
            num = num + BigInt::from_biguint(Sign::Plus, self_prime.clone());
        }

        FieldElement::new(
            num.to_biguint()
                .expect("BigInt convert to BigUint failed")
                .into(),
            self.prime,
        )
    }
}

impl Mul<Self> for FieldElement {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        if self.prime != rhs.prime {
            panic!("{}", FieldElementError::NotSamePrime);
        }

        let self_num: BigUint = self.num.into();
        let rhs_num: BigUint = rhs.num.into();
        let self_prime: BigUint = self.prime.into();
        let num = (self_num * rhs_num) % self_prime;

        FieldElement::new(num.into(), self.prime)
    }
}

impl<T> Mul<T> for FieldElement
where
    T: Into<U256>,
{
    type Output = FieldElement;
    fn mul(self, rhs: T) -> Self::Output {
        let self_num: BigUint = self.num.into();
        let rhs_num: BigUint = rhs.into().into();
        let self_prime: BigUint = self.prime.into();
        let num = (self_num * rhs_num) % self_prime;

        FieldElement::new(num.into(), self.prime)
    }
}

impl Mul<FieldElement> for U256 {
    type Output = FieldElement;
    fn mul(self, rhs: FieldElement) -> Self::Output {
        let self_num: BigUint = self.into();
        let rhs_num: BigUint = rhs.num.into();
        let prime: BigUint = rhs.prime.into();
        let num = (self_num * rhs_num) % prime.clone();

        FieldElement::new(num.into(), prime)
    }
}

impl Div<Self> for FieldElement {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        let t: BigUint = (self.prime - 2).into();
        let prime: BigUint = self.prime.into();
        let num: BigUint = (Into::<BigUint>::into(self.num)
            * Into::<BigUint>::into(rhs.num).modpow(&t, &prime))
            % prime;
        FieldElement::new(num.into(), self.prime)
    }
}

impl Div<U256> for FieldElement {
    type Output = Self;

    fn div(self, rhs: U256) -> Self::Output {
        let t: BigUint = (self.prime - 2).into();
        let prime: BigUint = self.prime.into();
        let num: BigUint = (Into::<BigUint>::into(self.num)
            * Into::<BigUint>::into(rhs).modpow(&t, &prime))
            % prime;

        FieldElement::new(num.into(), self.prime)
    }
}

impl Display for FieldElement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "FieldElement_{}({})", self.num, self.prime)
    }
}

mod test {
    use super::FieldElement;

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
        let _ = a + b;
    }

    #[test]
    fn test_sub() {
        let e1 = FieldElement::new(2, 3);
        let e2 = FieldElement::new(1, 3);
        assert_eq!(e1 - e2, FieldElement::new(1, 3));
    }

    #[test]
    #[should_panic(expected = "NotSamePrime Error")]
    fn test_sub_panic() {
        let e1 = FieldElement::new(2, 3);
        let e3 = FieldElement::new(1, 5);
        let _ = e1 - e3;
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
        let _ = a * d;
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
