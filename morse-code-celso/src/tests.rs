use super::*;

#[test]
fn it_should_convert_to_morse_code() {
    let message_string = "ABC".to_string();
    // let expected_morse_code: Message = vec![vec![Pulse::Short]];
    let message_morse_code = message_string.to_morse_code();

    dbg!(&message_morse_code);

    print_morse_code(message_morse_code);
}
