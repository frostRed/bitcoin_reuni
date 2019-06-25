use super::s256_field::S256Field;

use super::ec::point::PointError;

use super::ec::utils::U256;
use super::signature::Signature;
use super::utils::{encode_base58_checksum, hash160};
use num_bigint::BigUint;
use num_traits::{one, zero};
use std::fmt;
use std::ops::{Add, Mul};

#[derive(Clone, Debug, Eq, PartialEq)]
enum PointValue {
    InfPoint,
    NormalPoint {
        /// `x` axis
        x: S256Field,
        /// `y` axis
        y: S256Field,
    },
}

impl Copy for PointValue {}

/// Elliptic curve, (y^2) % primer = (x^3 + a*x + b) % primer
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Secp256K1EllipticCurve {
    /// Elliptic curve `a` argument
    a: S256Field,
    /// Elliptic curve `b` argument
    b: S256Field,
}
impl Copy for Secp256K1EllipticCurve {}

impl Default for Secp256K1EllipticCurve {
    fn default() -> Self {
        Secp256K1EllipticCurve {
            a: Self::ec_a(),
            b: Self::ec_b(),
        }
    }
}

impl Secp256K1EllipticCurve {
    pub fn ec_a() -> S256Field {
        S256Field::new(U256::from(0u32))
    }

    pub fn ec_b() -> S256Field {
        S256Field::new(U256::from(7u32))
    }

    /// Secp256K1 elliptic curve group order, nG=0
    pub fn n() -> U256 {
        U256::from_hex(b"fffffffffffffffffffffffffffffffebaaedce6af48a03bbfd25e8cd0364141")
    }
}

/// Elliptic curve point, y^2 = x^3 + a*x + b
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct S256Point {
    point: PointValue,
    elliptic_curve: Secp256K1EllipticCurve,
}

impl fmt::Display for S256Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.point {
            PointValue::InfPoint => write!(
                f,
                "Inf_y^2 = x^3 + {}*x + {}",
                self.elliptic_curve.a, self.elliptic_curve.b
            ),
            PointValue::NormalPoint { x, y } => write!(
                f,
                "Point({}, {})_{}_{} S256Field({})",
                x.num, y.num, self.elliptic_curve.a.num, self.elliptic_curve.b.num, x.prime
            ),
        }
    }
}

impl Copy for S256Point {}

impl S256Point {
    pub fn new(x: S256Field, y: S256Field) -> Result<Self, PointError> {
        let a = Secp256K1EllipticCurve::ec_a();
        let b = Secp256K1EllipticCurve::ec_b();
        let left = y.pow(2);
        let right = x.pow(3) + a * x + b;
        if left != right {
            return Err(PointError::NotInEllipticCurves);
        }

        Ok(S256Point {
            point: PointValue::NormalPoint { x, y },
            elliptic_curve: Secp256K1EllipticCurve::default(),
        })
    }

    pub fn inf() -> Self {
        S256Point {
            point: PointValue::InfPoint,
            elliptic_curve: Secp256K1EllipticCurve::default(),
        }
    }

    pub fn is_inf(&self) -> bool {
        match self.point {
            PointValue::InfPoint => true,
            _ => false,
        }
    }

    pub fn gen_point() -> Self {
        let gx =
            U256::from_hex(b"79be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798");

        let gy =
            U256::from_hex(b"483ada7726a3c4655da4fbfc0e1108a8fd17b448a68554199c47d08ffb10d4b8");
        let x = S256Field::new(gx);
        let y = S256Field::new(gy);
        S256Point::new(x, y).unwrap()
    }

    pub fn coordinate(&self) -> Option<(U256, U256)> {
        match self.point {
            PointValue::InfPoint => None,
            PointValue::NormalPoint { x, y } => Some((x.num, y.num)),
        }
    }

    pub fn verify(&self, z: U256, sig: Signature) -> bool {
        let n = Secp256K1EllipticCurve::n();
        let s_inv = sig.s.modpow(n - U256::from(2u32), n);

        let u = z.modmul(s_inv, n);
        let v = sig.r.modmul(s_inv, n);

        let g = S256Point::gen_point();
        let t = g * u + *self * v;
        sig.r == t.coordinate().unwrap().0
    }

