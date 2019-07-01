use nom::number::complete::le_u32;
use nom::IResult;

#[derive(Debug, PartialOrd, PartialEq, Clone, Hash)]
pub struct TxInputSequence(u32);
impl Copy for TxInputSequence {}

impl TxInputSequence {
    pub fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        let (input, seq) = le_u32(input)?;
        Ok((input, TxInputSequence(seq)))
    }

    pub fn sequence(&self) -> u32 {
        self.0
    }

    pub fn new(seq: u32) -> Self {
        TxInputSequence(seq)
    }
}

impl Default for TxInputSequence {
    fn default() -> Self {
        TxInputSequence(0xffffffff)
    }
}
