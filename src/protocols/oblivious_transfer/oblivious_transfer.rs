use std::assert_eq;

use crate::protocols::oblivious_transfer::oblivious_pke::{public_key::PublicKey, secret_key::{self, SecretKey}};


pub const MAX_NR_MESSAGES: u8 = 10;
pub const MAX_MESSAGE_LENGTH: u8 = u8::MAX;

pub fn string_to_u8(string: &String) -> &[u8] {
    return string.as_bytes();
}

pub struct Alice {
  pub message_choice: u8,
  pub secret_key: SecretKey,
  pub public_keys: Vec<PublicKey>,
  pub learnt_message: Option<String>
}

impl Alice {
  // 
  pub fn new(message_choice: u8) -> Self {
    if message_choice >= MAX_NR_MESSAGES {
      panic!("Alice initiated with a message choice out of bound: {:?} >= {:?}", message_choice, MAX_NR_MESSAGES);
    }
    let mut public_keys: Vec<PublicKey> = Vec::new();
    let secret_key = SecretKey::generate();
    for i in 0..message_choice  {
        public_keys.push(PublicKey::oblivious_generate(None));
    }
    public_keys.push(secret_key.public_key());
    for i in (message_choice + 1)..MAX_NR_MESSAGES {
      public_keys.push(PublicKey::oblivious_generate(None));
    }

    return Self {
      message_choice,
      secret_key,
      public_keys,
      learnt_message: None
    }
  }

  pub fn get_public_keys(&self) -> Vec<PublicKey> {
    return self.public_keys.clone();
  }

  pub fn retrieve(&mut self, cyphertexts: Vec<([u8; 256], [u8; 256])>) {
    let correct_cyphertext = cyphertexts.get(self.message_choice as usize);
    match correct_cyphertext {
      None => {
        panic!("Error on retrieving the correct cyphertext");
      }
      Some(c) => {
        match String::from_utf8(SecretKey::unpad(&self.secret_key.decrypt((&c.0,&c.1)))) {
          Ok(message) => {
            self.learnt_message = Some(message);
          }
          Err(e) => {
            panic!("{}", e);
          }
        }
      }
    }
  }

}

pub struct Bob {
  pub messages: Vec<String>
}

impl Bob {
  pub fn new(messages: Vec<String>) -> Self {
    messages.iter().for_each(|message| {
      if message.len() > (MAX_MESSAGE_LENGTH as usize) {
        panic!("Message length cannot be longer than {:?}. Message: {:?}", MAX_MESSAGE_LENGTH, message);
      }
    });
    Self {
      messages
    }
  }
  pub fn transfer(&self, public_keys: Vec<PublicKey>) -> Vec<([u8; 256], [u8; 256])> {
    assert_eq!(public_keys.len(), self.messages.len(), "Number of messages and public keys mismatch");
    let mut ciphertexts = Vec::new();
    for i in 0..MAX_NR_MESSAGES {
      ciphertexts.push(public_keys[i as usize].encrypt(&PublicKey::pad(self.messages[i as usize].as_bytes()))); 
    }
    ciphertexts
  }
}

pub struct OTProtocol<'a> {
  alice: &'a mut Alice,
  bob: &'a mut Bob
}

impl<'a> OTProtocol<'a> {
    pub fn new(alice: &'a mut Alice, bob: &'a mut Bob) -> OTProtocol<'a> {
      OTProtocol {alice, bob}
    }

    pub fn run(&mut self) {
      // Choose
      let public_keys = self.alice.get_public_keys();
      // Transfer
      let cyphertexts = self.bob.transfer(public_keys);
      // Retrieve
      self.alice.retrieve(cyphertexts)
    }
}

#[cfg(test)]
mod tests {
    use std::assert_eq;

use rand::{RngExt, rng};

use crate::protocols::oblivious_transfer::oblivious_transfer::{Alice, Bob, MAX_NR_MESSAGES, OTProtocol};

  #[test]
  pub fn test_ot() {
    let mut rng = rng();
    let mut choice = rng.random_range(0..MAX_NR_MESSAGES);
    let mut  alice = Alice::new(choice);
    let messages: Vec<String> = (0..MAX_NR_MESSAGES).map(|i| i.to_string()).collect();
    let mut bob = Bob::new(messages.clone());

    let mut ot = OTProtocol::new(&mut alice, &mut bob);
    ot.run();
    assert_eq!(
      &alice.learnt_message.unwrap(),
      messages.get(choice as usize).unwrap()
    );
  }
}