    pub fn sec(&self) -> [u8; 65] {
        let mut buf: Vec<u8> = Vec::with_capacity(65);
        buf.push(b'\x04');

        let (x, y) = self.coordinate().unwrap();
        let mut bytes = [0u8; 32];

        x.to_big_endian(&mut bytes);
        for i in &bytes {
            buf.push(*i);
        }

        y.to_big_endian(&mut bytes);
        for i in &bytes {
            buf.push(*i);
        }

        let mut bytes = [0u8; 65];
        bytes.copy_from_slice(&buf);
        bytes
    }

    pub fn compressed_sec(&self) -> [u8; 33] {
        let mut buf: Vec<u8> = Vec::with_capacity(33);

        let (x, y) = self.coordinate().unwrap();

        if y.is_even() {
            buf.push(b'\x02');
        } else {
            buf.push(b'\x03');
        }

        let mut bytes = [0u8; 32];
        x.to_big_endian(&mut bytes);
        for i in &bytes {
            buf.push(*i);
        }

        let mut bytes = [0u8; 33];
        bytes.copy_from_slice(&buf);
        bytes
    }

    pub fn parse_sec(sec_bytes: &[u8]) -> Self {
        assert!(sec_bytes.len() >= 33);
        if sec_bytes[0] == 4 {
            let x = U256::from_big_endian(&sec_bytes[1..33]);
            let y = U256::from_big_endian(&sec_bytes[33..65]);
            let x = S256Field::new(x);
            let y = S256Field::new(y);
            return S256Point::new(x, y)
                .expect("can not parse uncompressed sec format bytes to S256Point");
        }

        let is_even = if sec_bytes[0] == 2 { true } else { false };
        let x = S256Field::new(U256::from_big_endian(&sec_bytes[1..33]));
        // y^2 = x^3 + 7
        let alpha = x.pow(3) + Secp256K1EllipticCurve::ec_b();
        let beta = alpha.sqrt();

        let prime = S256Field::prime();
        let (even_beta, odd_beta) = if beta.num.is_even() {
            (beta, S256Field::new(prime - beta.num))
        } else {
            (S256Field::new(prime - beta.num), beta)
        };

        if is_even {
            S256Point::new(x, even_beta)
                .expect("can not parse compressed sec format bytes to S256Point")
        } else {
            S256Point::new(x, odd_beta)
                .expect("can not parse compressed sec format bytes to S256Point")
        }
    }

    pub fn hash160(&self, compressed: bool) -> Vec<u8> {
        if compressed {
            hash160(&self.compressed_sec())
        } else {
            hash160(&self.sec())
        }
    }

    pub fn address(&self, compressed: bool, testnet: bool) -> String {
        let h160 = self.hash160(compressed);
        let prefix = if testnet {
            vec![b'\x6f']
        } else {
            vec![b'\x00']
        };

        encode_base58_checksum(&[&prefix[..], &h160[..]].concat())
    }
}

impl Add<S256Point> for S256Point {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        if self.elliptic_curve != rhs.elliptic_curve {
            panic!("{}", PointError::NotInSameEllipticCurves);
        }

        let a = self.elliptic_curve.a;
        let _b = self.elliptic_curve.b;

        match (self.point, rhs.point) {
            (PointValue::NormalPoint { x, y }, PointValue::NormalPoint { x: rhs_x, y: rhs_y }) => {
                if x == rhs_x {
                    // vertical line
                    if y == rhs_y {
                        if y.num == U256::from(0) {
                            return Self::inf();
                        }
                        let s = (U256::from(3) * x.pow(2) + a) / (U256::from(2) * y);
                        let ret_x = s.pow(2) - U256::from(2) * x;
                        let ret_y = s * (x - ret_x) - y;
                        return S256Point::new(ret_x, ret_y).expect("Point add error");
                    }
                    return Self::inf();
                }

                let s = (rhs_y - y) / (rhs_x - x);
                let ret_x = s.pow(2) - x - rhs_x;
                let ret_y = s * (x - ret_x) - y;
                return S256Point::new(ret_x, ret_y).expect("Point add error");
            }
            // self or rhs is inf point
            (PointValue::InfPoint, _) => rhs,
            (_, PointValue::InfPoint) => self,
        }
    }
}

impl<T> Mul<T> for S256Point
where
    T: Into<U256>,
{
    type Output = Self;
    fn mul(self, rhs: T) -> Self::Output {
        let mut coef = rhs.into() % Secp256K1EllipticCurve::n();
        let mut current = self;

        let mut result = S256Point::inf();
        while coef > U256::from(0) {
            if coef & U256::from(1u32) == U256::from(1u32) {
                result = result + current;
            }
            current = current + current;
            coef = coef >> 1;
        }
        result
    }
}

