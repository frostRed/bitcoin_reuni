use std::fmt::{self, Display};
use std::ops::{Add, Sub};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FieldElement {
    num: i64,
    prime: u64,
}

#[derive(Debug, Eq, PartialEq)]
pub enum FieldElementError {
    PrimeNotEq,
}

impl fmt::Display for FieldElementError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            FieldElementError::PrimeNotEq => write!(f, "PrimeNotEq Error"),
        }
    }
}

impl std::error::Error for FieldElementError {
    fn description(&self) -> &str {
        match self {
            FieldElementError::PrimeNotEq => "PrimeNotEq",
        }
    }
}

impl FieldElement {
    pub fn new(num: i64, prime: u64) -> Self {
        FieldElement { num, prime }
    }
}

impl Add<Self> for FieldElement {
    type Output = Result<Self, FieldElementError>;

    fn add(self, rhs: Self) -> Self::Output {
        if self.prime != rhs.prime {
            return Err(FieldElementError::PrimeNotEq);
        }
        Ok(FieldElement::new(
            (self.num + rhs.num) % self.prime as i64,
            self.prime,
        ))
    }
}

impl Sub<Self> for FieldElement {
    type Output = Result<Self, FieldElementError>;

    fn sub(self, rhs: Self) -> Self::Output {
        if self.prime != rhs.prime {
            return Err(FieldElementError::PrimeNotEq);
        }
        Ok(FieldElement::new(
            (self.num - rhs.num) % self.prime as i64,
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
        let e = FieldElement::new(1, 3);
        assert_eq!("FieldElement_1(3)", format!("{}", e));
    }

    #[test]
    fn test_eq() {
        let e1 = FieldElement::new(1, 3);
        let e2 = FieldElement::new(1, 3);
        let e3 = FieldElement::new(2, 3);
        let e4 = FieldElement::new(1, 5);

        assert_eq!(e1, e2);
        assert_ne!(e1, e3);
        assert_ne!(e1, e4);
    }

    #[test]
    fn test_add() {
        let e1 = FieldElement::new(1, 3);
        let e2 = FieldElement::new(1, 3);
        let e3 = FieldElement::new(1, 5);
        let e4 = FieldElement::new(2, 3);

        assert_eq!(e1.clone() + e2, Ok(FieldElement::new(2, 3)));
        assert_eq!(e1.clone() + e3, Err(FieldElementError::PrimeNotEq));
        assert_eq!(e1 + e4, Ok(FieldElement::new(0, 3)));
    }

    #[test]
    fn test_sub() {
        let e1 = FieldElement::new(2, 3);
        let e2 = FieldElement::new(1, 3);
        let e3 = FieldElement::new(1, 5);

        assert_eq!(e1.clone() - e2, Ok(FieldElement::new(1, 3)));
        assert_eq!(e1 - e3, Err(FieldElementError::PrimeNotEq));
    }
}
