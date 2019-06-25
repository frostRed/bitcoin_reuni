struct Transaction {
    version: TxVersion,
    inputs: Vec<TxInput>,
    outputs: Vec<TxOutput>,
    locktime: TxLocktime,
    testnet: bool,
}

impl Transaction {}

struct TxVersion;
struct TxInput;
struct TxOutput;
struct TxLocktime;

mod test {
    #[test]
    fn test_() {
        assert_eq!(1, 1)
    }
}
