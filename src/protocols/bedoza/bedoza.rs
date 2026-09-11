use std::{sync::mpsc::{Receiver, Sender, channel}}; 

use rand::{Rng, RngExt};

use crate::protocols::common::Role;


pub struct Channel{
    pub incoming: Receiver<bool>, 
    pub outgoing: Sender<bool>,
}
impl Channel{
    pub fn send(&self, value: bool){
        self.outgoing.send(value).unwrap();
    } 
    pub fn receive(&self) -> bool{
        return self.incoming.recv().unwrap();
    } 
}

pub struct Party{
    pub role: Role, 
    pub channel: Channel,
}

impl Party{
    pub fn new(role: Role, channel: Channel) -> Self{
        return Self{
            role, 
            channel,
        }
    }
    pub fn share_input<R: Rng>(&self, input: Option<bool>, rng: &mut R) -> bool{
        match(input) {
            Some(input) => {
                let wire_share = rng.random_bool(0.5); 
                let own_share = input ^ wire_share; 
                self.channel.send(wire_share);  

                return own_share;
            }, 
            None => {
                let value = self.channel.receive();
                return value;
            }
        }
    }

    pub fn reconstruct_output(&self, own_share: bool) -> Option<bool>{
        match self.role {
            Role::Alice => {
               let x_b = self.channel.receive(); 
               return Some(own_share ^ x_b); 
            }, 
            Role::Bob => {
                self.channel.send(own_share); 
                None 
            }
        }
    }
}

pub struct BeDOZaProtocol{

}

impl BeDOZaProtocol {
    pub fn new() -> Self {
        Self {

        }
    }

    pub fn run_protocol(&mut self, alice_input: bool, bob_input: bool) -> bool {
        
        let mut rng = rand::rng(); 

        let (a_outgoing, a_incoming) = channel(); 
        let (b_outgoing, b_incoming) = channel(); 

        let alice = Party::new(Role::Alice, Channel{incoming: a_incoming, outgoing: b_outgoing});
        let bob = Party::new(Role::Bob, Channel{incoming: b_incoming, outgoing: a_outgoing}); 

        let x_a = alice.share_input(Some(alice_input), &mut rng); 
        let x_b = bob.share_input(None, &mut rng); 

        let _ = bob.reconstruct_output(x_b); 
        let z = alice.reconstruct_output(x_a).unwrap(); 

        z 
    }
}

#[cfg(test)]
mod test{
    #[test] 
    fn test_bedoza_protocol(){
        let mut protocol = super::BeDOZaProtocol::new(); 

        let test_cases = [
            (false, false),
            (false, true),
            (true, false),
            (true, true),
        ];

        for (alice_input, bob_input) in test_cases {
            let z = protocol.run_protocol(alice_input, bob_input);
            assert_eq!(
                z, alice_input,
            );
        }
    }
}