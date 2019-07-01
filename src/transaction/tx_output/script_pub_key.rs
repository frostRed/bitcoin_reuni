use bytes::{BufMut, BytesMut};
use nom::bytes::streaming::take;
use nom::IResult;

use std::fmt::Display;

use crate::transaction::varint::Varint;

#[derive(Debug, PartialOrd, PartialEq, Clone, Hash)]
pub struct ScriptPubKey {
    pub content: Vec<u8>,
}

impl Display for ScriptPubKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", hex::encode(&self.content))
    }
}

impl ScriptPubKey {
    pub fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        let (input, script_pub_key_len) = Varint::parse(&input[..])?;
        let script_pub_key_len = Into::<u64>::into(script_pub_key_len);
        let (input, content) = take(script_pub_key_len)(input)?;
        Ok((
            input,
            ScriptPubKey {
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

impl Default for ScriptPubKey {
    fn default() -> Self {
        ScriptPubKey { content: vec![] }
    }
}
