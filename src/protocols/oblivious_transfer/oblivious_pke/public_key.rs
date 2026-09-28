use crypto_bigint::{CtLt, NonZero, U2048, U4096};
use rand::{RngExt, rng};
use crate::protocols::oblivious_transfer::oblivious_pke::{common::{mul_mod, pow_mod, sample_from_zq, u2048_to_zp}, params::{GENERATOR, N, P}};
use rand_core::Rng; 
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

    pub fn oblivious_generate() -> PublicKey{
        let mut r = [0u8; N]; 
        rand::rng().fill_bytes(&mut r); 

        let s = Self::rejection_sample(&r);  

        let h = mul_mod(&s, &s);  

        PublicKey { bytes: h.to_be_bytes().try_into().unwrap()}
    }

    pub fn inverse_oblivious_generate(pk: &PublicKey) -> [u8; N]{
        let t = U2048::from_be_slice(&pk.bytes); 

        // Square root mod p, we don't use Tonelli-Shanks but 
        // https://en.wikipedia.org/wiki/Tonelli–Shanks_algorithm 
        // p = 3 mod 4 method, i.e. s = t^{p-1}/4
        let exponent = &P.wrapping_add(&U2048::ONE).shr(2);  
        let s = pow_mod(&t, exponent);  

        let random_bit = rng().random_bool(0.5); 
        if random_bit{
            Self::inv_rejection_sample(&s)
        }
        else{
            Self::inv_rejection_sample(&P.wrapping_sub(&s))
        }
    }

    fn inv_rejection_sample(s: &U2048) -> [u8; N]{
        unimplemented!()
    }

    fn rejection_sample(r: &[u8; N]) -> U2048{

        fn slice_to_u4096(offset: usize, slice: &[u8]) -> U4096{
            let mut buffer = [0u8; 512]; 
            buffer[offset..].copy_from_slice(slice); 

            let r_int = U4096::from_be_slice(&buffer); 
            
            r_int 
        }

        let r_int = slice_to_u4096(240, r);
        let p_4096 = slice_to_u4096(256, &P.to_be_bytes());

        let p_minus_1 = p_4096.wrapping_sub(&U4096::ONE);
        let p_minus_1_nonzero = NonZero::new(p_minus_1).expect("p-1 !=0");

        let r_mod_p_minus_1 = r_int.rem(&p_minus_1_nonzero);

        let s_4096 = r_mod_p_minus_1.wrapping_add(&U4096::ONE); 

        let s_bytes = s_4096.to_be_bytes(); 
        
        U2048::from_be_slice(&s_bytes[256..])
    }

   
}

