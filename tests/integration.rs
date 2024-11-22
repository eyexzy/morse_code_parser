use morse_code_parser::{decode_morse, parse_morse_code, validate_input};

#[test]
fn test_parse_valid_morse_code() {
    assert!(parse_morse_code("... --- ...").is_ok());
}

#[test]
fn test_parse_invalid_morse_code() {
    assert!(parse_morse_code("invalid").is_err());
}

#[test]
fn test_decode_valid_input() {
    let result = decode_morse("... --- ...");
    assert_eq!(result.unwrap(), "SOS");
}

#[test]
fn test_decode_invalid_character() {
    let result = decode_morse("... @ ...");
    assert!(result.is_err());
}

#[test]
fn test_validate_valid_input() {
    assert!(validate_input("... --- ..."));
}

#[test]
fn test_validate_invalid_input() {
    assert!(!validate_input("invalid"));
}

#[test]
fn test_decode_simple_letter() {
    let result = decode_morse(".-");
    assert_eq!(result.unwrap(), "A");
}

#[test]
fn test_decode_multiple_letters() {
    let result = decode_morse(".- -...");
    assert_eq!(result.unwrap(), "AB");
}

#[test]
fn test_decode_with_spaces() {
    let result = decode_morse(".-   -...").unwrap();
    assert_eq!(result, "A B");
}

#[test]
fn test_decode_with_numbers() {
    let result = decode_morse("..--- ----- ...--");
    assert_eq!(result.unwrap(), "203");
}

#[test]
fn test_parse_with_extra_spaces() {
    assert!(parse_morse_code("  ... --- ...  ").is_ok());
}

#[test]
fn test_decode_empty_input() {
    let result = decode_morse("");
    assert!(result.is_err());
}

#[test]
fn test_parse_empty_input() {
    let result = parse_morse_code("");
    assert!(result.is_err());
}

#[test]
fn test_decode_with_unknown_symbol() {
    let result = decode_morse(".- ..@ ..-");
    assert!(result.is_err());
}

#[test]
fn test_decode_sentence() {
    let result = decode_morse("... --- ... / - .... .. ... / .. ... / - . ... -");
    assert_eq!(result.unwrap(), "SOS THIS IS TEST");
}

#[test]
fn test_validate_with_slashes() {
    assert!(validate_input("... --- ... / ... --- ..."));
}

#[test]
fn test_decode_special_case_1() {
    let result = decode_morse("---... ---... ---...").unwrap();
    assert_eq!(result, "? ? ?");
}

#[test]
fn test_decode_with_only_unknown_symbols() {
    let result = decode_morse("@@@");
    assert!(result.is_err());
}

#[test]
fn test_decode_with_mixed_letters_and_numbers() {
    let result = decode_morse(".- ..--- -...");
    assert_eq!(result.unwrap(), "A2B");
}

#[test]
fn test_decode_with_trailing_spaces() {
    let result = decode_morse(".- -... ");
    assert_eq!(result.unwrap(), "AB");
}

#[test]
fn test_decode_with_leading_spaces() {
    let result = decode_morse("  .- -...");
    assert_eq!(result.unwrap(), "AB");
}

#[test]
fn test_parse_with_leading_and_trailing_spaces() {
    assert!(parse_morse_code("  ... --- ...  ").is_ok());
}

#[test]
fn test_validate_empty_string() {
    assert!(!validate_input(""));
}

#[test]
fn test_decode_with_single_word() {
    let result = decode_morse(".... . .-.. .-.. ---");
    assert_eq!(result.unwrap(), "HELLO");
}

#[test]
fn test_decode_with_multiple_words() {
    let result = decode_morse(".... . .-.. .-.. --- / .-- --- .-. .-.. -..");
    assert_eq!(result.unwrap(), "HELLO WORLD");
}

#[test]
fn test_decode_with_unknown_and_valid_symbols() {
    let result = decode_morse(".... . .-.. .-.. --- / @@@");
    assert!(result.is_err());
}

#[test]
fn test_parse_with_slashes() {
    assert!(parse_morse_code("... / ... / ...").is_ok());
}

#[test]
fn test_decode_with_mixed_valid_and_invalid_symbols() {
    let result = decode_morse(".- / .@");
    assert!(result.is_err());
}

