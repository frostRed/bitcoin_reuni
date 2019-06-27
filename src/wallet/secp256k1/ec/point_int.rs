use std::fmt;
use std::ops::Add;

#[derive(Clone, Debug, Eq, PartialEq)]
enum PointValue {
    InfPoint,
    NormalPoint {
        /// `x` axis
        x: i64,
        /// `y` axis
        y: i64,
    },
}

impl Copy for PointValue {}

/// Elliptic curve, y^2 = x^3 + a*x + b
#[derive(Clone, Debug, Eq, PartialEq)]
struct EllipticCurve {
    /// Elliptic curve `a` argument
    a: i64,
    /// Elliptic curve `b` argument
    b: i64,
}
impl Copy for EllipticCurve {}

impl EllipticCurve {
    fn new(a: i64, b: i64) -> Self {
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
                "({}, {})_y^2 = x^3 + {}*x + {}",
                x, y, self.elliptic_curve.a, self.elliptic_curve.b
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
    pub fn new(x: i64, y: i64, a: i64, b: i64) -> Result<Self, PointError> {
        if y.pow(2) != x.pow(3) + a * x + b {
            return Err(PointError::NotInEllipticCurves);
        }
        Ok(Point {
            point: PointValue::NormalPoint { x, y },
            elliptic_curve: EllipticCurve::new(a, b),
        })
    }

    pub fn inf(a: i64, b: i64) -> Self {
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
                        if y == 0 {
                            return Self::inf(a, b);
                        }

                        let s = (3 * x.pow(2) + a) / (2 * y);
                        let ret_x = s.pow(2) - 2 * x;
                        let ret_y = s * (x - ret_x) - y;
                        return Point::new(ret_x, ret_y, a, b).expect("Point add error");
                    }
                    return Self::inf(a, b);
                }

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
    use super::{EllipticCurve, Point, PointError, PointValue};

    #[test]
    fn test_display() {
        let p1 = Point::new(-1, -1, 5, 7).unwrap();
        assert_eq!("(-1, -1)_y^2 = x^3 + 5*x + 7", format!("{}", p1));
    }

    #[test]
    fn test_new() {
        assert_eq!(
            Point::new(-1, -1, 5, 7).unwrap(),
            Point {
                point: PointValue::NormalPoint { x: -1, y: -1 },
                elliptic_curve: EllipticCurve { a: 5, b: 7 }
            }
        );
        assert_eq!(
            Point::new(-1, -2, 5, 7),
            Err(PointError::NotInEllipticCurves)
        );
    }

    #[test]
    fn test_add() {
        let p1 = Point::new(-1, -1, 5, 7).unwrap();
        let p2 = Point::new(-1, 1, 5, 7).unwrap();
        let p3 = Point::new(2, 5, 5, 7).unwrap();
        let inf = Point::inf(5, 7);

        assert_eq!(p1 + inf, p1);
        assert_eq!(p2 + inf, p2);
        assert_eq!(p1 + p2, inf);
        assert_eq!(p1 + p3, Point::new(3, -7, 5, 7).unwrap());
        assert_eq!(
            p1 + p1,
            Point {
                point: PointValue::NormalPoint { x: 18, y: 77 },
                elliptic_curve: EllipticCurve { a: 5, b: 7 }
            }
        )
    }
}
