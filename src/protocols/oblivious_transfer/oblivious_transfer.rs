use crate::protocols::oblivious_transfer::oblivious_pke::{public_key::PublicKey, secret_key::{self, SecretKey}};


pub const MAX_NR_MESSAGES: u8 = 10;
pub const MAX_MESSAGE_LENGTH: u8 = u8::MAX;

pub fn string_to_u8(string: String) -> &[u8] {
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

  pub fn retrieve(&mut self, cyphertexts: Vec<Vec<u8>>) {
    let correct_cyphertext = cyphertexts.get(self.message_choice as usize);
    match correct_cyphertext {
      None => {
        panic!("Error on retrieving the correct cyphertext");
      }
      Some(c) => {
        match String::from_utf8(c.to_vec()) {
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
  pub fn transfer(&self, public_keys: Vec<PublicKey>) -> Vec<Vec<u8>> {
    return Vec::new();
  }
}

pub struct OTProtocol {
}

impl OTProtocol {
    pub fn runProtocol(alice: &Alice, bob: &bob) {
      // Choose
      let public_keys = alice.get_public_keys();
      // Transfer
      let cyphertexts = bob.transfer(public_keys);
      // Retrieve
      alice.retrieve(cyphertexts);
    }
}