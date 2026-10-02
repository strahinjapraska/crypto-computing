use crypto_bigint::U2048;

use crate::protocols::oblivious_transfer::oblivious_pke::{utils::{mul_mod, pow_mod, sample_from_zq}, params::{GENERATOR, Q}, public_key::PublicKey};


pub struct SecretKey{
    pub (crate) bytes: [u8; 256] 
}

impl SecretKey{
    pub fn generate() -> Self{
        let alpha = sample_from_zq();
        return SecretKey{
            bytes: alpha.to_be_bytes().try_into().unwrap()
        }
    }


    pub fn public_key(&self) -> PublicKey{
        let alpha = U2048::from_be_slice(&self.bytes);  

        let h = pow_mod(&GENERATOR ,&alpha);
        
        PublicKey { bytes: h.to_be_bytes().try_into().unwrap() }

    }
    pub fn decrypt(&self, ciphertext: (&[u8], &[u8])) -> [u8; 256] {
        let c1 = U2048::from_be_slice(ciphertext.0); 
        let c2 = U2048::from_be_slice(ciphertext.1); 

        let alpha = U2048::from_be_slice(&self.bytes);
        let neg_alpha = Q.wrapping_sub(&alpha); 

        let m = mul_mod( &c2, &pow_mod(&c1, &neg_alpha));

        m.to_be_bytes().as_slice().try_into().expect("Must be 256 bytes long")

    }

    

    pub fn unpad(buffer: &[u8;256]) -> Vec<u8>{
        let len = buffer[0] as usize;
    
        if len == 0 {
            return Vec::new();
        }
        
        let offset = 256 - len;
        buffer[offset..].to_vec()
    }

}