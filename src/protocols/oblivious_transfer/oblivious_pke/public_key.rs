use crypto_bigint::{CtLt, U2048};
use rand::{RngExt, rng};
use crate::protocols::oblivious_transfer::oblivious_pke::{utils::{inv_rejection_sample, mul_mod, pow_mod, rejection_sample, sample_from_zq, u2048_to_zp}, params::{GENERATOR, N, P}};
use rand_core::Rng;
 
#[derive(PartialEq, Clone)]
pub struct PublicKey{
    pub (crate) bytes: [u8; 256]
}

impl PublicKey{

    pub fn pad(plaintext: &[u8]) -> [u8; 256] {
        assert!(plaintext.len() <= 255);

        let mut buffer = [0u8; 256];
        let len = plaintext.len();
        
        buffer[0] = len as u8;
        
        let offset = 256 - len;
        buffer[offset..].copy_from_slice(plaintext);
        
        buffer

    }

    pub fn encrypt(&self, plaintext: &[u8; 256]) -> ([u8; 256], [u8; 256]){

        let mut buffer = [0u8; 256];
        let offset = 256 - plaintext.len();
        buffer[offset..].copy_from_slice(plaintext); 

        let m = U2048::from_be_slice(&buffer);  

        let is_m_less_than_p= m.ct_lt(&P).to_bool(); 

        assert!(is_m_less_than_p == true); 

        let m_reduced = u2048_to_zp(&m);

        let h = U2048::from_be_slice(&self.bytes);

        let r = sample_from_zq(); 

        let c1 = pow_mod(&GENERATOR, &r); 
        let c2 = mul_mod(&m_reduced, &pow_mod(&h, &r)); 



        (c1.to_be_bytes().as_slice().try_into().unwrap(), c2.to_be_bytes().as_slice().try_into().unwrap())

    }  

    pub fn oblivious_generate(r_opt: Option<[u8; N]>) -> PublicKey{

        let r = match r_opt {
            Some(provided_r) => provided_r, 
            None => {
                  let mut fresh_r = [0u8; N]; 
                  rand::rng().fill_bytes(&mut fresh_r); 
                  fresh_r 
            }
        }; 
      
        let s = rejection_sample(&r);  

        let h = mul_mod(&s, &s);  

        PublicKey { bytes: h.to_be_bytes().try_into().unwrap()}
    }

    pub fn inverse_oblivious_generate(pk: &PublicKey) -> [u8; N]{
        let t = U2048::from_be_slice(&pk.bytes); 

        // Square root mod p, we don't use Tonelli-Shanks but 
        // https://en.wikipedia.org/wiki/Tonelli–Shanks_algorithm 
        // p = 3 mod 4 method, i.e. s = t^{p+1}/4
        let exponent = &P.wrapping_add(&U2048::ONE).shr(2);  
        let s = pow_mod(&t, exponent);  

        let random_bit = rng().random_bool(0.5); 
        if random_bit{
            inv_rejection_sample(&s)
        }
        else{
            inv_rejection_sample(&P.wrapping_sub(&s))
        }
    }

   
}

