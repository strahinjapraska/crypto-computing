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
### Run demo 
You can also run the tests for OT by using
```bash
cargo test  
```

