use crypto_bigint::{NonZero, Odd, RandomMod, U128, U2048, U4096, modular::{FixedMontyForm, MontyParams}};
use rand::{Rng, rng};

use crate::protocols::oblivious_transfer::oblivious_pke::params::{N, P, Q};

fn monty_params() -> MontyParams<crypto_bigint::Uint<32>>{
    let p_odd = Option::from(Odd::new(P)).expect("p must be odd");  

    let params = MontyParams::new(p_odd); 

    params 
}

pub(crate) fn pow_mod(base: &U2048, exponent: &U2048) -> U2048{

    let params = monty_params();
    
    let g_monty:  FixedMontyForm<32> =  FixedMontyForm::new(base, &params); 

    let h_monty = g_monty.pow(exponent); 

    let h = h_monty.retrieve();

    h 
}

pub(crate) fn sample_from_zq() -> U2048{
        let q_nonzero = NonZero::new(Q).expect("q!=0"); 

        let alpha = U2048::random_mod_vartime(&mut rand::rng(), &q_nonzero);

        alpha 
} 

pub(crate) fn mul_mod(a: &U2048, b: &U2048) -> U2048{
    let params = monty_params(); 

    let a_monty: FixedMontyForm<32> = FixedMontyForm::new(a, &params); 
    let b_monty: FixedMontyForm<32> = FixedMontyForm::new(b, &params); 

    (a_monty * b_monty).retrieve()   
}

pub(crate) fn u2048_to_zp(m: &U2048) -> U2048{
        let p_nonzero = NonZero::new(P).expect("p !=0");  

        let m_reduced = m.rem(&p_nonzero);    

        m_reduced
}

pub(crate) fn slice_to_u4096(offset: usize, slice: &[u8]) -> U4096{
    let mut buffer = [0u8; 512]; 
    buffer[offset..].copy_from_slice(slice); 

    let r_int = U4096::from_be_slice(&buffer); 
    
    r_int 
}

pub(crate) fn rejection_sample(r: &[u8; N]) -> U2048{

    let r_int = slice_to_u4096(240, r);
    let p_4096 = slice_to_u4096(256, &P.to_be_bytes());

    let p_minus_1 = p_4096.wrapping_sub(&U4096::ONE);
    let p_minus_1_nonzero = NonZero::new(p_minus_1).expect("p-1 !=0");

    let r_mod_p_minus_1 = r_int.rem(&p_minus_1_nonzero);

    let s_4096 = r_mod_p_minus_1.wrapping_add(&U4096::ONE); 

    let s_bytes = s_4096.to_be_bytes(); 
    
    U2048::from_be_slice(&s_bytes[256..])
}


pub (crate) fn inv_rejection_sample(s: &U2048) -> [u8; N]{
    let s_minus_1 = s.wrapping_sub(&U2048::ONE);
    let p_minus_1 = P.wrapping_sub(&U2048::ONE);

    let s_4096: U4096 = s_minus_1.resize(); 
    let p_4096: U4096 = p_minus_1.resize(); 


    let mut k_bytes = [0u8; 16];
    rand::rng().fill_bytes(&mut k_bytes); 

    let k_128 = u128::from_be_bytes(k_bytes);
    let k_4096 = U4096::from_u128(k_128); 

    let k_times_p = k_4096.wrapping_mul(&p_4096);
    let r_4096 = s_4096.wrapping_add(&k_times_p);

    let r_bytes = r_4096.to_be_bytes(); 
    let mut r = [0u8; 272];
    r.copy_from_slice(&r_bytes[512 - N..]); 

    r  
}