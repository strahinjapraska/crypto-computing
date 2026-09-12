use std::{
    collections::HashMap,
    sync::mpsc::{Receiver, Sender, channel},
};

use rand::{Rng, RngExt, rngs::ThreadRng};

use crate::protocols::{
    bedoza::expressions::{Expression, ExpressionTypes, VariableNames},
    common::Role,
};

pub struct Channel {
    pub incoming: Receiver<bool>,
    pub outgoing: Sender<bool>,
}
impl Channel {
    pub fn send(&self, value: bool) {
        self.outgoing.send(value).unwrap();
    }
    pub fn receive(&self) -> bool {
        return self.incoming.recv().unwrap();
    }
}

pub struct Party {
    pub role: Role,
    pub channel: Channel,
    pub state: HashMap<VariableNames, bool>,
    pub rng: ThreadRng
}

impl Party {
    pub fn new(role: Role, channel: Channel, rng: ThreadRng) -> Self {
        return Self {
            role,
            channel,
            state: HashMap::new(),
            rng
        };
    }
    pub fn share_input(
        &mut self,
        store_in_variable: VariableNames,
        input: Option<bool>
    ) -> bool {
        match input {
            Some(input) => {
                let wire_share = self.rng.random_bool(0.5);
                let own_share = input ^ wire_share;
                self.state.insert(store_in_variable, own_share);
                println!("{:?} inserts ({:?}, {:?})", self.role, store_in_variable, own_share);
                self.channel.send(wire_share);

                return own_share;
            }
            None => {
                let value = self.channel.receive();
                println!(
                    "{:?} received share for var {:?}, value {}",
                    self.role, store_in_variable, value
                );
                self.state.insert(store_in_variable, value);
                return value;
            }
        }
    }

    pub fn reconstruct_output(&self, output_variable: VariableNames) -> Option<bool> {
        match self.role {
            Role::Alice => {
                let x_b = self.channel.receive();
                return Some(self.state.get(&output_variable).unwrap() ^ x_b);
            }
            Role::Bob => {
                self.channel
                    .send(*self.state.get(&output_variable).unwrap());
                None
            }
        }
    }

    pub fn evaluate_expression(&mut self, expression: Expression) {
        match expression.expression_type {
            ExpressionTypes::XORWithConstant => {
                if expression.constant.is_none() {
                    panic!("constant is none!");
                }
                match self.role {
                    Role::Alice => {
                        self.state.insert(
                            expression.output_variable_name,
                            self.state
                                .get(&expression.first_input_variable_name)
                                .unwrap()
                                ^ expression.constant.unwrap(),
                        );
                    }
                    Role::Bob => {
                        self.state.insert(
                            expression.output_variable_name,
                            *self.state
                                .get(&expression.first_input_variable_name)
                                .unwrap()
                        );
                    }
                }
            }
            ExpressionTypes::ANDWithConstant => {
                if expression.constant.is_none() {
                    panic!("constant is none!");
                }
                self.state.insert(
                    expression.output_variable_name,
                    *self
                        .state
                        .get(&expression.first_input_variable_name)
                        .unwrap()
                        && expression.constant.unwrap(),
                );
            }
            ExpressionTypes::XORWithTwoWires => {
                if expression.second_input_variable_name.is_none() {
                    panic!("second_input_variable_name is none!");
                }
                self.state.insert(
                    expression.output_variable_name,
                    self.state
                        .get(&expression.first_input_variable_name)
                        .unwrap()
                        ^ self
                            .state
                            .get(&expression.second_input_variable_name.unwrap())
                            .unwrap(),
                );
            }
            ExpressionTypes::ANDWithTwoWires => todo!(),
        }
    }
}

pub struct BeDOZaProtocol {
    expression_list: Vec<Expression>,
}

impl BeDOZaProtocol {
    pub fn new(expression_list: Vec<Expression>) -> Self {
        Self { expression_list }
    }

    pub fn run_protocol(&mut self, alice_input: bool, bob_input: bool) -> bool {
        let mut alice_rng = rand::rng();
        let mut bob_rng = rand::rng();

        let (a_outgoing, a_incoming) = channel();
        let (b_outgoing, b_incoming) = channel();

        let mut alice = Party::new(
            Role::Alice,
            Channel {
                incoming: a_incoming,
                outgoing: b_outgoing,
            },
            alice_rng
        );
        let mut bob = Party::new(
            Role::Bob,
            Channel {
                incoming: b_incoming,
                outgoing: a_outgoing,
            },
            bob_rng
        );

        alice.share_input(VariableNames::DONOR_A, Some(alice_input));
        // force Bob to receive the share from Alice
        bob.share_input(VariableNames::DONOR_A, None);

        bob.share_input(VariableNames::RECIPIENT_A, Some(bob_input));
        // force Alice to receive the share from Bob
        alice.share_input(VariableNames::RECIPIENT_A, None);

        println!("{:=>60}", "");

        println!("initial states");
        println!("Alice state: {:?}", alice.state);
        println!("Bob state: {:?}", bob.state);
        println!("{:=>60}", "");

        self.expression_list.iter().for_each(|expression| {
            alice.evaluate_expression(expression.clone());
            bob.evaluate_expression(expression.clone());
            println!("{:?}", expression);
            println!("Alice state: {:?}", alice.state);
            println!("Bob state: {:?}", bob.state);
            println!("{:=>60}", "");
        });

        let last_updated_variable = self.expression_list.last().unwrap().output_variable_name;

        let _ = bob.reconstruct_output(last_updated_variable);
        let z = alice.reconstruct_output(last_updated_variable).unwrap();

        z
    }
}

#[cfg(test)]
mod test {
    use crate::protocols::bedoza::expressions::get_local_running_expressions;

    #[test]
    fn test_bedoza_protocol() {
        //what it does: (d.a + d.b) * 1 + 1
        let mut protocol = super::BeDOZaProtocol::new(get_local_running_expressions().to_vec());

        let test_cases = [(false, false), (false, true), (true, false), (true, true)];

        for (alice_input, bob_input) in test_cases {
            let z = protocol.run_protocol(alice_input, bob_input);
            assert_eq!(z, alice_input ^ bob_input ^ true,);
        }
    }
}
