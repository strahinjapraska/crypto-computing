use crypto_computing::core::encoding::BloodType;
use crypto_computing::core::functionality::look_up_table_compatibility;
use crypto_computing::protocols::one_time_truth_tables::ottt::OtttProtocol;
fn main() {
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


}
