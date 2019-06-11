use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct TxIn;

#[derive(Serialize, Deserialize, Debug)]
struct TxOut;

#[derive(Serialize, Deserialize, Debug)]
struct Tx {
    version: u32,
    tx_ins: Vec<TxIn>,
    tx_outs: Vec<TxOut>,
    locktime: u32,
    testnet: bool,
}

impl Tx {
    fn new(
        version: u32,
        tx_ins: Vec<TxIn>,
        tx_outs: Vec<TxOut>,
        locktime: u32,
        testnet: bool,
    ) -> Self {
        Tx {
            version,
            tx_ins,
            tx_outs,
            locktime,
            testnet,
        }
    }
    fn version(&self) -> u32 {
        self.version
    }

    //    fn id(&self) -> [u8; 32] {
    //        self.hash().hex()
    //    }
    //
    //    fn hash(&self) {
    //
    //    }
}

mod test {
    use super::Tx;

    #[test]
    fn test_version() {}
}
