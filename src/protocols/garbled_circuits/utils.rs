use rand::RngExt;
use sha2::{Digest, Sha256};

pub (crate) fn prf_g(l_a: &[u8; 16], r_b: &[u8; 16], gate_id: usize) -> [u8; 32]{
    let mut hasher = Sha256::new(); 
    hasher.update(l_a);
    hasher.update(r_b);
    hasher.update(gate_id.to_le_bytes()); 
    
    hasher.finalize().into()
}

// Fisher-Yates shuffle 
pub (crate) fn shuffle(ciphertexts: &mut [[u8; 32]; 4]){
    let mut rng = rand::rng(); 
    for i in (1..4).rev() {
        let j = rng.random_range(0..=i);
        ciphertexts.swap(i, j);
    }
}