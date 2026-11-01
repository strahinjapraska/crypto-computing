use std::collections::HashMap;

use rand::Rng;

use crate::protocols::garbled_circuits::{gate::{GarbledGate, Gate, GateType, PublicGate, eval}, utils::{prf_g, shuffle}, wire::CircuitBuilder};
use crate::protocols::garbled_circuits::wire::WireLabels;


pub struct F{ 
    pub input_wires: Vec<usize>, // indices of input wires 
    pub output_wire: usize, // index of output wire 
    pub public_gates: Vec<PublicGate>,  
    pub permuted_tables: Vec<GarbledGate>,
}

pub struct GarbledCircuit{ 
    pub f: F, 
    pub e: Vec<WireLabels>, 
    pub d: WireLabels, 
}


pub (crate) fn garble(builder: &CircuitBuilder) -> GarbledCircuit{
    let labels = generate_all_labels(builder.next_wire_id); 
    let garbled_gates = garble_gates(&builder.gates, &labels); 

    let output_wire = builder.gates.last().unwrap().w; 


    build_garbled_circuit(
        &builder.gates, 
        garbled_gates, 
        &labels, 
        builder.num_inputs, 
        output_wire)
}

fn build_garbled_circuit(gates: &[Gate], permuted_tables: Vec<GarbledGate>, labels: &[WireLabels], n: usize, output_wire: usize) -> GarbledCircuit{
    let public_gates: Vec<PublicGate> = gates.iter().map(|gate| PublicGate {
        id: gate.id,
        u: gate.u,
        v: gate.v,
        w: gate.w,
    }).collect();

    let f = F{
        input_wires: (0..n).collect(),
        output_wire,
        public_gates,
        permuted_tables: permuted_tables,
    };

    let e = labels[0..n].to_vec();  

    let d = labels[output_wire].clone(); 

    GarbledCircuit { f, e, d }
}

fn garble_gates(gates: &[Gate], labels: &[WireLabels]) -> Vec<GarbledGate>{
    let mut garbled_gates = Vec::new(); 

    for gate in gates{
        let mut ciphertexts = [[0u8; 32]; 4]; 

        let mut j = 0; 

        for a in 0..=1{
            for b in 0..=1{ 
                let l_a = if a == 0 { &labels[gate.u].k0 } else { &labels[gate.u].k1 }; 
                let r_b = if b == 0 { &labels[gate.v].k0 } else { &labels[gate.v].k1 }; 

                let c = eval(&gate.gate_type, a, b); 
                let k_c = if c == 0 { &labels[gate.w].k0 } else { &labels[gate.w].k1 }; 


                let mut ciphertext = prf_g(l_a, r_b, gate.id);
                for i in 0..16 {
                    ciphertext[i] ^= k_c[i];
                }

                ciphertexts[j] = ciphertext;
                j+=1; 
            }
        }
        shuffle(&mut ciphertexts); 

        garbled_gates.push(GarbledGate { ciphertext: ciphertexts });
    }
    garbled_gates
}

fn generate_all_labels(n: usize) -> Vec<WireLabels>{
    let mut rng = rand::rng();
    (0..n).map(|_|  { 
        let mut k0 = [0u8; 16]; 
        let mut k1 = [0u8; 16]; 
    
        rng.fill_bytes(&mut k0); 
        rng.fill_bytes(&mut k1); 

        WireLabels { k0, k1 }
    }).collect()
}


