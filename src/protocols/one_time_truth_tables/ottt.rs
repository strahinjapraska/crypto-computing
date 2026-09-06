use crate::protocols::one_time_truth_tables::dealer::{Dealer, SIZE};
use std::{fmt::Error, io::ErrorKind::OutOfMemory};

type BitMatrix = [[bool; SIZE]; SIZE];

#[derive(Clone, Copy)]
pub(crate) enum Role {
    Alice = 0,
    Bob = 1,
}

pub struct Round1Msg {
    pub u: usize,
}

pub struct Round1 {
    pub role: Role,
    pub input: usize,
    pub shift: usize,
    pub matrix: BitMatrix,
}

pub struct Round2 {
    pub role: Role,
    pub input: usize,
    pub shift: usize,
    pub matrix: BitMatrix,
    pub u: usize,
}

pub struct Round2Msg {
    pub u: usize,
}

pub struct Round3 {
    pub role: Role,
    pub input: usize,
    pub shift: usize,
    pub matrix: BitMatrix,
    pub u: usize,
    pub v: usize,
    pub z_b: bool,
}

pub struct Round3Msg {
    pub v: usize,
    pub z_b: bool,
}

impl Round1 {
    pub fn new(role: Role, input: usize, shift: usize, matrix: BitMatrix) -> (Self, Option<Round1Msg>) {
        return (
            Round1 {
                role,
                input,
                shift,
                matrix,
            },
            None,
        );
    }

    pub fn proceed(self, msg: Option<Round1Msg>) -> Result<(Round2, Option<Round2Msg>), Error> {
        match self.role {
            Role::Alice => {
                let u = (self.input + self.shift) % (2 << SIZE);
                return Ok((
                    Round2 {
                        role: self.role,
                        input: self.input,
                        shift: self.shift,
                        matrix: self.matrix,
                        u,
                    },
                    Some(Round2Msg { u }),
                ));
            }
            Role::Bob => {
                return Ok((
                    Round2 {
                        role: self.role,
                        input: self.input,
                        shift: self.shift,
                        matrix: self.matrix,
                        u: 0,
                    },
                    None,
                ));
            }
        }
    }
}

impl Round2 {
    pub fn proceed(mut self, msg: Option<Round2Msg>) -> Result<(Round3, Option<Round3Msg>), Error> {
        match self.role {
            Role::Alice => {
                return Ok((
                    Round3 {
                        role: self.role,
                        input: self.input,
                        shift: self.shift,
                        matrix: self.matrix,
                        u: self.u,
                        v: 0,
                        z_b: false,
                    },
                    Some(Round3Msg { v: 0, z_b: false }),
                ));
            }
            Role::Bob => {
                let msgu = msg.unwrap();
                self.u = msgu.u;
                let v = (self.input + self.shift) % (2 << SIZE);
                let z_b = self.matrix[self.u as usize][v as usize];

                return Ok((
                    Round3 {
                        role: self.role,
                        input: self.input,
                        shift: self.shift,
                        matrix: self.matrix,
                        u: self.u,
                        v,
                        z_b,
                    },
                    Some(Round3Msg { v, z_b }),
                ));
            }
        }
    }
}

impl Round3 {
    pub fn proceed(mut self, msg: Option<Round3Msg>) -> Result<(bool), Error> {
        match self.role {
            Role::Alice => {
                let msgu = msg.unwrap();
                self.v = msgu.v;
                self.z_b = msgu.z_b;
                let z = self.matrix[self.u as usize][self.v as usize] ^ self.z_b;

                return Ok(z);
            }
            Role::Bob => Ok(false),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::protocols::one_time_truth_tables::{
        dealer,
        ottt::{
            Role::{Alice, Bob},
            Round1,
        },
    };

    #[test]
    pub fn test_ottt() {
        let mut rng = rand::rng();
        let mut dealer = dealer::Dealer::new(&mut rng);
        let alice_pair = dealer.query_alice().expect("alice");
        let bob_pair = dealer.query_bob().expect("bob");
        let alice_input = 3;
        let bob_input = 1;

        let (a1, a_msg1) = Round1::new(Alice, alice_input, alice_pair.0 as usize, alice_pair.1);
        let (b1, b_msg1) = Round1::new(Bob, bob_input, bob_pair.0 as usize, bob_pair.1);

        let (a2, a_msg2) = a1.proceed(None).unwrap();
        let (b2, b_msg2) = b1.proceed(None).unwrap();

        let (a3, a_msg3) = a2.proceed(b_msg2).unwrap();
        let (b3, b_msg3) = b2.proceed(a_msg2).unwrap();

        let a_result = a3.proceed(b_msg3).unwrap();
        let b_result = b3.proceed(a_msg3).unwrap();
    }
}
