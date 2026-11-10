use crypto_bigint::{Integer, Odd, RandomBits};
use num_bigint::BigUint;

use rsa::{
    BoxedUint, PrivateKey,
    pkcs8::der::Error,
    traits::{PrivateKeyParts, PublicKeyParts},
};

use crate::protocols::oblivious_transfer::oblivious_pke::{public_key, secret_key};

fn main() -> Result<(), rsa::Error> {
    let mut rng = rand::rng();
    let key = PrivateKey::new(&mut rng, 4096)?;
    println!("{}", key.primes()[0].to_string_radix_vartime(10));
    Ok(())
}

pub struct EncryptedEncodedBlood {
    pub a: BigUint,
    pub b: BigUint,
    pub rh: BigUint,
}

pub fn boxed_uint_to_big_uint(boxed_uint: &BoxedUint ) -> BigUint {
  let bytes = boxed_uint.to_be_bytes();
    BigUint::from_bytes_be(&bytes)
}

pub struct FhePublicKey {
    ys: Vec<BigUint>,
}

impl FhePublicKey {
    pub fn new(n: usize, secret_key: BigUint) -> Self {
        let mut rng = rand::rng();
        let mut ys: Vec<BigUint> = vec![];
        let bit_length = secret_key.bits();
        for i in 0..n {
            let q = boxed_uint_to_big_uint(
              &BoxedUint::random_bits(&mut rng, (bit_length / 2).try_into().unwrap())
            );
            let r = boxed_uint_to_big_uint(
              &BoxedUint::random_bits(&mut rng, (bit_length / 2).try_into().unwrap())
            );
            ys.push(&secret_key * q + BigUint::from(2u32) * r);
        }
        return Self { ys };
    }
    pub fn get(&mut self, index: usize) -> &BigUint {
        return &self.ys[index];
    }
}

pub struct FheKeyPair {
    secret_key: BigUint,
    public_key: FhePublicKey,
}
impl FheKeyPair {
    pub fn new(secret_key: BigUint, public_key: FhePublicKey) -> Self {
        return Self {
            secret_key,
            public_key,
        };
    }
}

pub struct FHEncryption {}

impl FHEncryption {
    pub fn generate_key_pair(n: usize, bit_length: usize) -> FheKeyPair {
        let mut rng = rand::rng();
        let secret_key = boxed_uint_to_big_uint(
          &PrivateKey::new(&mut rng, bit_length).unwrap().primes()[0].clone().get()
        );
        let public_key = FhePublicKey::new(n, secret_key.clone());
        return FheKeyPair::new(secret_key, public_key);
    }
}

mod tests {
    use super::*;
    #[test]
    fn test_prime() {
        let mut key_pair = FHEncryption::generate_key_pair(10, 1024);
        println!("{}",key_pair.secret_key);
        for i in 1..10 {
              println!("{}",key_pair.public_key.get(i));
        }
        // let a: BigUint = "12345600000000000000000000000000000000000078901234567890".parse().unwrap();
        // let b: BigUint = "98765432109876543210".parse().unwrap();

        // // Use references or direct values with the * operator
        // let product = &a * &b;
        // println!("Product: {}", product);
    }
}
