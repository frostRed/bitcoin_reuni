mod locktime;
mod tx_input;
mod tx_output;
mod tx_version;
mod varint;

use crate::wallet::hash256;

use bytes::{BufMut, BytesMut};
use itertools::Itertools;
use nom::IResult;

use locktime::TxLocktime;
use nom::multi::count;
use tx_input::TxInput;
use tx_output::TxOutput;
use tx_version::TxVersion;
use varint::Varint;

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

    pub fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        let (input, tx_version) = TxVersion::parse(&input[..])?;

        let (input, inputs_num) = Varint::parse(&input[..])?;
        let input_num = Into::<u64>::into(inputs_num) as usize;
        let (input, tx_inputs): (&[u8], Vec<TxInput>) = count(TxInput::parse, input_num)(&input)?;

        let (input, output_num) = Varint::parse(&input[..])?;
        let output_num = Into::<u64>::into(output_num) as usize;
        let (input, tx_outputs): (&[u8], Vec<TxOutput>) =
            count(TxOutput::parse, output_num)(&input)?;

        let (input, locktime) = TxLocktime::parse(&input[..])?;
        Ok((
            input,
            Transaction::new(tx_version, tx_inputs, tx_outputs, locktime, false),
        ))
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
        let mut inputs: Vec<Vec<u8>> = Vec::with_capacity(self.inputs.len());
        let mut inputs_len = 0;
        let mut outputs: Vec<Vec<u8>> = Vec::with_capacity(self.outputs.len());
        let mut outputs_len = 0;

        self.inputs.iter().for_each(|i| {
            let bytes = i.serialize();
            inputs_len += bytes.len();
            inputs.push(bytes);
        });

        self.outputs.iter().for_each(|i| {
            let bytes = i.serialize();
            outputs_len += bytes.len();
            outputs.push(bytes);
        });

        let mut buf = BytesMut::with_capacity(4 + 9 + inputs_len + 9 + outputs_len + 4 + 4);

        buf.put_u32_le(u32::from(self.version));

        buf.put(Varint::encode(self.inputs.len() as u64).unwrap());
        inputs.into_iter().for_each(|i: Vec<u8>| buf.put(&i));

        buf.put(Varint::encode(self.outputs.len() as u64).unwrap());
        outputs.into_iter().for_each(|i: Vec<u8>| buf.put(&i));

        buf.put_u32_le(u32::from(self.locktime));

        buf.take().to_vec()
    }
}

mod test {
    use super::locktime::TxLocktime;
    use super::tx_version::TxVersion;
    use super::Transaction;

    #[test]
    fn test_tx() {
        let data = hex!("0100000001813f79011acb80925dfe69b3def355fe914bd1d96a3f5f71bf8303c6a989c7d1000000006b483045022100ed81ff192e75a3fd2304004dcadb746fa5e24c5031ccfcf21320b0277457c98f02207a986d955c6e0cb35d446a89d3f56100f4d7f67801c31967743a9c8e10615bed01210349fc4e631e3624a545de3f89f5d8684c7b8138bd94bdd531d2e213bf016b278afeffffff02a135ef01000000001976a914bc3b654dca7e56b04dca18f2566cdaf02e8d9ada88ac99c39800000000001976a9141c4bc762dd5423e332166702cb75f40df79fea1288ac19430600");

        let (_data, tx) = Transaction::parse(&data[..]).unwrap();

        assert_eq!(TxVersion::new(1u32), tx.version);

        assert_eq!(1, tx.inputs.len());
        assert_eq!(
            format!("{}", tx.inputs[0]),
            "d1c789a9c60383bf715f3f6ad9d14b91fe55f3deb369fe5d9280cb1a01793f81:0".to_string()
        );

        assert_eq!(2, tx.outputs.len());
        assert_eq!(
            format!("{}", tx.outputs[0]),
            "32454049:76a914bc3b654dca7e56b04dca18f2566cdaf02e8d9ada88ac".to_string()
        );
        assert_eq!(
            format!("{}", tx.outputs[1]),
            "10011545:76a9141c4bc762dd5423e332166702cb75f40df79fea1288ac".to_string()
        );

        assert_eq!(tx.locktime, TxLocktime::new(410393));

        assert_eq!(
            hex::encode(tx.serialize()),
            "0100000001813f79011acb80925dfe69b3def355fe914bd1d96a3f5f71bf8303c6a989c7d1000000006b483045022100ed81ff192e75a3fd2304004dcadb746fa5e24c5031ccfcf21320b0277457c98f02207a986d955c6e0cb35d446a89d3f56100f4d7f67801c31967743a9c8e10615bed01210349fc4e631e3624a545de3f89f5d8684c7b8138bd94bdd531d2e213bf016b278afeffffff02a135ef01000000001976a914bc3b654dca7e56b04dca18f2566cdaf02e8d9ada88ac99c39800000000001976a9141c4bc762dd5423e332166702cb75f40df79fea1288ac19430600".to_string()
        );
    }
}
