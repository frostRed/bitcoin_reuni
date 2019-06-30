use nom::number::complete::le_u32;
use nom::IResult;
use std::fmt::Display;

#[derive(Debug, PartialOrd, PartialEq, Clone, Hash)]
pub struct TxLocktime(u32);
impl Copy for TxLocktime {}

impl AsRef<u32> for TxLocktime {
    fn as_ref(&self) -> &u32 {
        &self.0
    }
}

impl Display for TxLocktime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl TxLocktime {
    pub fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        let (input, locktime) = le_u32(input)?;
        Ok((input, TxLocktime(locktime)))
    }

    pub fn new(locktime: u32) -> Self {
        TxLocktime(locktime)
    }
}

impl From<TxLocktime> for u32 {
    fn from(locktime: TxLocktime) -> u32 {
        locktime.0
    }
}
