use std::{
    collections::HashMap,
    sync::mpsc::{Receiver, Sender, channel},
};

use rand::{Rng, RngExt, rngs::ThreadRng};

use crate::{core::encoding::{BloodType, encode}, protocols::{
    bedoza::{dealer::Dealer, expressions::{Expression, ExpressionTypes::{self, XORWithConstant, XORWithTwoWires}, VariableNames::{self, TEMP_W}}}, common::Role,
}};

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

// state to temporarily store the shares required to perform the Multiplication subprotocol

pub struct Party<R: Rng> {
    pub role: Role,
    pub channel: Channel,
    pub state: HashMap<VariableNames, bool>,
    pub rng: R
}

impl<R: Rng> Party<R> {
    pub fn new(role: Role, channel: Channel, rng: R) -> Self {
        return Self {
            role,
            channel,
            state: HashMap::new(),
            rng
        };
    }

    pub fn set_value(&mut self, store_in_variable: VariableNames, value: bool) {
        self.state.insert(store_in_variable, value);
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

pub struct BeDOZaProtocol<R: Rng> {
    expression_list: Vec<Expression>,
    dealer: Dealer<R>
}

impl<R: Rng> BeDOZaProtocol<R> {
    pub fn new(expression_list: Vec<Expression>, rng: R) -> Self {
        Self { 
            expression_list, 
            dealer: Dealer::new(rng),
        }
    }

    pub fn run_AndWithTwoWires_subprotocol(&mut self, expression: Expression, alice: &mut Party<R>, bob: &mut Party<R>) {
        // 1. The dealer outputs a random triple [u], [v], [w] with w = u * v
                let (alice_shares, bob_shares) = self.dealer.query();
                
                // init the temporary shares of variables
                alice.set_value(VariableNames::TEMP_U, alice_shares.0);
                bob.set_value(VariableNames::TEMP_U, bob_shares.0);
                alice.set_value(VariableNames::TEMP_V, alice_shares.1);
                bob.set_value(VariableNames::TEMP_V, bob_shares.1);
                alice.set_value(VariableNames::TEMP_W, alice_shares.2);
                bob.set_value(VariableNames::TEMP_W, bob_shares.2);

                // 2. Run subprotocol: [d] = [x] + [u]
                self.dispatch_evaluate_expression(Expression { 
                    expression_type: ExpressionTypes::XORWithTwoWires, 
                    output_variable_name: VariableNames::TEMP_D, 
                    first_input_variable_name: expression.first_input_variable_name, 
                    second_input_variable_name: Some(VariableNames::TEMP_U), 
                    constant: None 
                }, alice, bob);
                
                // 3. Run subprotocol: [e] = [y] + [v]
                self.dispatch_evaluate_expression(Expression { 
                    expression_type: ExpressionTypes::XORWithTwoWires, 
                    output_variable_name: VariableNames::TEMP_E, 
                    first_input_variable_name: expression.second_input_variable_name.unwrap(), 
                    second_input_variable_name: Some(VariableNames::TEMP_V), 
                    constant: None 
                }, alice, bob);

                // 4. Run subprotocol: d <- Open([d])
                let _ = bob.reconstruct_output(VariableNames::TEMP_D);
                let d = alice.reconstruct_output(VariableNames::TEMP_D);
                
                // 5. Run subprotocol: e <- Open([e])
                let _ = bob.reconstruct_output(VariableNames::TEMP_E);
                let e = alice.reconstruct_output(VariableNames::TEMP_E);
                
                // 6. Run subprotocol: [z] = [w] + e * [x] + d * [y] + e * d
                //// 6.1. [e] := e * [x]
                self.dispatch_evaluate_expression(Expression { 
                    expression_type: ExpressionTypes::ANDWithConstant, 
                    output_variable_name: VariableNames::TEMP_E, 
                    first_input_variable_name: expression.first_input_variable_name, 
                    second_input_variable_name: None, 
                    constant: e
                }, alice, bob);

                //// 6.2. [d] := d * [y]
                self.dispatch_evaluate_expression(Expression { 
                    expression_type: ExpressionTypes::ANDWithConstant, 
                    output_variable_name: VariableNames::TEMP_D, 
                    first_input_variable_name: expression.second_input_variable_name.unwrap(), 
                    second_input_variable_name: None, 
                    constant: d
                }, alice, bob);
                
                //// 6.3. [w] := [w] + [e]
                self.dispatch_evaluate_expression(Expression { 
                    expression_type: ExpressionTypes::XORWithTwoWires, 
                    output_variable_name: VariableNames::TEMP_W, 
                    first_input_variable_name: VariableNames::TEMP_W, 
                    second_input_variable_name: Some(VariableNames::TEMP_E), 
                    constant: None
                }, alice, bob);

                //// 6.4. [w] = [w] + [d]
                self.dispatch_evaluate_expression(Expression { 
                    expression_type: XORWithTwoWires, 
                    output_variable_name: VariableNames::TEMP_W, 
                    first_input_variable_name: TEMP_W, 
                    second_input_variable_name: Some(VariableNames::TEMP_D), 
                    constant: None }, 
                    alice, bob);

                //// 6.5. [z] = [w] + e * d
                self.dispatch_evaluate_expression(
                    Expression { expression_type: XORWithConstant, 
                        output_variable_name: expression.output_variable_name, 
                        first_input_variable_name: TEMP_W, 
                        second_input_variable_name: None, 
                        constant: Some(e.unwrap() && d.unwrap()) }, alice, bob);

    }

    pub fn parse_and_share_initial_input(alice_blood_type: BloodType, bob_blood_type: BloodType, alice: &mut Party<R>, bob: &mut Party<R>) {
        let alice_input = encode(alice_blood_type);
        let bob_input = encode(bob_blood_type);
        alice.share_input(VariableNames::DONOR_A, Some(alice_input.a));
        // force Bob to receive the share from Alice
        bob.share_input(VariableNames::DONOR_A, None);

        bob.share_input(VariableNames::RECIPIENT_A, Some(bob_input.a));
        // force Alice to receive the share from Bob
        alice.share_input(VariableNames::RECIPIENT_A, None);

        alice.share_input(VariableNames::DONOR_B, Some(alice_input.b));
        // force Bob to receive the share from Alice
        bob.share_input(VariableNames::DONOR_B, None);

        bob.share_input(VariableNames::RECIPIENT_B, Some(bob_input.b));
        // force Alice to receive the share from Bob
        alice.share_input(VariableNames::RECIPIENT_B, None);

        alice.share_input(VariableNames::DONOR_RH, Some(alice_input.rh));
        // force Bob to receive the share from Alice
        bob.share_input(VariableNames::DONOR_RH, None);

        bob.share_input(VariableNames::RECIPIENT_RH, Some(bob_input.rh));
        // force Alice to receive the share from Bob
        alice.share_input(VariableNames::RECIPIENT_RH, None);
    }

    pub fn dispatch_evaluate_expression(&self, expression: Expression, alice: &mut Party<R>, bob: &mut Party<R>) {
        alice.evaluate_expression(expression.clone());
        bob.evaluate_expression(expression.clone());
    }

    pub fn run_protocol(&mut self, alice_blood_type: BloodType, bob_blood_type: BloodType, alice: &mut Party<R>, bob: &mut Party<R>) -> bool {
        
        Self::parse_and_share_initial_input(alice_blood_type, bob_blood_type, alice, bob);

        println!("{:=>60}", "");

        println!("initial states");
        println!("Alice state: {:?}", alice.state);
        println!("Bob state: {:?}", bob.state);
        println!("{:=>60}", "");

        self.expression_list.clone().iter().for_each(|expression| {
            if expression.expression_type == ExpressionTypes::ANDWithTwoWires {
                self.run_AndWithTwoWires_subprotocol(expression.clone(), alice, bob);
            } else {
                self.dispatch_evaluate_expression(expression.clone(), alice, bob);

                println!("{:?}", expression);
                println!("Alice state: {:?}", alice.state);
                println!("Bob state: {:?}", bob.state);
                println!("{:=>60}", "");
            }
        });

        let last_updated_variable = self.expression_list.last().unwrap().output_variable_name;

        let _ = bob.reconstruct_output(last_updated_variable);
        let z = alice.reconstruct_output(last_updated_variable).unwrap();

        z
    }
}

pub fn setup_parties() -> (Party<ThreadRng>, Party<ThreadRng>) {
    let alice_rng = rand::rng();
        let bob_rng = rand::rng();

        let (a_outgoing, a_incoming) = channel();
        let (b_outgoing, b_incoming) = channel();

        let alice = Party::new(
            Role::Alice,
            Channel {
                incoming: a_incoming,
                outgoing: b_outgoing,
            },
            alice_rng
        );
        let bob = Party::new(
            Role::Bob,
            Channel {
                incoming: b_incoming,
                outgoing: a_outgoing,
            },
            bob_rng
        );
        return (alice, bob);
}

#[cfg(test)]
mod test {
    use crate::{core::functionality::{BLOOD_TYPES, circuit_compatiblity}, protocols::{bedoza::{bedoza::setup_parties, expressions::{get_blood_compatibility_expressions, get_local_running_expressions}}}};

    #[test]
    fn test_bedoza_protocol() {
        let rng = rand::rng();
        

        let mut protocol = super::BeDOZaProtocol::new(
            get_blood_compatibility_expressions().to_vec(), 
            rng);

        BLOOD_TYPES.iter().for_each(|alice_blood_type| {
            BLOOD_TYPES.iter().for_each(|bob_blood_type| {
                let (mut alice, mut bob) = setup_parties();
                let z = protocol.run_protocol(*alice_blood_type, *bob_blood_type, &mut alice, &mut bob);

                let circuit_result = circuit_compatiblity(*alice_blood_type, *bob_blood_type);
                assert_eq!(z, circuit_result, "mismatch for donor: {:?}, recipient: {:?}", alice_blood_type, bob_blood_type);
            });
        });
    }
}
