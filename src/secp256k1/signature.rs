use super::ec::utils::U256;
use std::collections::VecDeque;
use std::fmt::Display;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Signature {
    pub r: U256,
    pub s: U256,
}

impl Copy for Signature {}

impl Display for Signature {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Signature({}, {})", self.r, self.s)
    }
}

impl Signature {
    pub fn new(r: U256, s: U256) -> Self {
        Signature { r, s }
    }

    fn u256_der(v: U256) -> VecDeque<u8> {
        let mut buf = [0u8; 32];
        v.to_big_endian(&mut buf);

        let mut ret = VecDeque::new();
        for i in buf.iter() {
            if *i != b'\x00' {
                ret.push_back(*i);
            }
        }
        if ret.front().expect("VecDeque is empty") & 0x80 > 0u8 {
            ret.push_front(b'\x00');
        }
        let rbin_len = ret.len();

        ret.push_front(rbin_len as u8);
        ret.push_front(2u8);
        ret
    }

    pub fn der(&self) -> Vec<u8> {
        let mut ret: VecDeque<u8> = VecDeque::new();
        ret.append(&mut Self::u256_der(self.r));
        ret.append(&mut Self::u256_der(self.s));
        ret.push_front(ret.len() as u8);
        ret.push_front(b'\x30');

        ret.into_iter().collect()
    }

    fn parse_der_u256(bytes: &[u8]) -> U256 {
        let mut buf = [0u8; 32];
        assert_eq!(bytes[0], b'\x02');
        let len = bytes[1] as usize;
        assert!(len <= 33);
        let slice = if bytes[2] == b'\x00' {
            &bytes[3..2 + len]
        } else {
            &bytes[2..2 + len]
        };
        let zero_count = 32 - slice.len();
        for i in 0..zero_count {
            buf[i] = 0u8;
        }
        for (i, v) in slice.iter().enumerate() {
            buf[zero_count + i] = *v;
        }
        U256::from_big_endian(&buf)
    }

    pub fn parse_der(der_bytes: &[u8]) -> Self {
        assert_eq!(der_bytes[0], b'\x30');
        assert!(der_bytes.len() > der_bytes[1] as usize + 1);

        let r_len = der_bytes[3] as usize;
        let r = Self::parse_der_u256(&der_bytes[2..4 + r_len]);

        let s_len = der_bytes[5 + r_len] as usize;
        let s = Self::parse_der_u256(&der_bytes[4 + r_len..6 + r_len + s_len]);

        Signature::new(r, s)
    }
}

mod test {
    use crate::secp256k1::ec::utils::u256_parse_str;
    use crate::secp256k1::signature::Signature;

    #[test]
    fn test_sig_der_and_parse() {
        let r = u256_parse_str(
            b"37206a0610995c58074999cb9767b87af4c4978db68c06e8e6e81d282047a7c6",
            16,
        );
        let s = u256_parse_str(
            b"8ca63759c1157ebeaec0d03cecca119fc9a75bf8e6d0fa65c841c8e2738cdaec",
            16,
        );
        let sig = Signature::new(r, s);
        let der = sig.der();

        let parsed_sig = Signature::parse_der(&der);
        assert_eq!(sig, parsed_sig)
    }
}
