use std::sync::mpsc::{Receiver, Sender, channel};

use num_bigint::BigUint;
use rand::{Rng, rngs::ThreadRng};

use crate::{
    core::encoding::{BloodType, EncodedBlood, encode},
    protocols::homomorphic_encryption::encryption::{FHEncryption, generate_key_pair},
};

pub struct Channel {
    pub incoming: Receiver<BigUint>,
    pub outgoing: Sender<BigUint>,
}
impl Channel {
    pub fn send(&self, value: BigUint) {
        self.outgoing.send(value).unwrap();
    }
    pub fn receive(&self) -> BigUint {
        return self.incoming.recv().unwrap();
    }
}

pub struct Bob {
    encoded_blood: EncodedBlood,
    channel: Channel, // encrypted_encoded_blood: EncryptedEncodedBlood
}

impl Bob {
    pub fn new(channel: Channel) -> Self {
        return Self {
            channel,
            encoded_blood: encode(BloodType::ABNeg), // placeholder
        };
    }

    pub fn set_blood_type(&mut self, new_blood_type: BloodType) -> () {
        self.encoded_blood = encode(new_blood_type);
    }

    // (!d.a || r.a) == (!d.a && r.a) XOR (!d.a && r.a)
    fn sub_eval(first: &BigUint, second: &BigUint) -> BigUint {
        return ((first + BigUint::ONE) * second) + ((first + BigUint::ONE) + second);
    }

    // eval and send back the result
    pub fn eval(&self) -> () {
        let enncrypted_alice_a = self.channel.receive();
        let enncrypted_alice_b = self.channel.receive();
        let enncrypted_alice_rh = self.channel.receive();

        let bob_a = if self.encoded_blood.a {
            BigUint::ONE
        } else {
            BigUint::ZERO
        };
        let bob_b = if self.encoded_blood.b {
            BigUint::ONE
        } else {
            BigUint::ZERO
        };
        let bob_rh = if self.encoded_blood.rh {
            BigUint::ONE
        } else {
            BigUint::ZERO
        };

        let res_a = Bob::sub_eval(&enncrypted_alice_a, &bob_a);
        let res_b = Bob::sub_eval(&enncrypted_alice_b, &bob_b);
        let res_rh = Bob::sub_eval(&enncrypted_alice_rh, &bob_rh);
        let final_result = res_a * res_b * res_rh;
        self.channel.send(final_result);
    }
}

pub struct Alice<R: Rng> {
    encoded_blood: EncodedBlood,
    channel: Channel,
    fhe: FHEncryption<R>,
}

impl<R: Rng> Alice<R> {
    pub fn new(channel: Channel, fhe: FHEncryption<R>) -> Self {
        Self {
            channel,
            fhe,
            encoded_blood: encode(BloodType::ABNeg), // placeholder
        }
    }

    pub fn set_blood_type(&mut self, new_blood_type: BloodType) -> () {
        self.encoded_blood = encode(new_blood_type);
    }

    pub fn send_encrypted_blood_bits(&mut self) -> () {
        // send the encrypted blood bits through the channel
        self.channel.send(self.fhe.encrypt(self.encoded_blood.a));
        self.channel.send(self.fhe.encrypt(self.encoded_blood.b));
        self.channel.send(self.fhe.encrypt(self.encoded_blood.rh));
    }

    pub fn yield_result(&mut self) -> bool {
        return self.fhe.decrypt(self.channel.receive());
    }
}

pub fn setup_parties() -> (Alice<ThreadRng>, Bob) {
    let key_pair = generate_key_pair(1024, 20000, 20, 2000, 1000);

    let fhe = FHEncryption::new(key_pair.public_key, Some(key_pair.secret_key), rand::rng());
    let (a_outgoing, a_incoming) = channel();
    let (b_outgoing, b_incoming) = channel();

    let alice = Alice::new(
        Channel {
            incoming: a_incoming,
            outgoing: b_outgoing,
        },
        fhe,
    );
    let bob = Bob::new(Channel {
        incoming: b_incoming,
        outgoing: a_outgoing,
    });
    return (alice, bob);
}
mod tests {

    use crate::{
        core::functionality::{BLOOD_TYPES, circuit_compatiblity},
        protocols::homomorphic_encryption::encryption::generate_key_pair,
    };

    use super::*;
    #[test]
    fn test() {
        let (mut alice, mut bob) = setup_parties();
        BLOOD_TYPES.iter().for_each(|alice_blood_type| {
            BLOOD_TYPES.iter().for_each(|bob_blood_type| {
                alice.set_blood_type(*alice_blood_type);
                bob.set_blood_type(*bob_blood_type);
                alice.send_encrypted_blood_bits();
                bob.eval();
                let result = alice.yield_result();

                let circuit_result = circuit_compatiblity(*alice_blood_type, *bob_blood_type);
                assert_eq!(
                    result, circuit_result,
                    "mismatch for donor: {:?}, recipient: {:?}",
                    alice_blood_type, bob_blood_type
                );
            });
        });
    }
}
