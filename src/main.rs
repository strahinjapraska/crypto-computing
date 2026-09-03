use crate::encoding::BloodType::{ABPos, ONeg};

pub mod encoding;
pub mod functionality;

fn main() {
    println!("Hello, world!");
    let res = functionality::circuit_compatiblity(ONeg, ABPos); 
    println!("{}", res);
}
