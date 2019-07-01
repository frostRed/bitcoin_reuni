use nom::number::complete::le_u32;
use nom::IResult;
use std::fmt::Display;

#[derive(Debug, PartialOrd, PartialEq, Clone, Hash)]
pub struct PreTxIndex(u32);
impl Copy for PreTxIndex {}

impl AsRef<u32> for PreTxIndex {
    fn as_ref(&self) -> &u32 {
        &self.0
    }
}

impl Display for PreTxIndex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<PreTxIndex> for u32 {
    fn from(index: PreTxIndex) -> u32 {
        index.0
    }
}

impl PreTxIndex {
    pub fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        let (input, index) = le_u32(input)?;
        Ok((input, PreTxIndex(index)))
    }

    pub fn index(&self) -> u32 {
        self.0
    }

    pub fn new(index: u32) -> Self {
        PreTxIndex(index)
    }
}
