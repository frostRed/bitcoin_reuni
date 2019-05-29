use std::fmt::{self, Display};
use std::ops::{Add, Div, Mul, Sub};

/// Finite field element
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FieldElement {
    /// Finite field element number value
    pub num: u64,
    /// Finite field prime, finite field F = {0 , 1, 2, ..., p-1}
    pub prime: u64,
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
    pub fn new(num: u64, prime: u64) -> Self {
        FieldElement { num, prime }
    }

    pub fn pow(self, exp: i32) -> Self {
        let mut e = exp as i64;
        if exp < 0 {
            e += self.prime as i64 - 1;
        }
        debug_assert!(e > 0);
        // fast very big exp calculate
        let e = e as u64 % (self.prime - 1);
        FieldElement::new(self.num.pow(e as u32) % self.prime, self.prime)
    }

    pub fn prime(&self) -> u64 {
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

impl Add<u64> for FieldElement {
    type Output = FieldElement;

    fn add(self, rhs: u64) -> Self::Output {
        FieldElement::new((self.num + rhs) % self.prime, self.prime)
    }
}

impl Add<FieldElement> for u64 {
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
        let mut num = ((self.num as i128 - rhs.num as i128) % (self.prime as i128));
        if num < 0 {
            num += self.prime as i128;
        }
        FieldElement::new(num as u64, self.prime)
    }
}

impl Sub<u64> for FieldElement {
    type Output = Self;

    fn sub(self, rhs: u64) -> Self::Output {
        FieldElement::new((self.num - rhs) % self.prime, self.prime)
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

impl Mul<u64> for FieldElement {
    type Output = FieldElement;
    fn mul(self, rhs: u64) -> Self::Output {
        FieldElement::new((self.num * rhs) % self.prime, self.prime)
    }
}

impl Mul<FieldElement> for u64 {
    type Output = FieldElement;
    fn mul(self, rhs: FieldElement) -> Self::Output {
        FieldElement::new((rhs.num * self) % rhs.prime, rhs.prime)
    }
}

impl Div<Self> for FieldElement {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        let mut num =
            (self.num as i128 * rhs.num.pow(self.prime as u32 - 2u32) as i128) % self.prime as i128;
        if num < 0 {
            num += self.prime as i128;
        }
        FieldElement::new(num as u64, self.prime)
    }
}

impl Div<u64> for FieldElement {
    type Output = Self;

    fn div(self, rhs: u64) -> Self::Output {
        FieldElement::new(
            (self.num * rhs.pow(self.prime as u32 - 2u32)) % self.prime,
            self.prime,
        )
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
