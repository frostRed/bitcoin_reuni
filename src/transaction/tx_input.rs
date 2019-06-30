use super::varint::Varint;

use bytes::{BufMut, BytesMut};
use nom::bytes::streaming::take;
use nom::number::complete::le_u32;
use nom::IResult;
use std::fmt::Display;

#[derive(Debug, PartialOrd, PartialEq, Clone, Hash)]
pub struct TxInput {
    pub pre_tx_id: TxHash,
    pub pre_tx_index: PreTxIndex,
    pub script_sig: ScriptSig,
    pub sequence: TxInputSequence,
}

impl TxInput {
    pub fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        let (input, pre_tx_id) = TxHash::parse(&input[..])?;
        let (input, pre_tx_index) = PreTxIndex::parse(&input[..])?;
        let (input, script_sig) = ScriptSig::parse(&input[..])?;
        let (input, sequence) = TxInputSequence::parse(&input[..])?;
        Ok((
            input,
            TxInput {
                pre_tx_id,
                pre_tx_index,
                script_sig,
                sequence,
            },
        ))
    }
    pub fn new(
        pre_tx_id: TxHash,
        pre_tx_index: PreTxIndex,
        script_sig: ScriptSig,
        sequence: TxInputSequence,
    ) -> Self {
        TxInput {
            pre_tx_id,
            pre_tx_index,
            script_sig,
            sequence,
        }
    }

    pub fn serialize(&self) -> Vec<u8> {
        let mut buf = BytesMut::with_capacity(32 + 4 + 9 + self.script_sig.content.len() + 4 + 4);
        let mut pre_tx_id: Vec<u8> = Vec::with_capacity(32);
        pre_tx_id.extend(self.pre_tx_id.0.iter().rev());
        buf.put(&pre_tx_id);
        buf.put_u32_le(self.pre_tx_index.0);
        buf.put(&self.script_sig.serialize());
        buf.put_u32_le(self.sequence.0);
        buf.take().to_vec()
    }
}

impl Display for TxInput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.pre_tx_id, self.pre_tx_index)
    }
}

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
}

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

impl PreTxIndex {
    pub fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        let (input, index) = le_u32(input)?;
        Ok((input, PreTxIndex(index)))
    }
}

#[derive(Debug, PartialOrd, PartialEq, Clone, Hash)]
pub struct ScriptSig {
    content: Vec<u8>,
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

#[derive(Debug, PartialOrd, PartialEq, Clone, Hash)]
pub struct TxInputSequence(u32);
impl Copy for TxInputSequence {}

impl TxInputSequence {
    pub fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        let (input, seq) = le_u32(input)?;
        Ok((input, TxInputSequence(seq)))
    }
}

impl Default for TxInputSequence {
    fn default() -> Self {
        TxInputSequence(0xffffffff)
    }
}

mod test {
    use super::{PreTxIndex, ScriptSig, TxHash, TxInput, TxInputSequence};

    #[test]
    fn test_tx_hash_display() {
        let data = hex!("813f79011acb80925dfe69b3def355fe914bd1d96a3f5f71bf8303c6a989c7d1000000006b483045022100ed81ff192e75a3fd2304004dcadb746fa5e24c5031ccfcf21320b0277457c98f02207a986d955c6e0cb35d446a89d3f56100f4d7f67801c31967743a9c8e10615bed01210349fc4e631e3624a545de3f89f5d8684c7b8138bd94bdd531d2e213bf016b278afeffffff02a135ef01000000001976a914bc3b654dca7e56b04dca18f2566cdaf02e8d9ada88ac99c39800000000001976a9141c4bc762dd5423e332166702cb75f40df79fea1288ac19430600");
        let (_data, pre_tx_id) = TxHash::parse(&data[..]).unwrap();

        assert_eq!(
            format!("{}", pre_tx_id),
            "d1c789a9c60383bf715f3f6ad9d14b91fe55f3deb369fe5d9280cb1a01793f81".to_string()
        );
    }

    #[test]
    fn test_tx_input() {
        let data = hex!("813f79011acb80925dfe69b3def355fe914bd1d96a3f5f71bf8303c6a989c7d1000000006b483045022100ed81ff192e75a3fd2304004dcadb746fa5e24c5031ccfcf21320b0277457c98f02207a986d955c6e0cb35d446a89d3f56100f4d7f67801c31967743a9c8e10615bed01210349fc4e631e3624a545de3f89f5d8684c7b8138bd94bdd531d2e213bf016b278afeffffff02a135ef01000000001976a914bc3b654dca7e56b04dca18f2566cdaf02e8d9ada88ac99c39800000000001976a9141c4bc762dd5423e332166702cb75f40df79fea1288ac19430600");
        let (data, pre_tx_id) = TxHash::parse(&data[..]).unwrap();
        assert_eq!(
            pre_tx_id,
            TxHash(hex!(
                "d1c789a9c60383bf715f3f6ad9d14b91fe55f3deb369fe5d9280cb1a01793f81"
            ))
        );

        let (data, pre_tx_index) = PreTxIndex::parse(&data[..]).unwrap();
        assert_eq!(pre_tx_index, PreTxIndex(0u32));

        let (data, script_sig) = ScriptSig::parse(&data[..]).unwrap();
        assert_eq!(script_sig.content.len(), 107usize);
        assert_eq!(hex::encode(&script_sig.content), "483045022100ed81ff192e75a3fd2304004dcadb746fa5e24c5031ccfcf21320b0277457c98f02207a986d955c6e0cb35d446a89d3f56100f4d7f67801c31967743a9c8e10615bed01210349fc4e631e3624a545de3f89f5d8684c7b8138bd94bdd531d2e213bf016b278a".to_string());

        let (_data, seq) = TxInputSequence::parse(&data[..]).unwrap();
        assert_eq!(seq, TxInputSequence(0xfffffffeu32));

        let tx_input = TxInput::new(pre_tx_id, pre_tx_index, script_sig, seq);
        assert_eq!(
            format!("{}", tx_input),
            "d1c789a9c60383bf715f3f6ad9d14b91fe55f3deb369fe5d9280cb1a01793f81:0".to_string()
        );

        assert_eq!(hex::encode(tx_input.serialize()), "813f79011acb80925dfe69b3def355fe914bd1d96a3f5f71bf8303c6a989c7d1000000006b483045022100ed81ff192e75a3fd2304004dcadb746fa5e24c5031ccfcf21320b0277457c98f02207a986d955c6e0cb35d446a89d3f56100f4d7f67801c31967743a9c8e10615bed01210349fc4e631e3624a545de3f89f5d8684c7b8138bd94bdd531d2e213bf016b278afeffffff".to_string());
    }
}
