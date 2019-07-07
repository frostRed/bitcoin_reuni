use super::stack_element::StackElement;
use crate::wallet::{hash160, hash256, Hash256, Hex, S256Point, Signature};

pub type Stack = Vec<StackElement>;

impl Hex for Stack {
    fn hex(&self) -> String {
        let mut ret = String::new();
        for i in self {
            ret += &i.hex();
        }
        ret
    }
}

pub fn op_dup(stack: &mut Stack) -> bool {
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

pub fn op_hash256(stack: &mut Stack) -> bool {
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

pub fn op_hash160(stack: &mut Stack) -> bool {
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

pub fn op_unknown(stack: &mut Stack) -> bool {
    false
}

pub fn op_check_sig(stack: &mut Stack, hash: Hash256) -> bool {
    if stack.len() < 2 {
        return false;
    }
    let sec = stack.pop().expect("stack can not pop");

    let sig = stack.pop().expect("stack can not pop");

    let point = S256Point::parse_sec(&sec);
    let sig = Signature::parse_der(&sig[0..(sig.len() - 1)]);

    if point.verify(hash, sig) {
        stack.push(StackElement::DataElement(encode_num(1)));
    } else {
        stack.push(StackElement::DataElement(encode_num(0)));
    }
    true
}

fn encode_num(num: i8) -> Vec<u8> {
    if num == 0 {
        return vec![];
    }
    let mut abs_num = num.abs() as u8;
    let negative = if num < 0 { true } else { false };

    let mut result = vec![];
    while abs_num != 0 {
        result.push(abs_num & 0xff);
        abs_num = abs_num.checked_shr(8).unwrap_or(0);
    }

    if let Some(last) = result.last_mut() {
        if *last & 0x80 > 0 {
            if negative {
                result.push(0x80_u8);
            } else {
                result.push(0);
            }
        } else if negative {
            *last |= 0x80;
        }
    }
    result
}
