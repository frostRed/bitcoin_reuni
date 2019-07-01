use crate::transaction::varint::Varint;
use bytes::{BufMut, BytesMut};
use nom::bytes::streaming::take;
use nom::number::complete::le_u64;
use nom::IResult;
use std::fmt::Display;

#[derive(Debug, PartialOrd, PartialEq, Clone, Hash)]
pub struct TxOutput {
    pub amount: u64,
    pub script_pub_key: ScriptPubKey,
}

impl Display for TxOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.amount, self.script_pub_key)
    }
}

impl TxOutput {
    pub fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        let (input, amount) = le_u64(input)?;
        let (input, script_pub_key) = ScriptPubKey::parse(input)?;
        Ok((
            input,
            TxOutput {
                amount,
                script_pub_key,
            },
        ))
    }

    pub fn serialize(&self) -> Vec<u8> {
        let mut buf = BytesMut::with_capacity(8 + 9 + self.script_pub_key.content.len() + 4);
        buf.put_u64_le(self.amount);
        buf.put(self.script_pub_key.serialize());
        buf.take().to_vec()
    }
}

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

mod test {
    use super::{ScriptPubKey, TxOutput};

    #[test]
    fn test_script_pub_key() {
        let data = hex!("1976a914bc3b654dca7e56b04dca18f2566cdaf02e8d9ada88ac99c39800000000001976a9141c4bc762dd5423e332166702cb75f40df79fea1288ac19430600");
        let (_data, script_pub_key) = ScriptPubKey::parse(&data[..]).unwrap();
        assert_eq!(script_pub_key.content.len(), 25usize);
        assert_eq!(
            format!("{}", script_pub_key),
            "76a914bc3b654dca7e56b04dca18f2566cdaf02e8d9ada88ac".to_string()
        );
    }

    #[test]
    fn test_tx_ouput() {
        let data = hex!("a135ef01000000001976a914bc3b654dca7e56b04dca18f2566cdaf02e8d9ada88ac99c39800000000001976a9141c4bc762dd5423e332166702cb75f40df79fea1288ac19430600");
        let (_data, tx_output) = TxOutput::parse(&data[..]).unwrap();
        assert_eq!(tx_output.amount, 32454049u64);
        assert_eq!(tx_output.script_pub_key.content.len(), 25usize);

        assert_eq!(
            format!("{}", tx_output),
            "32454049:76a914bc3b654dca7e56b04dca18f2566cdaf02e8d9ada88ac".to_string()
        );

        assert_eq!(
            hex::encode(tx_output.serialize()),
            "a135ef01000000001976a914bc3b654dca7e56b04dca18f2566cdaf02e8d9ada88ac".to_string()
        );
    }
}
