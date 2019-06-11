mod ec;

use num_bigint::BigUint;
use num_traits::{One, Zero};

fn main() {
    let a = BigUint::from(2usize);
    let b = BigUint::from(50usize);
    let c = BigUint::from(119usize);
    let d = BigUint::from(1usize);
    println!("{:?}", a.modpow(&b, &c));
    println!("{:?}", &a + &b);
}
