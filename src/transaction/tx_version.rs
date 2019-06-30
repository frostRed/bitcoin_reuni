use nom::{number::complete::le_u32, IResult};

#[derive(Debug, PartialOrd, PartialEq, Clone, Hash)]
pub struct TxVersion(u32);
impl Copy for TxVersion {}

impl TxVersion {
    pub fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        let (input, version) = le_u32(input)?;
        Ok((input, TxVersion(version)))
    }

    pub fn new(version: u32) -> Self {
        TxVersion(version)
    }
}

impl From<TxVersion> for u32 {
    fn from(version: TxVersion) -> u32 {
        version.0
    }
}

mod test {
    use super::TxVersion;

    #[test]
    fn test_parse_version() {
        let data = hex!("0100000001813f79011acb80925dfe69b3def355fe914bd1d96a3f5f71bf8303c6a989c7d1000000006b483045022100ed81ff192e75a3fd2304004dcadb746fa5e24c5031ccfcf21320b0277457c98f02207a986d955c6e0cb35d446a89d3f56100f4d7f67801c31967743a9c8e10615bed01210349fc4e631e3624a545de3f89f5d8684c7b8138bd94bdd531d2e213bf016b278afeffffff02a135ef01000000001976a914bc3b654dca7e56b04dca18f2566cdaf02e8d9ada88ac99c39800000000001976a9141c4bc762dd5423e332166702cb75f40df79fea1288ac19430600");
        assert_eq!(TxVersion(1u32), TxVersion::parse(&data[..]).unwrap().1)
    }
}
