use crate::protocols::garbled_circuits::{expression::Expr, gate::{Gate, GateType}};

#[derive(Clone, Copy)]
pub struct Wire(usize);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct WireLabels{
    pub k0: [u8; 16], 
    pub k1: [u8; 16], 
}

pub struct CircuitBuilder{
    pub gates: Vec<Gate>, 
    pub next_wire_id: usize,
    pub num_inputs: usize,  
}

impl CircuitBuilder{

    pub fn new() -> Self {
        Self { gates: Vec::new(), next_wire_id: 0 , num_inputs: 0}
    }

    pub fn add_input(&mut self) -> Wire {
        let id = self.next_wire_id;
        self.next_wire_id += 1;
        self.num_inputs += 1; 
        Wire(id)
    }

    pub fn xor(&mut self, u: Wire, v: Wire) -> Wire{
        let out_id = self.next_wire_id; 
        self.next_wire_id += 1; 

        self.gates.push(Gate { id: out_id, gate_type: GateType::XOR, u: u.0, v: v.0, w: out_id });
        
        Wire(out_id)
    }

     pub fn and(&mut self, u: Wire, v: Wire) -> Wire{
        let out_id = self.next_wire_id; 
        self.next_wire_id += 1; 

        self.gates.push(Gate { id: out_id, gate_type: GateType::AND, u: u.0, v: v.0, w: out_id });
        
        Wire(out_id)
    }

    pub fn or(&mut self, u: Wire, v: Wire) -> Wire{
        let out_id = self.next_wire_id; 
        self.next_wire_id += 1; 

        self.gates.push(Gate { id: out_id, gate_type: GateType::OR, u: u.0, v: v.0, w: out_id });
        
        Wire(out_id)
    }

    pub fn not(&mut self, u: Wire) -> Wire {
        let out_id = self.next_wire_id; 
        self.next_wire_id += 1; 

        self.gates.push(Gate { 
            id: out_id, 
            gate_type: GateType::NOT, 
            u: u.0, 
            v: u.0, 
            w: out_id 
        });
        
        Wire(out_id)
    }

       pub fn from_expr(expr: &Expr) -> Self {
        let mut builder = Self::new();
        let num_inputs = expr.num_inputs();

        for _ in 0..num_inputs {
            builder.add_input();
        }

        fn build(e: &Expr, b: &mut CircuitBuilder) -> Wire {
            match e {
                Expr::Input(id) => Wire(*id),
                Expr::Not(inner) => {
                    let u = build(inner, b);
                    b.not(u)
                }
                Expr::And(left, right) => {
                    let u = build(left, b);
                    let v = build(right, b);
                    b.and(u, v)
                }
                Expr::Or(left, right) => {
                    let u = build(left, b);
                    let v = build(right, b);
                    b.or(u, v)
                }
                Expr::Xor(left, right) => {
                    let u = build(left, b);
                    let v = build(right, b);
                    b.xor(u, v)
                }
            }
        }

        build(expr, &mut builder);
        builder
    }
}

#[cfg(test)]
mod test{
    use crate::protocols::garbled_circuits::{expression::Expr, wire::CircuitBuilder};

    #[test]
    fn test_reused_inputs() {
        let x = Expr::Input(0);
        let y = Expr::Input(1);
        let z = Expr::Input(2);

        let logic = (!&x & &y) | (&x ^ &z);

        let builder = CircuitBuilder::from_expr(&logic);
        assert_eq!(builder.gates.len(), 4);
    }
}