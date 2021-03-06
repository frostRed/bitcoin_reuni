pub trait Hex {
    fn hex(&self) -> String;
}

impl Hex for Vec<u8> {
    fn hex(&self) -> String {
        hex::encode(self)
    }
}

impl Hex for &[u8] {
    fn hex(&self) -> String {
        hex::encode(self)
    }
}

pub trait FromHex {
    fn from_hex(hex: &[u8]) -> Self;
}

mod test {
    use super::Hex;

    #[test]
    fn test_vec_u8_hex() {
        let s = vec![1, 2, 15, 16u8];
        assert_eq!("01020f10".to_string(), s.hex());
    }
}
