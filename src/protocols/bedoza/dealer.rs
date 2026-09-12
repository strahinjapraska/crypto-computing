use rand::{Rng, RngExt};

type BeaverTriple = (bool, bool, bool);

pub struct Dealer {
  alice_triple: BeaverTriple,
  bob_triple: BeaverTriple,
}

impl Dealer {
  pub fn new<T: Rng> (mut rng: T,) -> Dealer {
    let u = rng.random_bool(0.5);
    let v = rng.random_bool(0.5);
    let w = u & v;

    let u_a = u ^ rng.random_bool(0.5);
    let u_b = u^ u_a;

      let v_a = v ^ rng.random_bool(0.5);
    let v_b = v^ v_a;

    let w_a = w ^ rng.random_bool(0.5);
    let w_b = w^ w_a;

    Dealer {alice_triple:(u_a,v_a,w_a), bob_triple: (u_b,v_b,w_b)}
  }

  pub fn query_alice(&self) -> BeaverTriple {
    self.alice_triple
  }

  pub fn query_bob(&self) -> BeaverTriple {
    self.bob_triple
  }
}

#[cfg(test)]
mod tests {
    use std::assert_eq;

use crate::protocols::bedoza::dealer::Dealer;


  #[test]
  pub fn test_dealer() {
    let rng = rand::rng();
    let dealer = Dealer::new(rng);

    let alice = dealer.query_alice();
    let bob = dealer.query_bob();

    assert_eq!(alice.2^bob.2, alice.0^bob.0 & alice.1^bob.1)
  }
}