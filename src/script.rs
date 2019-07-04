use bytes::{BufMut, BytesMut};
use nom::bytes::streaming::take;
use nom::number::complete::{le_u16, le_u8};
use nom::IResult;

use crate::transaction::Varint;
use crate::wallet::{hash160, hash256, Hash256, Hex, S256Point, Signature};
use std::ops::{Add, Deref};

#[derive(Fail, Debug)]
pub enum ScriptError {
    #[fail(display = "parse hex script length error")]
    ParseLengthError,
    #[fail(display = "nom parse error")]
    NomParseError,
    #[fail(display = "serialize too long element error")]
    SerializeTooLongError,
    #[fail(display = "op code: {} evaluate error", _0)]
    OpCodeEvaluateError(u8),
}

type Stack = Vec<StackElement>;

#[derive(Debug, Clone)]
pub enum StackElement {
    DataElement(Vec<u8>),
    OpCode(OpCode),
}

impl Deref for StackElement {
    type Target = [u8];
    fn deref(&self) -> &Self::Target {
        match self {
            StackElement::DataElement(ref data) => &*data,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct OpCode {
    num: u8,
    kind: OpCodeKind,
}

#[derive(Debug, Clone)]
pub enum OpCodeKind {
    OpDup,
    OpHash256,
    OpHash160,
    OpCheckSig,
    Unknown,
}

impl OpCode {
    pub fn new(code: u8) -> Self {
        let kind = match code {
            0x76_u8 => OpCodeKind::OpDup,
            0xaa_u8 => OpCodeKind::OpHash256,
            0xa9_u8 => OpCodeKind::OpHash160,
            0xac_u8 => OpCodeKind::OpCheckSig,
            _ => OpCodeKind::Unknown,
        };
        OpCode { num: code, kind }
    }

    pub fn operation(&self) -> OperationType {
        match self.kind {
            OpCodeKind::OpDup => OperationType::Stack(Box::new(op_dup)),
            OpCodeKind::OpHash256 => OperationType::Stack(Box::new(op_hash256)),
            OpCodeKind::OpHash160 => OperationType::Stack(Box::new(op_hash160)),
            OpCodeKind::OpCheckSig => OperationType::StackSig(Box::new(op_check_sig)),
            OpCodeKind::Unknown => OperationType::Stack(Box::new(op_unknown)),
        }
    }
}

pub enum OperationType {
    Stack(Box<dyn Fn(&mut Stack) -> bool>),
    StackSig(Box<dyn Fn(&mut Stack, Hash256) -> bool>),
    StackStack(Box<dyn Fn(&mut Stack, &mut Stack) -> bool>),
}

fn op_dup(stack: &mut Stack) -> bool {
    if stack.len() < 1 {
        return false;
    }
    let last = stack.last().unwrap();
    match last {
        StackElement::DataElement(d) => {
            let d = (*d).clone();
            stack.push(StackElement::DataElement(d));
        }
        _ => unreachable!(),
    }
    true
}

fn op_hash256(stack: &mut Stack) -> bool {
    if stack.len() < 1 {
        return false;
    }

    let last = stack.last().unwrap();
    match last {
        StackElement::DataElement(d) => {
            let d = (*d).clone();
            let hash = hash256(&d[..]);
            stack.push(StackElement::DataElement(hash.to_vec()));
        }
        _ => unreachable!(),
    }
    true
}

fn op_hash160(stack: &mut Stack) -> bool {
    if stack.len() < 1 {
        return false;
    }

    let last = stack.last().unwrap();
    match last {
        StackElement::DataElement(d) => {
            let d = (*d).clone();
            let hash = hash160(&d[..]);
            stack.push(StackElement::DataElement(hash.to_vec()));
        }
        _ => unreachable!(),
    }
    true
}

fn op_unknown(stack: &mut Stack) -> bool {
    false
}

fn op_check_sig(stack: &mut Stack, hash: Hash256) -> bool {
    if stack.len() < 2 {
        return false;
    }
    let sec = stack.pop().expect("stack can not pop");
    let sig = stack.pop().expect("stack can not pop");

    let point = S256Point::parse_sec(&sec);
    let sig = Signature::parse_der(&sig);

    point.verify(hash, sig)
}

pub struct Script {
    cmds: Stack,
}

impl Script {
    pub fn new() -> Self {
        Script { cmds: Vec::new() }
    }

    pub fn push_opcode(&mut self, opcode: OpCode) {
        self.cmds.push(StackElement::OpCode(opcode))
    }

    pub fn push_data_ele(&mut self, data: &[u8]) {
        self.cmds.push(StackElement::DataElement(data.to_vec()))
    }

    // todo
    // How to chain the error of nom and failure
    pub fn parse(input: &[u8]) -> Result<(&[u8], Self), ScriptError> {
        let (input, (consumed_exactly_len, cmds)) =
            Self::nom_parse(input).or(Err(ScriptError::NomParseError))?;
        if !consumed_exactly_len {
            Err(ScriptError::ParseLengthError)
        } else {
            Ok((input, Script { cmds }))
        }
    }

    fn nom_parse(input: &[u8]) -> IResult<&[u8], (bool, Stack)> {
        let (input, length) = Varint::parse(input)?;
        let length = Into::<u64>::into(length) as usize;
        let mut cmds = Vec::new();
        let mut count = 0;

        let (mut outer_input, mut current) = (input, 0);
        while count < length {
            let (input, current) = le_u8(outer_input)?;
            count += 1;

            outer_input = if current >= 0x01 && current <= 0x4b {
                let (input, bytes) = take(current)(input)?;
                count += current as usize;
                cmds.push(StackElement::DataElement(bytes.to_vec()));
                input
            } else if current == 0x4c {
                // OP_PUSHDATA1
                let (input, data_len) = le_u8(input)?;
                count += 1;
                let (input, bytes) = take(data_len)(input)?;
                count += data_len as usize;
                cmds.push(StackElement::DataElement(bytes.to_vec()));
                input
            } else if current == 0x4d {
                // OP_PUSHDATA2
                let (input, data_len) = le_u16(input)?;
                count += 1;
                let (input, bytes) = take(data_len)(input)?;
                count += data_len as usize;
                cmds.push(StackElement::DataElement(bytes.to_vec()));
                input
            } else {
                let op_code = current;
                cmds.push(StackElement::OpCode(OpCode::new(op_code)));
                input
            };
        }

        Ok((input, (count == length, cmds)))
    }

    pub fn serialize(&self) -> Result<Vec<u8>, ScriptError> {
        let mut buf_len = 9usize + 9 + 4;
        for i in &self.cmds {
            match i {
                StackElement::OpCode(_) => buf_len += 1,
                StackElement::DataElement(data) => buf_len += 1 + 9 + data.len(),
            }
        }

        let mut buf = BytesMut::with_capacity(buf_len);
        for i in &self.cmds {
            match i {
                StackElement::OpCode(op_code) => buf.put_u8(op_code.num),
                StackElement::DataElement(data) => {
                    let len = data.len();
                    if len < 0x4b {
                        // less than 75 bytes
                        buf.put(Varint::encode(len as u64).unwrap());
                    } else if len > 75 && len < 0x100 {
                        buf.put_u8(0x4c);
                        buf.put(Varint::encode(len as u64).unwrap());
                    } else if len >= 0x100 && len <= 520 {
                        buf.put_u8(0x4d);
                        buf.put(Varint::encode(len as u64).unwrap());
                    } else {
                        return Err(ScriptError::SerializeTooLongError);
                    }
                    buf.put(data);
                }
            }
        }
        let mut raw_ret = buf.take().to_vec();
        buf.put(Varint::encode(raw_ret.len() as u64).unwrap());
        let mut ret = buf.take().to_vec();
        ret.append(&mut raw_ret);
        Ok(ret)
    }

    pub fn evaluate(&self, hash: Option<Hash256>) -> Result<bool, ScriptError> {
        let mut cmds = self.cmds.clone();
        let mut stack = Stack::new();
        let mut altstack = Stack::new();
        while cmds.len() > 0 {
            let cmd = cmds.remove(0);
            match cmd {
                StackElement::DataElement(d) => stack.push(StackElement::DataElement(d)),
                StackElement::OpCode(opcode) => {
                    let opcode_num = opcode.num;
                    let operation = opcode.operation();
                    if opcode_num >= 99 && opcode_num <= 100 {
                        match operation {
                            OperationType::StackStack(operation) => {
                                if !(*operation)(&mut stack, &mut cmds) {
                                    return Err(ScriptError::OpCodeEvaluateError(opcode_num));
                                }
                            }
                            _ => unreachable!(),
                        }
                    } else if opcode_num >= 107 && opcode_num <= 108 {
                        match operation {
                            OperationType::StackStack(operation) => {
                                if !(*operation)(&mut stack, &mut altstack) {
                                    return Err(ScriptError::OpCodeEvaluateError(opcode_num));
                                }
                            }
                            _ => unreachable!(),
                        }
                    } else if opcode_num >= 172 && opcode_num <= 175 {
                        match operation {
                            OperationType::StackSig(operation) => {
                                if !(*operation)(
                                    &mut stack,
                                    hash.expect("this op code need a hash256"),
                                ) {
                                    return Err(ScriptError::OpCodeEvaluateError(opcode_num));
                                }
                            }
                            _ => unreachable!(),
                        }
                    } else {
                        match operation {
                            OperationType::Stack(operation) => {
                                if !(*operation)(&mut stack) {
                                    return Err(ScriptError::OpCodeEvaluateError(opcode_num));
                                }
                            }
                            _ => unreachable!(),
                        }
                    }
                }
            }
        }

        if stack.is_empty() {
            return Ok(false);
        }
        if let Some(i) = stack.pop() {
            match i {
                StackElement::DataElement(data) => {
                    if data.is_empty() {
                        return Ok(false);
                    }
                }
                _ => {
                    return Ok(true);
                }
            }
        }
        Ok(true)
    }
}

impl Hex for StackElement {
    fn hex(&self) -> String {
        match self {
            StackElement::OpCode(op_code) => vec![op_code.num].hex(),
            StackElement::DataElement(datas) => datas.hex(),
        }
    }
}

impl Hex for Stack {
    fn hex(&self) -> String {
        let mut ret = String::new();
        for i in self {
            ret += &i.hex();
        }
        ret
    }
}

impl Hex for Script {
    fn hex(&self) -> String {
        self.cmds.hex()
    }
}

impl Add<&Self> for Script {
    type Output = Script;
    fn add(self, rhs: &Script) -> Self::Output {
        let mut cmds = self.cmds;
        let mut rhs_cmds = rhs.cmds.clone();
        cmds.append(&mut rhs_cmds);
        Script { cmds }
    }
}

impl Add<Self> for &Script {
    type Output = Script;
    fn add(self, rhs: &Script) -> Self::Output {
        let mut cmds = self.cmds.clone();
        let mut rhs_cmds = rhs.cmds.clone();
        cmds.append(&mut rhs_cmds);
        Script { cmds }
    }
}

mod test {
    use crate::script::StackElement;
    use crate::script::{OpCode, Script};
    use crate::wallet::{Hash256, Hex};

    #[test]
    fn test_script_parse() {
        let data = hex!("6a47304402207899531a52d59a6de200179928ca900254a36b8dff8bb75f5f5d71b1cdc26125022008b422690b8461cb52c3cc30330b23d574351872b7c361e9aae3649071c1a7160121035d5c93d9ac96881f19ba1f686f15f009ded7c62efe85a872e6a19b43c15a2937");
        let (_data, script) = Script::parse(&data[..]).unwrap();
        assert_eq!(
            script.cmds[0].hex(),
            "304402207899531a52d59a6de200179928ca900254a36b8dff8bb75f5f5d71b1cdc26125022008b422690b8461cb52c3cc30330b23d574351872b7c361e9aae3649071c1a71601".to_string()
        );
        assert_eq!(
            script.cmds[1].hex(),
            "035d5c93d9ac96881f19ba1f686f15f009ded7c62efe85a872e6a19b43c15a2937".to_string()
        );

        assert_eq!(
            script.hex(),
            "304402207899531a52d59a6de200179928ca900254a36b8dff8bb75f5f5d71b1cdc26125022008b422690b8461cb52c3cc30330b23d574351872b7c361e9aae3649071c1a71601035d5c93d9ac96881f19ba1f686f15f009ded7c62efe85a872e6a19b43c15a2937".to_string()
        );
    }
    #[test]
    fn test_script_serialize() {
        let data = hex!("6a47304402207899531a52d59a6de200179928ca900254a36b8dff8bb75f5f5d71b1cdc26125022008b422690b8461cb52c3cc30330b23d574351872b7c361e9aae3649071c1a7160121035d5c93d9ac96881f19ba1f686f15f009ded7c62efe85a872e6a19b43c15a2937");
        let (_data, script) = Script::parse(&data[..]).unwrap();

        assert_eq!(
            script.serialize().unwrap().hex(),
            "6a47304402207899531a52d59a6de200179928ca900254a36b8dff8bb75f5f5d71b1cdc26125022008b422690b8461cb52c3cc30330b23d574351872b7c361e9aae3649071c1a7160121035d5c93d9ac96881f19ba1f686f15f009ded7c62efe85a872e6a19b43c15a2937".to_string()
        );
    }

    #[test]
    fn test_script_evaluation() {
        let mut script_pubkey = Script::new();
        let data = hex!("04887387e452b8eacc4acfde10d9aaf7f6d9a0f975aabb10d006e4da568744d06c61de6d95231cd89026e286df3b6ae4a894a3378e393e93a0f45b666329a0ae34");
        script_pubkey.push_data_ele(&data);
        script_pubkey.push_opcode(OpCode::new(0xac));

        let mut script_sig = Script::new();
        let data = hex!("3045022000eff69ef2b1bd93a66ed5219add4fb51e11a840f404876325a1e8ffe0529a2c022100c7207fee197d27c618aea621406f6bf5ef6fca38681d82b2f06fddbdce6feab601");
        script_sig.push_data_ele(&data);

        let combined_script = script_sig + &script_pubkey;

        let hash = hex!("7c076ff316692a3d7eb3c3bb0f8b1488cf72e1afcd929e29307032997a838a3d");
        let hash = Hash256::new(&hash);
        //        assert!(combined_script.evaluate(Some(hash)).unwrap());
    }
}
