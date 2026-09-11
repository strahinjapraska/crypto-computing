use crate::core::encoding::{BloodType, encode};

pub const TABLE: [[bool; 8]; 8] = [
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

pub const BLOOD_TYPES: [BloodType; 8] = [
            BloodType::ONeg,
            BloodType::OPos,
            BloodType::ANeg,
            BloodType::APos,
            BloodType::BNeg,
            BloodType::BPos,
            BloodType::ABNeg,
            BloodType::ABPos,
        ];

#[cfg(test)]
mod tests {
use super::*;

    #[test]
    fn test_compatibility() {
        BLOOD_TYPES.iter().for_each(|donor| {
            BLOOD_TYPES.iter().for_each(|recipient| {
                let table_result = look_up_table_compatibility(*donor, *recipient);
                let circuit_result = circuit_compatiblity(*donor, *recipient);
                assert_eq!(table_result, circuit_result, "mismatch for donor: {:?}, recipient: {:?}", donor, recipient);
            });
        });
    }
}