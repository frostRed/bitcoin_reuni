mod transaction;
mod wallet;
#[macro_use]
extern crate uint;

fn main() {
    println!("Hello, world!");
}

//construct_uint! {
//    pub struct U256(4);
//}
//
//construct_uint! {
//    pub struct U512(8);
//}

//#[test]
//fn u128_conversions() {
//    let mut a = U256::from(u128::max_value());
//    assert_eq!(a.low_u128(), u128::max_value());
//    a += 2u128.into();
//    assert_eq!(a.low_u128(), 1u128);
//    a -= 3u128.into();
//    assert_eq!(a.low_u128(), u128::max_value() - 1);
//}
