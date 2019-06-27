pub mod tx_version;
pub mod varint;

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

use tx_version::TxVersion;

struct Transaction {
    version: TxVersion,
    inputs: Vec<TxInput>,
    outputs: Vec<TxOutput>,
    locktime: TxLocktime,
    testnet: bool,
}

impl Transaction {
    pub fn new(
        version: TxVersion,
        inputs: Vec<TxInput>,
        outputs: Vec<TxOutput>,
        locktime: TxLocktime,
        testnet: bool,
    ) -> Self {
        Transaction {
            version,
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

struct TxInput;
struct TxOutput;
struct TxLocktime;
