mod tx_input;
mod tx_version;
mod varint;

use crate::wallet::hash256;
use nom::number::complete::{le_u16, le_u32, le_u64, le_u8};
use nom::{
    bytes::complete::{tag, take_while_m_n},
    combinator::map_res,
    sequence::tuple,
    IResult,
};

use std::error::Error;
use std::fmt::Display;

use tx_input::TxInput;
use tx_version::TxVersion;
use varint::Varint;

struct Transaction {
    version: TxVersion,
    inputs_num: Varint,
    inputs: Vec<TxInput>,
    outputs: Vec<TxOutput>,
    locktime: TxLocktime,
    testnet: bool,
}

impl Transaction {
    pub fn new(
        version: TxVersion,
        inputs_num: Varint,
        inputs: Vec<TxInput>,
        outputs: Vec<TxOutput>,
        locktime: TxLocktime,
        testnet: bool,
    ) -> Self {
        Transaction {
            version,
            inputs_num,
            inputs,
            outputs,
            locktime,
            testnet,
        }
    }

    pub fn parse(data: &[u8]) -> Self {
        unimplemented!()
    }

    pub fn id(&self) -> String {
        self.hash();
        unimplemented!()
    }

    fn hash(&self) -> Vec<u8> {
        hash256(
            &self
                .serialize()
                .iter()
                .rev()
                .map(|i| *i)
                .collect::<Vec<u8>>(),
        )
    }

    fn serialize(&self) -> Vec<u8> {
        unimplemented!()
    }
}

struct TxOutput;
struct TxLocktime;

mod test {
    use crate::transaction::tx_input::TxInput;
    use crate::transaction::tx_version::TxVersion;
    use crate::transaction::varint::Varint;
    use nom::IResult;

    #[test]
    fn test_tx() {
        let data = hex!("0100000001813f79011acb80925dfe69b3def355fe914bd1d96a3f5f71bf8303c6a989c7d1000000006b483045022100ed81ff192e75a3fd2304004dcadb746fa5e24c5031ccfcf21320b0277457c98f02207a986d955c6e0cb35d446a89d3f56100f4d7f67801c31967743a9c8e10615bed01210349fc4e631e3624a545de3f89f5d8684c7b8138bd94bdd531d2e213bf016b278afeffffff02a135ef01000000001976a914bc3b654dca7e56b04dca18f2566cdaf02e8d9ada88ac99c39800000000001976a9141c4bc762dd5423e332166702cb75f40df79fea1288ac19430600");
        let (data, tx_version) = TxVersion::parse(&data[..]).unwrap();
        let (data, inputs_num) = Varint::parse(&data[..]).unwrap();
        let num = Into::<u64>::into(inputs_num) as usize;
        let mut tx_inputs = Vec::with_capacity(num);
        for i in 0..num {
            let (input, tx_input) = TxInput::parse(&data[..]).unwrap();
            tx_inputs.push(tx_input);
        }
    }
}
