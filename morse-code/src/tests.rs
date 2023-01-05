use super::*;

#[test]
fn whole_alphabet() {
    // let alphabet = "abcdefghijklmnopqrstuvxz1234567890".to_string();

    // println!("{:?}", alphabet.to_morse_code());
    // println!("{:?}", alphabet.to_uppercase().to_morse_code());

    let invite = "a0A0b0B0".to_string();
    let message = invite.to_morse_code();
    println!("coded invite: {:?}", message);

    print_morse_code(message);
}
