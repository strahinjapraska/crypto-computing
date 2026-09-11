### Usage example 

```rust 
    let donor = BloodType::APos; 
    let recipient: BloodType = BloodType::BPos; 

    let alice_input = recipient as usize; 
    let bob_input = donor as usize; 

    let mut rng = rand::rng(); 
    let mut ottt_protocol = OtttProtocol::new(&mut rng);   

    let ottt_result = ottt_protocol.run_protocol(alice_input, bob_input); 

    let lut_result = look_up_table_compatibility(donor, recipient); 

    assert_eq!(ottt_result, lut_result); 

    if ottt_result {
        println!("The donor and recipient are compatible.");
    } else {
        println!("The donor and recipient are not compatible.");
    }
```

### Run demo 
You can also run a demo script for our OTTT implementation for blood compatibility by using 
```bash
cargo run  
```

