use crypto_computing::core::encoding::BloodType;
use crypto_computing::core::functionality::look_up_table_compatibility;
use crypto_computing::protocols::bedoza::bedoza::{BeDOZaProtocol, setup_parties};
use crypto_computing::protocols::bedoza::expressions::get_blood_compatibility_expressions;
use crypto_computing::protocols::one_time_truth_tables::ottt::OtttProtocol;

use dialoguer::Select;

fn run_protocol(donor: BloodType, recipient: BloodType){

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

fn select_protocol() -> String{
    let options = &["One-Time Truth Tables (OTTT)", "BeDoZa"]; 
    let selected_index = Select::new()
        .with_prompt("Select a protocol")
        .items(options)
        .default(0)
        .interact()
        .unwrap();
    options[selected_index].to_string()
}

fn select_input(party: &str) -> BloodType {
    let options = &["O-", "O+", "A-", "A+", "B-", "B+", "AB-", "AB+"];

    let selected_index = Select::new()
        .with_prompt(format!("Select {}'s blood type", party))
        .items(options)
        .default(0)
        .interact()
        .unwrap();

    let blood_type: BloodType = unsafe {
        std::mem::transmute(selected_index)
    };

    blood_type
}
fn main() {
   let alice_input = select_input("Alice"); 
   let bob_input = select_input("Bob");
   let protocol_name = select_protocol();

    let result = match protocol_name.as_str() {
        "One-Time Truth Tables (OTTT)" => {
            let mut protocol = OtttProtocol::new(&mut rand::rng()); 
            protocol.run_protocol(alice_input as usize, bob_input as usize)
        },
        "BeDoZa" => {
            let rng = rand::rng();
            let mut protocol = BeDOZaProtocol::new(get_blood_compatibility_expressions().to_vec(), rng); 
            let (mut alice, mut bob) = setup_parties();
            protocol.run_protocol(alice_input, bob_input, &mut alice, &mut bob)
        }
        ,
        _ => unreachable!(),
    }; 

    let lut_result = look_up_table_compatibility(alice_input, bob_input); 

    assert_eq!(result, lut_result); 

    if result {
        println!("The donor and recipient are compatible.");
    } else {
        println!("The donor and recipient are not compatible.");
    }

}
