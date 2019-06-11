use crate::ec::field_element::FieldElement;
use num_traits::zero;
use std::fmt;
use std::ops::Add;

#[derive(Clone, Debug, Eq, PartialEq)]
enum PointValue {
    InfPoint,
    NormalPoint {
        /// `x` axis
        x: FieldElement,
        /// `y` axis
        y: FieldElement,
    },
}

/// Elliptic curve, (y^2) % primer = (x^3 + a*x + b) % primer
#[derive(Clone, Debug, Eq, PartialEq)]
struct EllipticCurve {
    /// Elliptic curve `a` argument
    a: FieldElement,
    /// Elliptic curve `b` argument
    b: FieldElement,
}

impl Default for EllipticCurve {
    fn default() -> Self {
        EllipticCurve {
            a: FieldElement::new(0u32, 223u32),
            b: FieldElement::new(7u32, 223u32),
        }
    }
}

impl EllipticCurve {
    fn new(a: FieldElement, b: FieldElement) -> Self {
        EllipticCurve { a, b }
    }
}

/// Elliptic curve point, y^2 = x^3 + a*x + b
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Point {
    point: PointValue,
    elliptic_curve: EllipticCurve,
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.point {
            PointValue::InfPoint => write!(
                f,
                "Inf_y^2 = x^3 + {}*x + {}",
                self.elliptic_curve.a, self.elliptic_curve.b
            ),
            PointValue::NormalPoint { x, y } => write!(
                f,
                "Point({}, {})_{}_{} FieldElement({})",
                x.num, y.num, self.elliptic_curve.a.num, self.elliptic_curve.b.num, x.prime
            ),
        }
    }
}

/// The Error of Point operate
#[derive(Debug, Eq, PartialEq)]
pub enum PointError {
    NotInEllipticCurves,
    NotInSameEllipticCurves,
}
impl fmt::Display for PointError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PointError::NotInEllipticCurves => write!(f, "NotInEllipticCurves Error"),
            PointError::NotInSameEllipticCurves => write!(f, "NotInSameEllipticCurves Error"),
        }
    }
}
impl std::error::Error for PointError {
    fn description(&self) -> &str {
        match self {
            PointError::NotInEllipticCurves => "The Point NotInEllipticCurves",
            PointError::NotInSameEllipticCurves => "The Points NotInSameEllipticCurves",
        }
    }
}

impl Point {
    pub fn new(
        x: FieldElement,
        y: FieldElement,
        a: FieldElement,
        b: FieldElement,
    ) -> Result<Self, PointError> {
        if y.pow(2) != x.pow(3) + a.clone() * x.clone() + b.clone() {
            return Err(PointError::NotInEllipticCurves);
        }
        Ok(Point {
            point: PointValue::NormalPoint { x, y },
            elliptic_curve: EllipticCurve::new(a, b),
        })
    }

    pub fn inf(a: FieldElement, b: FieldElement) -> Self {
        Point {
            point: PointValue::InfPoint,
            elliptic_curve: EllipticCurve::new(a, b),
        }
    }

    pub fn is_inf(&self) -> bool {
        match self.point {
            PointValue::InfPoint => true,
            _ => false,
        }
    }
}

impl Add<Point> for Point {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        if self.elliptic_curve != rhs.elliptic_curve {
            panic!("{}", PointError::NotInSameEllipticCurves);
        }

        let a = self.elliptic_curve.a.clone();
        let b = self.elliptic_curve.b.clone();

        match (self.point.clone(), rhs.point.clone()) {
            (PointValue::NormalPoint { x, y }, PointValue::NormalPoint { x: rhs_x, y: rhs_y }) => {
                if x == rhs_x {
                    // vertical line
                    if y == rhs_y {
                        if y.num == zero() {
                            return Self::inf(a, b);
                        }

                        let s = (x.pow(2) * 3u32 + a.clone()) / (y.clone() * 2u32);
                        let ret_x = s.pow(2) - x.clone() * 2u32;
                        let ret_y = s * (x.clone() - ret_x.clone()) - y.clone();
                        return Point::new(ret_x, ret_y, a, b).expect("Point add error");
                    }
                    return Self::inf(a, b);
                }

                let tmp1 = rhs_y.clone() - y.clone();
                let tmp2 = rhs_x.clone() - x.clone();
                let tmp3 = tmp1 / tmp2;

                let s = (rhs_y.clone() - y.clone()) / (rhs_x.clone() - x.clone());
                let ret_x = s.pow(2) - x.clone() - rhs_x.clone();
                let ret_y = s * (x - ret_x.clone()) - y.clone();
                return Point::new(ret_x, ret_y, a, b).expect("Point add error");
            }
            // self or rhs is inf point
            (PointValue::InfPoint, _) => rhs,
            (_, PointValue::InfPoint) => self,
        }
    }
}

mod test {
    use crate::ec::field_element::FieldElement;
    use crate::ec::point_field_element::{EllipticCurve, Point, PointError, PointValue};

    #[test]
    fn test_display() {
        let x = FieldElement::new(192u32, 223u32);
        let y = FieldElement::new(105u32, 223u32);
        let a = FieldElement::new(0u32, 223u32);
        let b = FieldElement::new(7u32, 223u32);
        let p1 = Point::new(x, y, a, b).unwrap();
        assert_eq!("Point(192, 105)_0_7 FieldElement(223)", format!("{}", p1));
    }

    #[test]
    fn test_on_curve() {
        let prime = 223u32;
        let a = FieldElement::new(0u32, prime);
        let b = FieldElement::new(7u32, 223u32);

        let valid_points: [(u32, u32); 3] = [(192, 105), (17, 56), (1, 193)];
        let invalid_points: [(u32, u32); 2] = [(200, 119), (42, 99)];

        for (x, y) in valid_points.iter() {
            let x = FieldElement::new(*x, prime);
            let y = FieldElement::new(*y, prime);
            assert!(Point::new(x, y, a.clone(), b.clone()).is_ok())
        }

        for (x, y) in invalid_points.iter() {
            let x = FieldElement::new(*x, prime);
            let y = FieldElement::new(*y, prime);
            assert_eq!(
                Point::new(x, y, a.clone(), b.clone()),
                Err(PointError::NotInEllipticCurves)
            )
        }
    }

    #[test]
    fn test_add() {
        let prime = 223u32;
        let a = FieldElement::new(0u32, prime);
        let b = FieldElement::new(7u32, 223u32);

        let x1 = FieldElement::new(192u32, prime);
        let y1 = FieldElement::new(105u32, prime);

        let x2 = FieldElement::new(17u32, prime);
        let y2 = FieldElement::new(56u32, prime);

        let p1 = Point::new(x1, y1, a.clone(), b.clone()).unwrap();
        let p2 = Point::new(x2, y2, a, b).unwrap();

        assert_eq!(
            "Point(170, 142)_0_7 FieldElement(223)",
            format!("{}", p1 + p2)
        );
    }
}
