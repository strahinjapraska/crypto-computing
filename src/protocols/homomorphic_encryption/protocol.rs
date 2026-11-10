
use std::{
    collections::HashMap,
    sync::mpsc::{Receiver, Sender, channel},
};


use crate::{core::encoding::{BloodType, EncodedBlood, encode}, protocols::homomorphic_encryption::encryption::EncryptedEncodedBlood};

pub struct Channel {
    pub incoming: Receiver<bool>,
    pub outgoing: Sender<bool>,
}
impl Channel {
    pub fn send(&self, value: bool) {
        self.outgoing.send(value).unwrap();
    }
    pub fn receive(&self) -> bool {
        return self.incoming.recv().unwrap();
    }
}


pub struct Bob {
  encoded_blood: EncodedBlood,
  // encrypted_encoded_blood: EncryptedEncodedBlood
}

impl Bob {
  pub fn new(blood_type: BloodType) -> Self {
    return Self {
      encoded_blood: encode(blood_type)
    }
  }

}