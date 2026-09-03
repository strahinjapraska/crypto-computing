#[derive(Debug, Clone, Copy)]
#[repr(usize)]
pub enum BloodType {
    ONeg = 0,
    OPos,
    ANeg,
    APos,
    BNeg,
    BPos,
    ABNeg,
    ABPos,
}

pub struct EncodedBlood {
    pub a: bool,
    pub b: bool,
    pub rh: bool,
}

pub fn encode(blood_type: BloodType) -> EncodedBlood {
    match blood_type {
        BloodType::ONeg  => EncodedBlood { a: false, b: false, rh: false },
        BloodType::OPos  => EncodedBlood { a: false, b: false, rh: true  },
        BloodType::ANeg  => EncodedBlood { a: true,  b: false, rh: false },
        BloodType::APos  => EncodedBlood { a: true,  b: false, rh: true  },
        BloodType::BNeg  => EncodedBlood { a: false, b: true,  rh: false },
        BloodType::BPos  => EncodedBlood { a: false, b: true,  rh: true  },
        BloodType::ABNeg => EncodedBlood { a: true,  b: true,  rh: false },
        BloodType::ABPos => EncodedBlood { a: true,  b: true,  rh: true  },
    }

}