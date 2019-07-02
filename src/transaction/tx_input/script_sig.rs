use bytes::{BufMut, BytesMut};
use nom::bytes::streaming::take;
use nom::IResult;

use super::super::varint::Varint;

#[derive(Debug, PartialOrd, PartialEq, Clone, Hash)]
pub struct ScriptSig {
    pub content: Vec<u8>,
}

impl ScriptSig {
    pub fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        let (input, script_sig_len) = Varint::parse(&input[..])?;
        let script_sig_len = Into::<u64>::into(script_sig_len);
        let (input, content) = take(script_sig_len)(input)?;
        Ok((
            input,
            ScriptSig {
                content: content.to_vec(),
            },
        ))
    }

    pub fn serialize(&self) -> Vec<u8> {
        let mut buf = BytesMut::with_capacity(9 + self.content.len() + 4);
        buf.put(Varint::encode(self.content.len() as u64).unwrap());
        buf.put(&self.content);
        buf.take().to_vec()
    }
}

impl Default for ScriptSig {
    fn default() -> Self {
        ScriptSig { content: vec![] }
    }
}
