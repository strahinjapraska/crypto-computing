use rand::{RngExt, rand_core::Rng};
use crate::core::functionality::TABLE;

const N: usize = 3; 
const M: usize = 100; 
const SIZE: usize = 1<<N; 

type BitMatrix = [[bool; SIZE]; SIZE];

pub(crate) struct Dealer{
    alice_pairs: Vec<(usize, BitMatrix)>, 
    bob_pairs: Vec<(usize, BitMatrix)>,
}

impl Dealer{
    pub(crate) fn new<R: Rng>(rng: &mut R) -> Self{
        let mut alice_pairs = Vec::new(); 
        let mut bob_pairs = Vec::new();

        for _ in 0..M{
            let ((r, alice_matrix), (s, bob_matrix)) = Dealer::gen_correlated_randomness(rng);

            alice_pairs.push((r, alice_matrix));
            bob_pairs.push((s, bob_matrix));
        }

        Self {alice_pairs, bob_pairs}
    }

    fn gen_correlated_randomness<R: Rng>(rng: &mut R) -> ((usize, BitMatrix), (usize, BitMatrix)){
          let sample_rand_bit_matrix = |rng: &mut R| -> BitMatrix {
            std::array::from_fn(|_| {
            std::array::from_fn(|_| rng.random_bool(0.5))
        })};

        let s: usize = rng.random_range(0..SIZE); 
        let r: usize = rng.random_range(0..SIZE);

        let bob_matrix: BitMatrix = sample_rand_bit_matrix(rng);

        let alice_matrix: BitMatrix = std::array::from_fn(|i|{
            std::array::from_fn(|j|{
                let row = (i + SIZE - r) % SIZE; 
                let col = (j + SIZE - s) % SIZE; 

                bob_matrix[i][j] ^ TABLE[row][col]
            })
        }); 
        ((r, alice_matrix), (s, bob_matrix))
    }
    pub(crate) fn query_alice(&mut self) -> Option<(usize, BitMatrix)>{
        self.alice_pairs.pop() 
    }
    pub(crate) fn query_bob(&mut self) -> Option<(usize, BitMatrix)>{
        self.bob_pairs.pop()
    }
}

