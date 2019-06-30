use std::collections::HashMap;

use super::tx_input::TxHash;
use super::Transaction;
use std::error::Error;

pub struct TxFetcher {
    cache: HashMap<TxHash, Transaction>,
}

impl TxFetcher {
    fn get_url(testnet: bool) -> &'static str {
        "https://blockchain.info"
    }

    pub fn fetch(&mut self, tx_id: TxHash, testnet: bool, fresh: bool) -> Transaction {
        // todo Error idiom
        let url = format!("{}/tx/{}?format=hex", Self::get_url(testnet), tx_id);
        //        let body = reqwest::get("https://www.rust-lang.org")?.text()?;
        let body = reqwest::get(&url)
            .expect("send Get failed")
            .text()
            .expect("get response text failed");
        let hex = hex::decode(body).expect("hex response decode failed");
        let (input, tx) = Transaction::parse(&hex).expect("Transaction parse failed");
        tx
    }

    pub fn new() -> Self {
        TxFetcher {
            cache: HashMap::new(),
        }
    }
}

mod test {
    use super::super::tx_fetcher::TxFetcher;
    use super::super::tx_input::TxHash;
    use crate::transaction::Transaction;

    #[test]
    fn test_tx_fetch() {
        let data = hex!("0100000002d8c8df6a6fdd2addaf589a83d860f18b44872d13ee6ec3526b2b470d42a96d4d000000008b483045022100b31557e47191936cb14e013fb421b1860b5e4fd5d2bc5ec1938f4ffb1651dc8902202661c2920771fd29dd91cd4100cefb971269836da4914d970d333861819265ba014104c54f8ea9507f31a05ae325616e3024bd9878cb0a5dff780444002d731577be4e2e69c663ff2da922902a4454841aa1754c1b6292ad7d317150308d8cce0ad7abffffffff2ab3fa4f68a512266134085d3260b94d3b6cfd351450cff021c045a69ba120b2000000008b4830450220230110bc99ef311f1f8bda9d0d968bfe5dfa4af171adbef9ef71678d658823bf022100f956d4fcfa0995a578d84e7e913f9bb1cf5b5be1440bcede07bce9cd5b38115d014104c6ec27cffce0823c3fecb162dbd576c88dd7cda0b7b32b0961188a392b488c94ca174d833ee6a9b71c0996620ae71e799fc7c77901db147fa7d97732e49c8226ffffffff02c0175302000000001976a914a3d89c53bb956f08917b44d113c6b2bcbe0c29b788acc01c3d09000000001976a91408338e1d5e26db3fce21b011795b1c3c8a5a5d0788ac00000000");
        let mut tx_fetcher = TxFetcher::new();
        let tx = tx_fetcher.fetch(
            TxHash::new(&hex!(
                "9021b49d445c719106c95d561b9c3fac7bcb3650db67684a9226cd7fa1e1c1a0"
            ))
            .unwrap()
            .1,
            false,
            false,
        );
        assert_eq!(
            hex::encode(tx.serialize()),
            "0100000002d8c8df6a6fdd2addaf589a83d860f18b44872d13ee6ec3526b2b470d42a96d4d000000008b483045022100b31557e47191936cb14e013fb421b1860b5e4fd5d2bc5ec1938f4ffb1651dc8902202661c2920771fd29dd91cd4100cefb971269836da4914d970d333861819265ba014104c54f8ea9507f31a05ae325616e3024bd9878cb0a5dff780444002d731577be4e2e69c663ff2da922902a4454841aa1754c1b6292ad7d317150308d8cce0ad7abffffffff2ab3fa4f68a512266134085d3260b94d3b6cfd351450cff021c045a69ba120b2000000008b4830450220230110bc99ef311f1f8bda9d0d968bfe5dfa4af171adbef9ef71678d658823bf022100f956d4fcfa0995a578d84e7e913f9bb1cf5b5be1440bcede07bce9cd5b38115d014104c6ec27cffce0823c3fecb162dbd576c88dd7cda0b7b32b0961188a392b488c94ca174d833ee6a9b71c0996620ae71e799fc7c77901db147fa7d97732e49c8226ffffffff02c0175302000000001976a914a3d89c53bb956f08917b44d113c6b2bcbe0c29b788acc01c3d09000000001976a91408338e1d5e26db3fce21b011795b1c3c8a5a5d0788ac00000000".to_string()
        );
    }

}
