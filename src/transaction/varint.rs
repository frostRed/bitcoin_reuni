use bytes::{BufMut, BytesMut};
use nom::{
    number::complete::{le_u16, le_u32, le_u64, le_u8},
    IResult,
};

#[derive(Debug, PartialOrd, PartialEq, Clone, Hash)]
pub enum Varint {
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
}
impl Copy for Varint {}

impl Into<u64> for Varint {
    fn into(self) -> u64 {
        match self {
            Varint::U8(int) => int as u64,
            Varint::U16(int) => int as u64,
            Varint::U32(int) => int as u64,
            Varint::U64(int) => int as u64,
        }
    }
}

/// The Error of Varint
#[derive(Debug, Eq, PartialEq)]
pub enum VarintError {
    IntTooLarge,
}

impl std::fmt::Display for VarintError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            VarintError::IntTooLarge => write!(f, "IntTooLarge Error"),
        }
    }
}

impl std::error::Error for VarintError {
    fn description(&self) -> &str {
        match self {
            VarintError::IntTooLarge => "integer too large",
        }
    }
}

impl Varint {
    pub fn encode(&self) -> Result<Vec<u8>, VarintError> {
        let int: u64 = (*self).into();

        let mut buf = BytesMut::with_capacity(10);
        if int < 0xfd_u64 {
            buf.put_u8(int as u8);
        } else if int < 0x10000_u64 {
            buf.put(&b"\xfd"[..]);
            buf.put_u16_le(int as u16);
        } else if int < 0x100000000_u64 {
            buf.put(&b"\xfe"[..]);
            buf.put_u32_le(int as u32);
        } else if (int as u128) < 0x10000000000000000u128 {
            buf.put(&b"\xff"[..]);
            buf.put_u64_le(int as u64);
        } else {
            return Err(VarintError::IntTooLarge);
        };
        Ok(buf.take().to_vec())
    }

    pub fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        let i = input[0];
        let (input, varint) = if i == 0xfd {
            let (input, int) = le_u16(&input[1..])?;
            (input, Varint::U16(int))
        } else if i == 0xfe {
            let (input, int) = le_u32(&input[1..])?;
            (input, Varint::U32(int))
        } else if i == 0xff {
            let (input, int) = le_u64(&input[1..])?;
            (input, Varint::U64(int))
        } else {
            let (input, int) = le_u8(input)?;
            (input, Varint::U8(int))
        };

        Ok((input, varint))
    }
}

mod test {
    use super::Varint;

    #[test]
    fn test_parse_varint() {
        let data = hex!("01");
        assert_eq!(Varint::U8(1u8), Varint::parse(&data[..]).unwrap().1)
    }

    #[test]
    fn test_encode_varint() {
        let varint = Varint::U8(1u8);
        let data = hex!("01");
        assert_eq!(varint.encode().unwrap(), &data[..])
    }
}
