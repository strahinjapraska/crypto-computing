use crate::protocols::{garbled_circuits::{expression::Expr, wire::{CircuitBuilder, WireLabels}, yao::{self, F, GarbledCircuit, garble}}, oblivious_transfer::oblivious_transfer::OTProtocol};
use crate::protocols::oblivious_transfer::oblivious_transfer::{Alice, Bob};

pub struct Garbler{
    gc: GarbledCircuit,
    y: Vec<usize>,
}

impl Garbler{
    pub fn new(circuit: &Expr, y: Vec<usize>) -> Self{ 
        let builder = CircuitBuilder::from_expr(circuit); 
        let gc = garble(&builder);
        Self { gc, y }
    }

    pub fn send_circuit(&self) -> (&F, &WireLabels){
        (&self.gc.f, &self.gc.d)
    }

    pub fn encode(&self) -> Vec<[u8; 16]> {
        let n = self.y.len(); 
        let e_y = &self.gc.e[n..].to_vec();     

        yao::encode(e_y, &self.y)
    }

    pub fn get_ot_input(&self)-> Vec<([u8; 16], [u8; 16])>{
        let n = self.y.len(); 
        let e_x = &self.gc.e[0..n];    

        e_x.iter().map(|labels| (labels.k0, labels.k1)).collect()
    }
}

pub struct Evaluator{
    pub x: Vec<usize>, 
    pub z: Option<usize>
}

impl Evaluator{
    pub fn new(x: Vec<usize>) -> Self{
        Self { x, z: None }
    }

    pub fn compute(&mut self, f: &F, d: &WireLabels, x: &Vec<[u8; 16]>, y: &Vec<[u8; 16]>) -> Result<(), &'static str>{
        let mut x_concat_y = Vec::new(); 
        x_concat_y.extend(x);
        x_concat_y.extend(y);

        let z_label = yao::evaluate(f, &x_concat_y)?; 
        let z = yao::decode(d, &z_label)?; 

        self.z = Some(z); 

        Ok(())
    }
}

pub struct YaoProtocol<'a> {
    pub alice: &'a mut Evaluator,
    pub bob: &'a mut Garbler,
}

impl<'a> YaoProtocol<'a> {
    pub fn new(alice: &'a mut Evaluator, bob: &'a mut Garbler) -> Self {
        Self { alice, bob }
    }

    pub fn run(&mut self){ 
        // 1. 
        let (f, d) = self.bob.send_circuit(); 

        // 2. 
        let y = self.bob.encode(); 

        // 3. 
        let ot_bob_input = self.bob.get_ot_input(); 
        let mut x = Vec::new(); 

        for(i, &choice) in self.alice.x.iter().enumerate(){
            let (k0, k1) = ot_bob_input[i]; 
            
            let k0_hex = hex::encode(k0); 
            let k1_hex = hex::encode(k1); 
        

            let mut ot_alice = Alice::new(choice as u8); 
            let mut ot_bob = Bob::new(vec![k0_hex, k1_hex]);  


            let mut ot = OTProtocol::new(&mut ot_alice, &mut ot_bob); 
            ot.run();


            let learnt_str = ot_alice.learnt_message.clone().expect("OT failed to retrieve message");
            let learnt_bytes = hex::decode(learnt_str).expect("Failed to decode hex");
            let learnt_key: [u8; 16] = learnt_bytes.try_into().expect("Key length mismatch");


            x.push(learnt_key);
        }


        // 4. 
        self.alice.compute(f, d, &x, &y).expect("Protocol eval failed"); 
    }
}

#[cfg(test)]
mod test{
    use crate::{core::{encoding::{BloodType, encode, to_bits}, functionality::circuit_compatiblity_usize}, protocols::garbled_circuits::{expression::Expr, protocol::{Evaluator, Garbler, YaoProtocol}}};


    #[test]
    fn test_all_blood_types_yao(){
        let a_r = Expr::Input(0);
        let b_r = Expr::Input(1);
        let rh_r = Expr::Input(2);

        let a_d = Expr::Input(3);
        let b_d = Expr::Input(4);
        let rh_d = Expr::Input(5);

        let a_compat = !&a_d | &a_r;
        let b_compat = !&b_d | &b_r;
        let rh_compat = !&rh_d | &rh_r;

        let logic = &(&a_compat & &b_compat) & &rh_compat;

        let all_types = [
            BloodType::ONeg, BloodType::OPos, BloodType::ANeg, BloodType::APos,
            BloodType::BNeg, BloodType::BPos, BloodType::ABNeg, BloodType::ABPos,
        ];

        for &recipient_type in &all_types {
            for &donor_type in &all_types {
                
        
                let alice_bits = to_bits(encode(recipient_type));
                let bob_bits = to_bits(encode(donor_type));

                let mut alice = Evaluator::new(alice_bits);
                let mut bob = Garbler::new(&logic, bob_bits);

                let mut protocol = YaoProtocol::new(&mut alice, &mut bob);
                protocol.run();

                let expected: usize = circuit_compatiblity_usize(donor_type, recipient_type);
                assert_eq!(
                    alice.z.unwrap(), 
                    expected, 
                    "Failed on Donor: {:?}, Recipient: {:?}", donor_type, recipient_type
                );
            }
        }



    }
}