pub (crate) fn evaluate(f: &F, x: &[[u8; 16]])->  Result<[u8; 16], &'static str>{

    // 1. Assign the i-th label in X to input wire i.
    let mut active_wires: HashMap<usize, [u8; 16]> = HashMap::new(); 

    for(i, &wire_id) in f.input_wires.iter().enumerate() {
        active_wires.insert(wire_id, x[i]); 
    } 

    // 2. ...  
    for(i, gate) in f.public_gates.iter().enumerate() {
        let l = active_wires.get(&gate.u).unwrap(); 
        let r = active_wires.get(&gate.v).unwrap(); 

        let pad = prf_g(l, r, gate.id); 

        let mut matches = 0; 
        let mut output_key: Option<[u8; 16]> = None; 
        
        for ciphertext in f.permuted_tables[i].ciphertext.iter(){
            let mut decrypted = [0u8; 32]; 
            for j in 0..32{
                decrypted[j] = ciphertext[j] ^ pad[j]; 
            }

            let is_zero = decrypted[16..32].iter().all(|&b| b == 0); 

            if is_zero{ 
                output_key = Some(decrypted[0..16].try_into().unwrap()); 
                matches += 1;
            }
        }
        if matches != 1{
            return Err("⊥");
        }else{
            active_wires.insert(gate.w, output_key.unwrap());
        }
    }

    // 3. Return the label Z assigned to the circuit’s output wire t.
    let z = active_wires.get(&f.output_wire).unwrap();
    Ok(*z)
}

pub(crate) fn encode(e: &Vec<WireLabels>, x: &Vec<usize>) -> Vec<[u8; 16]>{
    e.iter()
        .zip(x.iter())
        .map(|(label, &bit)| {
            if bit == 0 { label.k0 } else { label.k1 }
        })
        .collect()
}

pub(crate) fn decode(d: &WireLabels, z: &[u8; 16]) -> Result<usize, &'static str>{
    if *z == d.k0 {
        Ok(0)
    } else if *z == d.k1 {
        Ok(1)
    } else {
        Err("⊥")
    }
}

#[cfg(test)]
mod test {
    use crate::protocols::garbled_circuits::{expression::Expr, wire::CircuitBuilder, yao::{WireLabels, decode, encode, evaluate, garble}};


    #[test] 
    fn test_garble() {

        let x= Expr::Input(0);
        let y = Expr::Input(1);
        let z = Expr::Input(2);

        let builder = CircuitBuilder::from_expr(&((x ^ y) ^ z));

        let garbled_circuit = garble(&builder); 

        assert_eq!(garbled_circuit.f.input_wires.len(), 3);
        assert_eq!(garbled_circuit.f.output_wire, 4);
        assert_eq!(garbled_circuit.f.public_gates.len(), 2);
        assert_eq!(garbled_circuit.f.permuted_tables.len(), 2);
        assert_eq!(garbled_circuit.e.len(), 3);
    } 

    #[test]
    fn test_encode_decode(){
        let label = WireLabels {
            k0: [42u8; 16],
            k1: [39u8; 16], 
        }; 

        let e = vec![label.clone()];
        let d = label;

        let zero_bit = vec![0];
        let encoding_0 = encode(&e, &zero_bit);

        
        let decoded_0 = decode(&d, &encoding_0[0]).unwrap();
        assert_eq!(decoded_0, 0);

        let one_bit = vec![1];
        let encoding_1 = encode(&e, &one_bit); 
        
        let decoded_1 = decode(&d, &encoding_1[0]).unwrap();
        assert_eq!(decoded_1, 1);
    }

    #[test]
    fn test_end_to_end(){
        let x = Expr::Input(0);
        let y = Expr::Input(1);
        let logic = &x ^ &y; 

        let builder = CircuitBuilder::from_expr(&logic);


        let garbled_circuit = garble(&builder);

        for a in 0..=1 {
            for b in 0..=1 {
                let plain_inputs = vec![a, b];
                let expected = a ^ b;

                let active_keys = encode(&garbled_circuit.e, &plain_inputs);
                
                let z = evaluate(&garbled_circuit.f, &active_keys)
                    .expect("⊥");
                    
                let output = decode(&garbled_circuit.d, &z)
                    .expect("⊥");

                assert_eq!(output, expected, "Failed on {} ^ {}", a, b);
            }
        }
        }
}