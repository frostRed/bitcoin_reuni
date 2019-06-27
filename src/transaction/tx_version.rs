use nom::{number::complete::le_u32, IResult};

#[derive(Debug, PartialOrd, PartialEq, Clone, Hash)]
pub struct TxVersion(u32);
impl Copy for TxVersion {}

impl TxVersion {
    pub fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        let (input, version) = le_u32(input)?;
        Ok((input, TxVersion(version)))
    }
}

mod test {
    use super::TxVersion;

    #[test]
    fn test_parse_version() {
        let data = hex!("01000000");
        assert_eq!(TxVersion(1u32), TxVersion::parse(&data[..]).unwrap().1)
    }
}
