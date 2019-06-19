use super::ec::utils::U256;
use std::fmt::Display;

pub struct Signature {
    pub r: U256,
    pub s: U256,
}

impl Display for Signature {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Signature({}, {})", self.r, self.s)
    }
}

impl Signature {
    pub fn new(r: U256, s: U256) -> Self {
        Signature { r, s }
    }

    pub fn der(&self) -> Vec<u8> {
        unimplemented!()
    }
}
