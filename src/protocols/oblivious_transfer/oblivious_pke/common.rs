use crypto_bigint::{NonZero, Odd, RandomMod, U2048, modular::{FixedMontyForm, MontyParams}};

use crate::protocols::oblivious_transfer::oblivious_pke::params::{P, Q};

pub fn monty_params() -> MontyParams<crypto_bigint::Uint<32>>{
    let p_odd = Option::from(Odd::new(P)).expect("p must be odd");  

    let params = MontyParams::new(p_odd); 

    params 
}

pub fn pow_mod(base: &U2048, exponent: &U2048) -> U2048{

    let params = monty_params();
    
    let g_monty:  FixedMontyForm<32> =  FixedMontyForm::new(base, &params); 

    let h_monty = g_monty.pow(exponent); 

    let h = h_monty.retrieve();

    h 
}

pub fn sample_from_zq() -> U2048{
        let q_nonzero = NonZero::new(Q).expect("q!=0"); 

        let alpha = U2048::random_mod_vartime(&mut rand::rng(), &q_nonzero);

        alpha 
} 

pub fn mul_mod(a: &U2048, b: &U2048) -> U2048{
    let params = monty_params(); 

    let a_monty: FixedMontyForm<32> = FixedMontyForm::new(a, &params); 
    let b_monty: FixedMontyForm<32> = FixedMontyForm::new(b, &params); 

    (a_monty * b_monty).retrieve()   
}

pub fn u2048_to_zp(m: &U2048) -> U2048{
        let p_nonzero = NonZero::new(P).expect("p !=0");  

        let m_reduced = m.rem(&p_nonzero);    

        m_reduced
}