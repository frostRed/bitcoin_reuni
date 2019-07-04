mod pre_tx_index;
mod script_sig;
mod tx_hash;
mod tx_input_sequence;

use bytes::{BufMut, BytesMut};
use nom::IResult;
use std::fmt::Display;

use super::tx_fetcher::TxFetcher;
use super::tx_output::ScriptPubKey;
use super::tx_output::TxOutputAmount;
use super::Transaction;
use crate::wallet::Hex;
pub use pre_tx_index::PreTxIndex;
pub use script_sig::ScriptSig;
pub use tx_hash::TxHash;
pub use tx_input_sequence::TxInputSequence;

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
        buf.put(&self.pre_tx_id.to_little_endian());
        buf.put_u32_le(self.pre_tx_index.index());
        buf.put(&self.script_sig.serialize());
        buf.put_u32_le(self.sequence.sequence());
        buf.take().to_vec()
    }

    pub fn fetch_tx<'a>(
        &'a self,
        fetcher: &'a mut TxFetcher,
        testnet: bool,
    ) -> Result<&'a Transaction, failure::Error> {
        fetcher.fetch(self.pre_tx_id, testnet, false)
    }

    pub fn value(&self, fetcher: &mut TxFetcher, testnet: bool) -> TxOutputAmount {
        let tx = self
            .fetch_tx(fetcher, testnet)
            .expect("get pre transaction failed");
        tx.outputs[u32::from(self.pre_tx_index) as usize].amount
    }

    pub fn script_pubkey<'a>(
        &'a self,
        fetcher: &'a mut TxFetcher,
        testnet: bool,
    ) -> &'a ScriptPubKey {
        let tx = self
            .fetch_tx(fetcher, testnet)
            .expect("get pre transaction failed");
        &tx.outputs[u32::from(self.pre_tx_index) as usize].script_pub_key
    }
}

impl Display for TxInput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.pre_tx_id, self.pre_tx_index)
    }
}

impl Hex for TxInput {
    fn hex(&self) -> String {
        hex::encode(self.serialize())
    }
}

mod test {
    use super::super::super::wallet::Hex;
    use super::{PreTxIndex, ScriptSig, TxHash, TxInput, TxInputSequence};
    use std::str::FromStr;

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
            TxHash::from_str("d1c789a9c60383bf715f3f6ad9d14b91fe55f3deb369fe5d9280cb1a01793f81")
                .unwrap()
        );

        let (data, pre_tx_index) = PreTxIndex::parse(&data[..]).unwrap();
        assert_eq!(pre_tx_index, PreTxIndex::new(0u32));

        let (data, script_sig) = ScriptSig::parse(&data[..]).unwrap();
        assert_eq!(script_sig.content.len(), 107usize);
        assert_eq!(hex::encode(&script_sig.content), "483045022100ed81ff192e75a3fd2304004dcadb746fa5e24c5031ccfcf21320b0277457c98f02207a986d955c6e0cb35d446a89d3f56100f4d7f67801c31967743a9c8e10615bed01210349fc4e631e3624a545de3f89f5d8684c7b8138bd94bdd531d2e213bf016b278a".to_string());

        let (_data, seq) = TxInputSequence::parse(&data[..]).unwrap();
        assert_eq!(seq, TxInputSequence::new(0xfffffffeu32));

        let tx_input = TxInput::new(pre_tx_id, pre_tx_index, script_sig, seq);
        assert_eq!(
            format!("{}", tx_input),
            "d1c789a9c60383bf715f3f6ad9d14b91fe55f3deb369fe5d9280cb1a01793f81:0".to_string()
        );

        assert_eq!(tx_input.hex(), "813f79011acb80925dfe69b3def355fe914bd1d96a3f5f71bf8303c6a989c7d1000000006b483045022100ed81ff192e75a3fd2304004dcadb746fa5e24c5031ccfcf21320b0277457c98f02207a986d955c6e0cb35d446a89d3f56100f4d7f67801c31967743a9c8e10615bed01210349fc4e631e3624a545de3f89f5d8684c7b8138bd94bdd531d2e213bf016b278afeffffff".to_string());
    }
}