#[test]
fn test_validate_with_invalid_characters() {
    assert!(!validate_input("... --- ... @@@"));
}

#[test]
fn test_validate_with_valid_and_invalid_characters() {
    assert!(!validate_input("... --- ... / ... @@@"));
}

#[test]
fn test_decode_all_valid_numbers() {
    let result = decode_morse("----- .---- ..--- ...-- ....- ..... -.... --... ---.. ----.");
    assert_eq!(result.unwrap(), "0123456789");
}

#[test]
fn test_decode_repeated_unknown_symbols() {
    let result = decode_morse(".... @@@ ....");
    assert!(result.is_err());
}

#[test]
fn test_decode_long_sentence() {
    let result = decode_morse(".... . .-.. .-.. --- / .. / .- -- / - . ... - .. -. --.");
    assert_eq!(result.unwrap(), "HELLO I AM TESTING");
}

#[test]
fn test_parse_large_input() {
    let large_input = "... --- ... ".repeat(1000);
    assert!(parse_morse_code(&large_input).is_ok());
}

#[test]
fn test_decode_with_all_letters() {
    let input = ".- -... -.-. -.. . ..-. --. .... .. .--- -.- .-.. -- -.".to_string()
        + " --- .--. --.- .-. ... - ..- ...- .-- -..- -.-- --..";
    let result = decode_morse(&input);
    assert_eq!(result.unwrap(), "ABCDEFGHIJKLMNOPQRSTUVWXYZ");
}

#[test]
fn test_validate_large_input() {
    let large_input = "... --- ... / ... --- ... ".repeat(500);
    assert!(validate_input(&large_input));
}

#[test]
fn test_decode_mixed_case_handling() {
    let result = decode_morse(".... . .-.. .-.. --- / .-- --- .-. .-.. -..");
    assert_eq!(result.unwrap(), "HELLO WORLD");
}

#[test]
fn test_decode_with_extra_spaces_between_letters() {
    let result = decode_morse("....  .  .-..  .-..  ---");
    assert_eq!(result.unwrap(), "HELLO");
}

#[test]
fn test_decode_with_double_slash() {
    let result = decode_morse(".... . .-.. .-.. --- // .-- --- .-. .-.. -..");
    assert!(result.is_err());
}

#[test]
fn test_parse_with_large_input() {
    let input = "... --- ... / ... --- ... ".repeat(200);
    assert!(parse_morse_code(&input).is_ok());
}

#[test]
fn test_decode_sentence_with_numbers() {
    let result = decode_morse(".... . .-.. .-.. --- / .---- ..--- ...--");
    assert_eq!(result.unwrap(), "HELLO 123");
}

#[test]
fn test_validate_with_mixed_valid_and_invalid_characters() {
    assert!(!validate_input(".... . .-.. .-.. --- / ... --- @@@"));
}

#[test]
fn test_decode_mixed_valid_and_invalid_symbols() {
    let result = decode_morse(".... . .-.. .-.. --- / @@@");
    assert!(result.is_err());
}

#[test]
fn test_decode_complex_sentence() {
    let result = decode_morse(
        ".... . .-.. .-.. --- / .-- --- .-. .-.. -.. / .. ... / --. .-. . .- - / ..-. --- .-. / - . ... - .. -. --.",
    );
    assert_eq!(result.unwrap(), "HELLO WORLD IS GREAT FOR TESTING");
}

#[test]
fn test_decode_only_valid_symbols() {
    let result = decode_morse(".... . .-.. .-.. --- / .-- --- .-. .-.. -..");
    assert_eq!(result.unwrap(), "HELLO WORLD");
}

#[test]
fn test_validate_only_valid_characters() {
    assert!(validate_input(
        ".... . .-.. .-.. --- / .-- --- .-. .-.. -.."
    ));
}

#[test]
fn test_parse_with_invalid_structure() {
    assert!(parse_morse_code(".- / --..").is_ok());
}

#[test]
fn test_decode_single_valid_character() {
    let result = decode_morse(".");
    assert_eq!(result.unwrap(), "E");
}

#[test]
fn test_parse_valid_code_with_multiple_words() {
    let result = parse_morse_code("... --- ... / .- .- .-");
    assert!(result.is_ok());
}

#[test]
fn test_validate_valid_input_with_spaces_and_slashes() {
    assert!(validate_input("... --- ... / .- -... -.-."));
}
