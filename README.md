### Usage example 

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

### Run demo 
You can also run the tests for OT by using
```bash
cargo test  
```