mod test {
    use super::super::ec::utils::U256;
    use super::super::ec::utils::{pow, sha256_to_u256};
    use crate::secp256k1::s256_point::{S256Point, Secp256K1EllipticCurve};
    use crate::secp256k1::signature::Signature;
    use num_bigint::BigUint;

    #[test]
    fn test_s256_point() {
        let n = Secp256K1EllipticCurve::n();
        let gen_point = S256Point::gen_point();

        assert_eq!(S256Point::inf(), gen_point * n)
    }

    #[test]
    fn test_verify_sig() {
        let z = U256::from_hex(b"bc62d4b80d9e36da29c16c5d4d9f11731f36052c72401a76c23c0fb5a9b74423");
        let r = U256::from_hex(b"37206a0610995c58074999cb9767b87af4c4978db68c06e8e6e81d282047a7c6");
        let s = U256::from_hex(b"8ca63759c1157ebeaec0d03cecca119fc9a75bf8e6d0fa65c841c8e2738cdaec");
        let sig = Signature::new(r, s);

        let px =
            U256::from_hex(b"04519fac3d910ca7e7138f7013706f619fa8f033e6ec6e09370ea38cee6a7574");
        let py =
            U256::from_hex(b"82b51eab8c27c66e26c858a079bcdf4f1ada34cec420cafc7eac1a42216fb6c4");
        let point = S256Point::new(px.into(), py.into()).unwrap();
        assert!(point.verify(z, sig))
    }

    #[test]
    fn test_signature_and_verify() {
        let e = sha256_to_u256(b"my secret");
        let z = sha256_to_u256(b"my message");
        let k = U256::from(1234567890u32);

        let r: U256 = (S256Point::gen_point() * k).coordinate().unwrap().0;

        let n = Secp256K1EllipticCurve::n();
        let k_inv = k.modpow(n - U256::from(2), n);

        // U256 mul will overflow
        // let s = u256_modmul(z + r * e, k_inv, n);
        let s: BigUint = (Into::<BigUint>::into(z)
            + Into::<BigUint>::into(r) * Into::<BigUint>::into(e))
            * Into::<BigUint>::into(k_inv);
        let s = s % Into::<BigUint>::into(n);
        let s: U256 = s.into();

        let point = S256Point::gen_point() * e;
        assert_eq!(
            "0x28d003eab2e428d11983f3e97c3fa0addf3b42740df0d211795ffb3be2f6c52",
            "0x".to_string() + &format!("{:x}", point.coordinate().unwrap().0)
        );
        assert_eq!(
            "0xae987b9ec6ea159c78cb2a937ed89096fb218d9e7594f02b547526d8cd309e2",
            "0x".to_string() + &format!("{:x}", point.coordinate().unwrap().1)
        );

        assert_eq!(
            "0x231c6f3d980a6b0fb7152f85cee7eb52bf92433d9919b9c5218cb08e79cce78",
            "0x".to_string() + &format!("{:x}", z)
        );
        assert_eq!(
            "0x2b698a0f0a4041b77e63488ad48c23e8e8838dd1fb7520408b121697b782ef22",
            "0x".to_string() + &format!("{:x}", r)
        );

        assert_eq!(
            "0xbb14e602ef9e3f872e25fad328466b34e6734b7a0fcd58b1eb635447ffae8cb9",
            "0x".to_string() + &format!("{:x}", s)
        );

        assert!(point.verify(z, Signature::new(r, s)))
    }

    #[test]
    fn test_parse_uncompressed_sec() {
        let point = S256Point::gen_point();
        let uncompressed_sec = point.sec();

        for i in uncompressed_sec.iter() {
            println!("{}", *i);
        }
        let parsed_point = S256Point::parse_sec(&uncompressed_sec);
        assert_eq!(point, parsed_point);
    }

    #[test]
    fn test_parse_compressed_sec() {
        let point = S256Point::gen_point();
        let compressed_sec = point.compressed_sec();

        for i in compressed_sec.iter() {
            println!("{}", *i);
        }
        let parsed_point = S256Point::parse_sec(&compressed_sec);
        assert_eq!(point, parsed_point);
    }
}
