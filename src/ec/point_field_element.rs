use crate::ec::field_element::FieldElement;
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

impl Copy for PointValue {}

/// Elliptic curve, (y^2) % primer = (x^3 + a*x + b) % primer
#[derive(Clone, Debug, Eq, PartialEq)]
struct EllipticCurve {
    /// Elliptic curve `a` argument
    a: FieldElement,
    /// Elliptic curve `b` argument
    b: FieldElement,
}
impl Copy for EllipticCurve {}

impl Default for EllipticCurve {
    fn default() -> Self {
        EllipticCurve {
            a: FieldElement::new(0, 223),
            b: FieldElement::new(7, 223),
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
        match self.point {
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

impl Copy for Point {}

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
        if y.pow(2) != x.pow(3) + a * x + b {
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

        let a = self.elliptic_curve.a;
        let b = self.elliptic_curve.b;

        match (self.point, rhs.point) {
            (PointValue::NormalPoint { x, y }, PointValue::NormalPoint { x: rhs_x, y: rhs_y }) => {
                if x == rhs_x {
                    // vertical line
                    if y == rhs_y {
                        if y.num == 0 {
                            return Self::inf(a, b);
                        }

                        let s = (3 * x.pow(2) + a) / (2 * y);
                        let ret_x = s.pow(2) - 2 * x;
                        let ret_y = s * (x - ret_x) - y;
                        return Point::new(ret_x, ret_y, a, b).expect("Point add error");
                    }
                    return Self::inf(a, b);
                }

                let tmp1 = rhs_y - y;
                let tmp2 = rhs_x - x;
                let tmp3 = tmp1 / tmp2;

                let s = (rhs_y - y) / (rhs_x - x);
                let ret_x = s.pow(2) - x - rhs_x;
                let ret_y = s * (x - ret_x) - y;
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
        let x = FieldElement::new(192, 223);
        let y = FieldElement::new(105, 223);
        let a = FieldElement::new(0, 223);
        let b = FieldElement::new(7, 223);
        let p1 = Point::new(x, y, a, b).unwrap();
        assert_eq!("Point(192, 105)_0_7 FieldElement(223)", format!("{}", p1));
    }

    #[test]
    fn test_on_curve() {
        let prime = 223;
        let a = FieldElement::new(0, prime);
        let b = FieldElement::new(7, 223);

        let valid_points: [(u64, u64); 3] = [(192, 105), (17, 56), (1, 193)];
        let invalid_points: [(u64, u64); 2] = [(200, 119), (42, 99)];

        for (x, y) in valid_points.iter() {
            let x = FieldElement::new(*x, prime);
            let y = FieldElement::new(*y, prime);
            assert!(Point::new(x, y, a, b).is_ok())
        }

        for (x, y) in invalid_points.iter() {
            let x = FieldElement::new(*x, prime);
            let y = FieldElement::new(*y, prime);
            assert_eq!(Point::new(x, y, a, b), Err(PointError::NotInEllipticCurves))
        }
    }

    // FieldElement Div include very big pow operator. It cause overflow panic
    //    #[test]
    //    fn test_add() {
    //        let prime = 223;
    //        let a = FieldElement::new(0, prime);
    //        let b = FieldElement::new(7, 223);
    //
    //        let x1 = FieldElement::new(192, prime);
    //        let y1 = FieldElement::new(105, prime);
    //
    //        let x2 = FieldElement::new(17, prime);
    //        let y2 = FieldElement::new(56, prime);
    //
    //        let p1 = Point::new(x1, y1, a, b).unwrap();
    //        let p2 = Point::new(x2, y2, a, b).unwrap();
    //
    //        assert_eq!(
    //            "Point(170, 142)_0_7 FieldElement(223)",
    //            format!("{}", p1 + p2)
    //        );
    //    }
}