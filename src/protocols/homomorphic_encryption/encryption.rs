use std::ops::Add;

use crypto_bigint::RandomBits;
use num_bigint::BigUint;

use rand::{Rng, seq::SliceRandom};
use rsa::{BoxedUint, PrivateKey, traits::PrivateKeyParts};


pub struct EncryptedEncodedBlood {
    pub a: BigUint,
    pub b: BigUint,
    pub rh: BigUint,
}

pub fn boxed_uint_to_big_uint(boxed_uint: &BoxedUint) -> BigUint {
    let bytes = boxed_uint.to_be_bytes();
    BigUint::from_bytes_be(&bytes)
}

pub struct FhePublicKey {
    ys: Vec<BigUint>,
    subset_size: usize,
}

impl FhePublicKey {
    pub fn new(
        q_size: usize,
        r_size: usize,
        n: usize,
        secret_key: BigUint,
        subset_size: usize,
    ) -> Self {
        if subset_size > n{
            panic!("subset_size should be smaller than n.");
        }
        let mut rng = rand::rng();
        let mut ys: Vec<BigUint> = vec![];
        for _ in 0..n {
            let q = boxed_uint_to_big_uint(&BoxedUint::random_bits(
                &mut rng,
                (q_size).try_into().unwrap(),
            ));
            let r = boxed_uint_to_big_uint(&BoxedUint::random_bits(
                &mut rng,
                (r_size).try_into().unwrap(),
            ));
            ys.push(&secret_key * q + BigUint::from(2u32) * r);
        }
        return Self { ys, subset_size };
    }
    pub fn get(&mut self, index: usize) -> &BigUint {
        return &self.ys[index];
    }
}

pub struct FheKeyPair {
    pub secret_key: BigUint,
    pub public_key: FhePublicKey,
}
impl FheKeyPair {
    pub fn new(secret_key: BigUint, public_key: FhePublicKey) -> Self {
        return Self {
            secret_key,
            public_key,
        };
    }
}

pub fn generate_key_pair(p_size: usize, q_size: usize, r_size: usize, n: usize, subset_size: usize) -> FheKeyPair {
    let mut rng = rand::rng();
    let secret_key = boxed_uint_to_big_uint(
        &PrivateKey::new(&mut rng, p_size).unwrap().primes()[0]
            .clone()
            .get(),
    );
    let public_key = FhePublicKey::new(q_size, r_size, n, secret_key.clone(), subset_size);
    return FheKeyPair::new(secret_key, public_key);
}
pub struct FHEncryption<R: Rng> {
    public_key: FhePublicKey,
    secret_key: Option<BigUint>,
    pub rng: R
}

impl<R:Rng> FHEncryption<R> {

    pub fn new(public_key: FhePublicKey, secret_key: Option<BigUint>, rng: R) -> Self {
        return Self {
            public_key,
            secret_key,
            rng
        };
    }

    pub fn encrypt(&mut self, bit: bool) -> BigUint {
      // perform a random shuffle on the ys vector and only go through the first sub_size elements
      let mut numbers: Vec<usize> = (1..=(self.public_key.ys.len())).collect();
      numbers.shuffle(&mut self.rng);
      let mut accumulator: BigUint = BigUint::ZERO;
      for index in 1..self.public_key.subset_size {
        accumulator += self.public_key.ys.get(index).unwrap();
      }
      if bit {
        return accumulator + BigUint::ONE;
      } else {
        return accumulator;
      }
    }
  
  pub fn decrypt(&mut self, c: BigUint) -> bool {
    match self.secret_key {
      Some(ref secret_key) => {
        return (c % secret_key) % BigUint::from(2u32) == BigUint::ONE;
      }
      None => {
        panic!("Cannot decrypt without a secret key");
      }
    }
  }
}

mod tests {

use rand::rngs::ThreadRng;

use super::*;
    #[test]
    fn test_prime() {
        let mut key_pair = generate_key_pair(2000, 20000, 6, 20, 10);
        println!("{}", key_pair.secret_key);
        for i in 1..10 {
            println!("{}", key_pair.public_key.get(i));
        }
        let mut fhe = FHEncryption::new(key_pair.public_key, Some(key_pair.secret_key), rand::rng());
        for i in 0..1 {
          let c = fhe.encrypt(i == 1);
          assert_eq!(i == 1, fhe.decrypt(c));
        }
    }
}
