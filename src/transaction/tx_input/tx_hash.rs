use nom::bytes::streaming::take;
use nom::IResult;

use std::fmt::Display;
use std::str::FromStr;

#[derive(Debug, PartialOrd, PartialEq, Clone, Hash, Eq)]
pub struct TxHash([u8; 32]);
impl Copy for TxHash {}

impl AsRef<[u8]> for TxHash {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl Display for TxHash {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", hex::encode(self))
    }
}

impl TxHash {
    pub fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        let mut buf: [u8; 32] = Default::default();
        let (input, tx_hash) = take(32usize)(input)?;
        buf.copy_from_slice(&tx_hash[..]);
        // little endian
        buf.reverse();
        Ok((input, TxHash(buf)))
    }

    pub fn hex(&self) -> String {
        format!("{}", self)
    }

    pub fn new(hash: &[u8]) -> IResult<&[u8], Self> {
        let mut buf: [u8; 32] = Default::default();
        let (input, tx_hash) = take(32usize)(hash)?;
        buf.copy_from_slice(&tx_hash[..]);
        Ok((input, TxHash(buf)))
    }

    pub fn to_little_endian(&self) -> Vec<u8> {
        let mut pre_tx_id: Vec<u8> = Vec::with_capacity(32);
        pre_tx_id.extend(self.0.iter().rev());
        pre_tx_id
    }
}

#[derive(Fail, Debug)]
pub enum TxHashError {
    #[fail(display = "parse hex str error")]
    ParseStrError,
    #[fail(display = "hex str decode str error")]
    HexDecodeError,
}

impl FromStr for TxHash {
    type Err = TxHashError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let bytes = hex::decode(s).map_err(|_| TxHashError::HexDecodeError)?;
        if bytes.len() != 32 {
            Err(TxHashError::ParseStrError)
        } else {
            let mut content = [0u8; 32];
            content.copy_from_slice(&bytes[..]);
            Ok(TxHash(content))
        }
    }
}
