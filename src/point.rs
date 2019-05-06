use std::fmt;
use std::ops::Add;

/// Elliptic curve point, y^2 = x^3 + a*x + b
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Point {
    x: Option<i64>,
    y: Option<i64>,
    a: i64,
    b: i64,
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Point({}, {})_{}_{}", self.x.unwrap(), self.y.unwrap(), self.a, self.b)
    }
}

impl Copy for Point {}

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
            PointError::NotInEllipticCurves => "NotInEllipticCurves",
            PointError::NotInSameEllipticCurves => "NotInSameEllipticCurves",
        }
    }
}

impl Point {
    pub fn new(x: i64, y: i64, a: i64, b: i64) -> Result<Self, PointError> {
        if y.pow(2) != x.pow(3) + a * x + b {
            return Err(PointError::NotInEllipticCurves);
        }
        Ok(Point {
            x: Some(x),
            y: Some(y),
            a,
            b,
        })
    }

    pub fn inf(a: i64, b: i64) -> Self {
        Point {
            x: None,
            y: None,
            a,
            b,
        }
    }
}

impl Add<Point> for Point {
    type Output = Result<Self, PointError>;

    fn add(self, rhs: Self) -> Self::Output {
        if self.a != rhs.a || self.b != rhs.b {
            return Err(PointError::NotInSameEllipticCurves);
        }
        // self or rhs is inf point
        if self.x.is_none() {
            return Ok(rhs);
        }
        if rhs.x.is_none() {
            return Ok(self);
        }
        // vertical line
        if (self.x == rhs.x) && (self.y != rhs.y || (self.y == rhs.y && self.y == Some(0))) {
            return Ok(Self::inf(self.a, self.b));
        }

        if self.x != rhs.x {
            let s = (rhs.y.unwrap() - self.y.unwrap()) / (rhs.x.unwrap() - self.x.unwrap());
            let x = s.pow(2) - self.x.unwrap() - rhs.x.unwrap();
            let y = s * (self.x.unwrap() - x) - self.y.unwrap();
            return Point::new(x, y, self.a, self.b);
        }

        assert_eq!(self, rhs);
        let s = (3 * self.x.unwrap().pow(2) + self.a) / (2 * self.y.unwrap());
        let x = s.pow(2) - 2 * self.x.unwrap();
        let y = s * (self.x.unwrap() - x) - self.y.unwrap();
        Point::new(x, y, self.a, self.b)
    }
}

mod test {
    use crate::point::{Point, PointError};

    #[test]
    fn test_display() {
        let p1 = Point::new(-1, -1, 5, 7).unwrap();
        assert_eq!("Point(-1, -1)_5_7", format!("{}", p1));
    }

    #[test]
    fn test_new() {
        assert_eq!(
            Point::new(-1, -1, 5, 7).unwrap(),
            Point {
                x: Some(-1),
                y: Some(-1),
                a: 5,
                b: 7
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

        assert_eq!(p1 + inf, Ok(p1));
        assert_eq!(p2 + inf, Ok(p2));
        assert_eq!(p1 + p2, Ok(inf));
        assert_eq!(
            p1 + p3,
            Ok(Point {
                x: Some(3),
                y: Some(-7),
                a: 5,
                b: 7
            })
        );
        assert_eq!(
            p1 + p1,
            Ok(Point {
                x: Some(18),
                y: Some(77),
                a: 5,
                b: 7
            })
        )
    }
}
