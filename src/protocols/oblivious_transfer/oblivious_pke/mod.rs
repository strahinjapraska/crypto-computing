pub mod keypair;
pub mod public_key;
pub mod secret_key;
pub mod params; 
pub mod common;

#[cfg(test)]
mod test {
    use crate::protocols::oblivious_transfer::oblivious_pke::{public_key::PublicKey, secret_key::SecretKey};
    

    #[test] 
    fn correctness_test(){
        let m = [2u8, 2u8, 2u8, 2u8];  

        let sk = SecretKey::generate(); 
        let pk = sk.public_key();  

        let ciphertext = pk.encrypt(&PublicKey::pad(&m)); 
        let plaintext = sk.decrypt((&ciphertext.0, &ciphertext.1));

        println!("{:?}", SecretKey::unpad(&plaintext));
    }
}