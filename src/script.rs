use nom::bytes::streaming::take;
use nom::number::complete::{le_u16, le_u8};

use crate::transaction::Varint;
use crate::wallet::{hash160, hash256};
use nom::IResult;

trait Op {
    fn execute(&self, stack: &mut Stack) -> bool;
}

#[derive(Fail, Debug)]
enum ScriptError {
    #[fail(display = "parse hex script length error")]
    ParseLengthError,
    #[fail(display = "nom parse error")]
    NomParseError,
}

type Stack = Vec<StackElement>;

#[derive(Debug, Clone)]
enum StackElement {
    OpCode(OpCode),
    DataElement(Vec<u8>),
}

#[derive(Debug, Clone)]
enum OpCode {
    OpDup,
    OpHash256,
    OpHash160,
    Unknown,
}

impl OpCode {
    pub fn new(code: u8) -> Self {
        match code {
            0x76_u8 => OpCode::OpDup,
            0xaa_u8 => OpCode::OpHash256,
            0xa9_u8 => OpCode::OpHash160,
            _ => OpCode::Unknown,
        }
    }
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
            stack.push(StackElement::DataElement(hash));
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
            stack.push(StackElement::DataElement(hash));
        }
        _ => unreachable!(),
    }
    true
}

pub struct Script {
    cmds: Stack,
}

impl Script {
    // todo
    // How to chain the error of nom and failure
    fn parse(input: &[u8]) -> Result<(&[u8], Self), ScriptError> {
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

        while count < length {
            let (input, current) = le_u8(input)?;
            count += 1;

            if current >= 0x01 && current <= 0x4b {
                let (input, bytes) = take(current)(input)?;
                count += current as usize;
                cmds.push(StackElement::DataElement(bytes.to_vec()));
            } else if current == 0x4c {
                // OP_PUSHDATA1
                let (input, data_len) = le_u8(input)?;
                count += 1;
                let (input, bytes) = take(data_len)(input)?;
                count += data_len as usize;
                cmds.push(StackElement::DataElement(bytes.to_vec()));
            } else if current == 0x4d {
                // OP_PUSHDATA2
                let (input, data_len) = le_u16(input)?;
                count += 1;
                let (input, bytes) = take(data_len)(input)?;
                count += data_len as usize;
                cmds.push(StackElement::DataElement(bytes.to_vec()));
            } else {
                let op_code = current;
                cmds.push(StackElement::OpCode(OpCode::new(op_code)));
            }
        }

        Ok((input, (count == length, cmds)))
    }
}

mod teset {
    #[test]
    fn test_script() {}
}
