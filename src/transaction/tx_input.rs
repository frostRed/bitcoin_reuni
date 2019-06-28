use super::varint::Varint;

use nom::bytes::streaming::take;
use nom::number::complete::le_u32;
use nom::IResult;

#[derive(Debug, PartialOrd, PartialEq, Clone, Hash)]
pub struct TxInput {
    pre_tx_id: TxHash,
    pre_tx_index: PreTxIndex,
    script_sig: ScriptSig,
    sequence: TxInputSequence,
}

impl TxInput {
    pub fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        let (input, pre_tx_id) = TxHash::parse(&input[..])?;
        let (data, pre_tx_index) = PreTxIndex::parse(&input[..])?;
        let (data, script_sig) = ScriptSig::parse(&input[..])?;
        let (data, sequence) = TxInputSequence::parse(&input[..])?;
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
}

#[derive(Debug, PartialOrd, PartialEq, Clone, Hash)]
struct TxHash([u8; 32]);
impl Copy for TxHash {}

impl TxHash {
    pub fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        let mut buf: [u8; 32] = Default::default();
        let (input, tx_hash) = take(32usize)(input)?;
        buf.copy_from_slice(&tx_hash[..]);
        Ok((input, TxHash(buf)))
    }
}

#[derive(Debug, PartialOrd, PartialEq, Clone, Hash)]
struct PreTxIndex(u32);
impl Copy for PreTxIndex {}

impl PreTxIndex {
    pub fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        let (input, index) = le_u32(input)?;
        Ok((input, PreTxIndex(index)))
    }
}

#[derive(Debug, PartialOrd, PartialEq, Clone, Hash)]
pub struct ScriptSig {
    len: u64,
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
                len: script_sig_len,
                content: content.to_vec(),
            },
        ))
    }
}

#[derive(Debug, PartialOrd, PartialEq, Clone, Hash)]
struct TxInputSequence(u32);
impl Copy for TxInputSequence {}

impl TxInputSequence {
    pub fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        let (input, seq) = le_u32(input)?;
        Ok((input, TxInputSequence(seq)))
    }
}
mod test {
    use crate::transaction::tx_input::{PreTxIndex, ScriptSig, TxHash, TxInputSequence};
    use crate::transaction::varint::Varint;

    #[test]
    fn test_tx_input() {
        let data = hex!("813f79011acb80925dfe69b3def355fe914bd1d96a3f5f71bf8303c6a989c7d1000000006b483045022100ed81ff192e75a3fd2304004dcadb746fa5e24c5031ccfcf21320b0277457c98f02207a986d955c6e0cb35d446a89d3f56100f4d7f67801c31967743a9c8e10615bed01210349fc4e631e3624a545de3f89f5d8684c7b8138bd94bdd531d2e213bf016b278afeffffff02a135ef01000000001976a914bc3b654dca7e56b04dca18f2566cdaf02e8d9ada88ac99c39800000000001976a9141c4bc762dd5423e332166702cb75f40df79fea1288ac19430600");
        let (data, pre_tx_id) = TxHash::parse(&data[..]).unwrap();
        assert_eq!(
            pre_tx_id,
            TxHash(hex!(
                "813f79011acb80925dfe69b3def355fe914bd1d96a3f5f71bf8303c6a989c7d1"
            ))
        );

        let (data, pre_tx_index) = PreTxIndex::parse(&data[..]).unwrap();
        assert_eq!(pre_tx_index, PreTxIndex(0u32));

        let (data, script_sig) = ScriptSig::parse(&data[..]).unwrap();
        assert_eq!(script_sig.len, 107u64);
        assert_eq!(hex::encode(script_sig.content), "483045022100ed81ff192e75a3fd2304004dcadb746fa5e24c5031ccfcf21320b0277457c98f02207a986d955c6e0cb35d446a89d3f56100f4d7f67801c31967743a9c8e10615bed01210349fc4e631e3624a545de3f89f5d8684c7b8138bd94bdd531d2e213bf016b278a".to_string());

        let (data, seq) = TxInputSequence::parse(&data[..]).unwrap();
        assert_eq!(seq, TxInputSequence(0xfffffffeu32));
    }
}
