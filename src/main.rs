#[warn(dead_code)]
#[macro_use]
extern crate hex_literal;
#[macro_use]
extern crate uint;
#[macro_use]
extern crate failure;

mod script;
mod transaction;
mod wallet;

fn main() {
    println!("Hello, world!");
}
