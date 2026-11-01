pub enum GateType{
    AND, 
    OR, 
    XOR, 
    NOT 
}

pub struct Gate{
    pub id: usize, 
    pub gate_type: GateType, 
    pub u: usize, // input wire 1  
    pub v: usize, // input wire 2
    pub w: usize  // output wire 
}

pub struct GarbledGate{
    pub ciphertext: [[u8; 32]; 4]
}

pub struct PublicGate{
    pub id: usize, 
    pub u: usize,
    pub v: usize,
    pub w: usize,
}

pub (crate) fn eval(gate_type: &GateType, a: usize, b: usize) -> usize{
    match gate_type {
        GateType::AND => a & b, 
        GateType::OR => a | b, 
        GateType::XOR => a ^ b, 
        GateType::NOT => 1 ^ a, 
    }
}