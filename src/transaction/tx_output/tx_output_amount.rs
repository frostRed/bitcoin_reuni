use nom::number::complete::le_u64;
use nom::IResult;
use std::fmt::Display;

#[derive(Debug, PartialOrd, PartialEq, Clone, Hash)]
pub struct TxOutputAmount(u64);
impl Copy for TxOutputAmount {}

impl Display for TxOutputAmount {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<TxOutputAmount> for u64 {
    fn from(amount: TxOutputAmount) -> u64 {
        amount.0
    }
}

impl TxOutputAmount {
    pub fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        let (input, index) = le_u64(input)?;
        Ok((input, TxOutputAmount(index)))
    }
}
