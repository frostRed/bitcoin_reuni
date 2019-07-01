#[warn(dead_code)]
#[macro_use]
extern crate hex_literal;
#[macro_use]
extern crate uint;
#[macro_use]
extern crate failure;

mod transaction;
mod wallet;

use bytes::{BigEndian, BufMut, BytesMut};

fn main() {
    println!("Hello, world!");
}
