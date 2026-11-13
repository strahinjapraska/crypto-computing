### Usage example 
#### OT
```rust 
    let mut rng = rng();
    let mut choice = rng.random_range(0..MAX_NR_MESSAGES);
    let mut  alice = Alice::new(choice);
    let messages: Vec<String> = (0..MAX_NR_MESSAGES).map(|i| i.to_string()).collect();
    let mut bob = Bob::new(messages.clone());

    let mut ot = OTProtocol::new(&mut alice, &mut bob);
    ot.run();
    assert_eq!(
      &alice.learnt_message.unwrap(),
      messages.get(choice as usize).unwrap()
    );
```
#### Garbled Circuits
```rust
    let x = Expr::Input(0);
    let y = Expr::input(1);

    let boolean_circuit = &x ^ &y;
    let alice_input = [0];
    let bob_input = [1];

    let mut alice = Evaluator::new(alice_input);
    let mut bob = Garbler::new(&boolean_circuit, bob_input);

    let mut protocol = YaoProtocol::new(&mut alice, &mut bob);
    protocol.run();

    assert_eq(alice.z.unwrap(), 1); 
```
### d-Homomorphic Encryption
Usage examples can be seen in `src/main.ts`, and in the test function in 
`src/protocols/homomorphic_encryption/protocol.rs`. The general usage of it
looks like this:
```rust
// generate a reasonably-sized key pair
let key_pair = generate_key_pair(1024, 2048, 32, 128, 16);
// initialize FHEencryption. This will be part of the party that encrypts the 
// input and decrypts the result (Alice)
let fhe = FHEncryption::new(key_pair.public_key, Some(key_pair.secret_key), rand::rng());
// setup the parties and their communication
let (a_outgoing, a_incoming) = channel();
let (b_outgoing, b_incoming) = channel();

let alice = Alice::new(
    Channel {
        incoming: a_incoming,
        outgoing: b_outgoing,
    },
    fhe,
);
let bob = Bob::new(Channel {
    incoming: b_incoming,
    outgoing: a_outgoing,
});
// set up the input
alice.set_blood_type(BloodType::ONeg);
bob.set_blood_type(BloodType::OPos);
// perform the communication
alice.send_encrypted_blood_bits();
// Bob evaluates the function with the encrypted bits from Alice
bob.eval();
// Alice recieves the encrypted result of eval, decrypts it and returns it.
let result = alice.yield_result();
assert_eq(result, true); 
```
Details about the chosen security parameters are discussed in 
`src/protocols/homomorphic_encryption/README.md`.
### Run demo 
You can also run the tests for OT by using
```bash
cargo test  
```

