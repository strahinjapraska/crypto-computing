use crate::encoding::{BloodType, encode};

const TABLE: [[bool; 8]; 8] = [
    [true,  false, false, false, false, false, false, false], 
    [true,  true,  false, false, false, false, false, false], 
    [true,  false, true,  false, false, false, false, false], 
    [true,  true,  true,  true,  false, false, false, false], 
    [true,  false, false, false, true,  false, false, false], 
    [true,  true,  false, false, true,  true,  false, false], 
    [true,  false, true,  false, true,  false, true,  false], 
    [true,  true,  true,  true,  true,  true,  true,  true ], 
];

pub fn look_up_table_compatibility(donor: BloodType, recipient: BloodType) -> bool{
    TABLE[recipient as usize][donor as usize]
}

pub fn circuit_compatiblity(donor: BloodType, recipient: BloodType) -> bool{
    let d = encode(donor);
    let r = encode(recipient);

    (!d.a || r.a)
    && (!d.b || r.b)
    && (!d.rh || r.rh)
}