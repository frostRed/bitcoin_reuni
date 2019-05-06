use std::fmt::{self, Display};
use std::ops::{Add, Div, Mul, Sub};

/// Finite field element
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FieldElement {
    num: u64,
    prime: u64,
}

impl Copy for FieldElement {}

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
            FieldElementError::NotSamePrime => "NotSamePrime",
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
        // reduce very big exp
        let e = e as u64 % (self.prime - 1);
        FieldElement::new(self.num.pow(e as u32) % self.prime, self.prime)
    }
}

impl Add<Self> for FieldElement {
    type Output = Result<Self, FieldElementError>;

    fn add(self, rhs: Self) -> Self::Output {
        if self.prime != rhs.prime {
            return Err(FieldElementError::NotSamePrime);
        }
        Ok(FieldElement::new(
            (self.num + rhs.num) % self.prime,
            self.prime,
        ))
    }
}

impl Sub<Self> for FieldElement {
    type Output = Result<Self, FieldElementError>;

    fn sub(self, rhs: Self) -> Self::Output {
        if self.prime != rhs.prime {
            return Err(FieldElementError::NotSamePrime);
        }
        Ok(FieldElement::new(
            (self.num - rhs.num) % self.prime,
            self.prime,
        ))
    }
}

impl Mul<Self> for FieldElement {
    type Output = Result<Self, FieldElementError>;

    fn mul(self, rhs: Self) -> Self::Output {
        if self.prime != rhs.prime {
            return Err(FieldElementError::NotSamePrime);
        }
        Ok(FieldElement::new(
            (self.num * rhs.num) % self.prime,
            self.prime,
        ))
    }
}

impl Div<Self> for FieldElement {
    type Output = Result<Self, FieldElementError>;

    fn div(self, rhs: Self) -> Self::Output {
        if self.prime != rhs.prime {
            return Err(FieldElementError::NotSamePrime);
        }
        Ok(FieldElement::new(
            (self.num * rhs.num.pow(self.prime as u32 - 2u32)) % self.prime,
            self.prime,
        ))
    }
}

impl Display for FieldElement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "FieldElement_{}({})", self.num, self.prime)
    }
}

mod test {
    use crate::field_element::{FieldElement, FieldElementError};

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
        let d = FieldElement::new(6, 17);

        assert_eq!(a + b, Ok(c));
        assert_eq!(a + d, Err(FieldElementError::NotSamePrime));
    }

    #[test]
    fn test_sub() {
        let e1 = FieldElement::new(2, 3);
        let e2 = FieldElement::new(1, 3);
        let e3 = FieldElement::new(1, 5);

        assert_eq!(e1 - e2, Ok(FieldElement::new(1, 3)));
        assert_eq!(e1 - e3, Err(FieldElementError::NotSamePrime));
    }

    #[test]
    fn test_mul() {
        let a = FieldElement::new(7, 13);
        let b = FieldElement::new(12, 13);
        let c = FieldElement::new(6, 13);
        let d = FieldElement::new(6, 17);

        assert_eq!(a * b, Ok(c));
        assert_eq!(a * d, Err(FieldElementError::NotSamePrime));
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

        assert_eq!(e1 / e2, Ok(FieldElement::new(3, 19)));
        assert_eq!(e2 / e3, Ok(FieldElement::new(9, 19)));
    }
}
