use std::ops::Deref;

use super::op_function::{op_check_sig, op_dup, op_hash160, op_hash256, op_unknown, Stack};
use crate::wallet::{Hash256, Hex};

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

    pub fn num(&self) -> u8 {
        self.num
    }
}

pub enum OperationType {
    Stack(Box<dyn Fn(&mut Stack) -> bool>),
    StackSig(Box<dyn Fn(&mut Stack, Hash256) -> bool>),
    StackStack(Box<dyn Fn(&mut Stack, &mut Stack) -> bool>),
}

impl Hex for StackElement {
    fn hex(&self) -> String {
        match self {
            StackElement::OpCode(op_code) => vec![op_code.num()].hex(),
            StackElement::DataElement(datas) => datas.hex(),
        }
    }
}
