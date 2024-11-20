use morse_code_parser::{parse_morse_code, decode_morse, validate_input};
use anyhow::Result;

// Тестуємо валідність введення
#[test]
fn test_valid_input_simple() {
    let input = "... --- ...";
    assert!(validate_input(input));
}

#[test]
fn test_valid_input_with_spaces() {
    let input = "    ... --- ...    ";
    assert!(validate_input(input));
}

#[test]
fn test_invalid_input_with_letters() {
    let input = "... --- ... INVALID";
    assert!(!validate_input(input));
}

#[test]
fn test_invalid_input_with_numbers() {
    let input = "... --- ... 123";
    assert!(!validate_input(input));
}

#[test]
fn test_invalid_input_with_special_characters() {
    let input = "... --- ... @#$%";
    assert!(!validate_input(input));
}

#[test]
fn test_empty_input_is_valid() {
    let input = "";
    assert!(validate_input(input));
}

// Тестуємо парсинг
#[test]
fn test_parse_valid_morse_code() {
    let input = "... --- ... / - .... .. ...";
    assert!(parse_morse_code(input).is_ok());
}

#[test]
fn test_parse_with_extra_spaces() {
    let input = "...   ---   ... / - .... .. ...";
    assert!(parse_morse_code(input).is_ok());
}

#[test]
fn test_parse_invalid_structure() {
    let input = "... / --- ---";
    assert!(parse_morse_code(input).is_ok());
}

// Тестуємо декодування
#[test]
fn test_decode_simple_word() {
    let input = "... --- ...";
    let decoded = decode_morse(input).unwrap();
    assert_eq!(decoded, "SOS");
}

#[test]
fn test_decode_multiple_words() {
    let input = "... --- ... / - .... .. ...";
    let decoded = decode_morse(input).unwrap();
    assert_eq!(decoded, "SOS THIS");
}

#[test]
fn test_decode_with_numbers() {
    let input = ".---- ..--- ...-- / ....- ..... -....";
    let decoded = decode_morse(input).unwrap();
    assert_eq!(decoded, "123 456");
}

#[test]
fn test_decode_with_unknown_symbols() {
    let input = "... --- ... / ..--.. / - .... .. ...";
    let decoded = decode_morse(input).unwrap();
    assert_eq!(decoded, "SOS ? THIS");
}

#[test]
fn test_decode_empty_input() {
    let input = "";
    let decoded = decode_morse(input).unwrap();
    assert_eq!(decoded, "");
}

#[test]
fn test_decode_long_sentence() {
    let input = "- .... .. ... / .. ... / .- / ... .. -- .--. .-.. . / ... . -. - . -. -.-. .";
    let decoded = decode_morse(input).unwrap();
    assert_eq!(decoded, "THIS IS A SIMPLE SENTENCE");
}

#[test]
fn test_decode_with_extra_spaces() {
    let input = "...   ---   ... /   - .... .. ...";
    let decoded = decode_morse(input).unwrap();
    assert_eq!(decoded, "SOS THIS");
}

#[test]
fn test_decode_with_large_input() {
    let input = "... --- ... / ".repeat(1000);
    let decoded = decode_morse(&input).unwrap();
    assert!(decoded.contains("SOS"));
}

#[test]
fn test_decode_with_invalid_characters() {
    let input = "... --- ... !!!";
    let decoded = decode_morse(input).unwrap();
    assert_eq!(decoded, "SOS");
}

#[test]
fn test_decode_special_case() {
    let input = ".-- .... .- - / .. ... / - .... .. ... / -.-. --- -.. .";
    let decoded = decode_morse(input).unwrap();
    assert_eq!(decoded, "WHAT IS THIS CODE");
}

#[test]
fn test_decode_mixed_numbers_and_letters() {
    let input = ".- .---- -... ..--- -.-. ...--";
    let decoded = decode_morse(input).unwrap();
    assert_eq!(decoded, "A1B2C3");
}

// Тестуємо крайні випадки
#[test]
fn test_decode_only_unknown_symbols() {
    let input = "..--.. ..--..";
    let decoded = decode_morse(input).unwrap();
    assert_eq!(decoded, "? ?");
}

#[test]
fn test_decode_only_spaces() {
    let input = "    ";
    let decoded = decode_morse(input).unwrap();
    assert_eq!(decoded, "");
}

#[test]
fn test_decode_repeated_unknown_symbols() {
    let input = "..--.. / ..--.. / ..--..";
    let decoded = decode_morse(input).unwrap();
    assert_eq!(decoded, "? ? ?");
}

#[test]
fn test_decode_with_no_valid_symbols() {
    let input = "!!! ??? @@@ ###";
    let decoded = decode_morse(input).unwrap();
    assert_eq!(decoded, "");
}

#[test]
fn test_decode_complex_sentence() {
    let input = ".-- .... .- - / .- / -... . .- ..- - .. ..-. ..- .-.. / -.. .- -.--";
    let decoded = decode_morse(input).unwrap();
    assert_eq!(decoded, "WHAT A BEAUTIFUL DAY");
}

#[test]
fn test_decode_real_world_example() {
    let input = ".-.. --- ...- . / -.-- --- ..- / - --- / - .... . / -- --- --- -. / .- -. -.. / -... .- -.-. -.-";
    let decoded = decode_morse(input).unwrap();
    assert_eq!(decoded, "LOVE YOU TO THE MOON AND BACK");
}

// Тестуємо перевірку вводу
#[test]
fn test_validate_simple_input() {
    let input = "... --- ...";
    assert!(validate_input(input));
}

#[test]
fn test_validate_empty_input() {
    let input = "";
    assert!(validate_input(input));
}

#[test]
fn test_validate_only_spaces() {
    let input = "    ";
    assert!(validate_input(input));
}

#[test]
fn test_validate_invalid_input_with_special_characters() {
    let input = "... --- ... @#$%";
    assert!(!validate_input(input));
}

#[test]
fn test_validate_mixed_valid_and_invalid_input() {
    let input = "... --- ... INVALID";
    assert!(!validate_input(input));
}

#[test]
fn test_validate_with_extra_spaces() {
    let input = "   ...   ---   ...   ";
    assert!(validate_input(input));
}

#[test]
fn test_validate_large_input() {
    let input = "... --- ... / ".repeat(1000);
    assert!(validate_input(&input));
}

#[test]
fn test_validate_real_world_example() {
    let input = ".-.. --- ...- . / -.-- --- ..- / - --- / - .... . / -- --- --- -. / .- -. -.. / -... .- -.-. -.-";
    assert!(validate_input(input));
}
