use pest::Parser;
use pest_derive::Parser;
use anyhow::Result;
use std::collections::HashMap;

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct MorseCodeParser;

/// Функція для парсингу коду Морзе
pub fn parse_morse_code(input: &str) -> Result<()> {
    let parsed = MorseCodeParser::parse(Rule::morse_code, input)?;
    println!("Parsed structure:");
    for pair in parsed {
        println!("{:?}", pair);
    }
    Ok(())
}

/// Функція для декодування коду Морзе
pub fn decode_morse(input: &str) -> Result<String> {
    let morse_map: HashMap<&str, &str> = [
        (".-", "A"), ("-...", "B"), ("-.-.", "C"), ("-..", "D"), (".", "E"),
        ("..-.", "F"), ("--.", "G"), ("....", "H"), ("..", "I"), (".---", "J"),
        ("-.-", "K"), (".-..", "L"), ("--", "M"), ("-.", "N"), ("---", "O"),
        (".--.", "P"), ("--.-", "Q"), (".-.", "R"), ("...", "S"), ("-", "T"),
        ("..-", "U"), ("...-", "V"), (".--", "W"), ("-..-", "X"), ("-.--", "Y"),
        ("--..", "Z"), ("-----", "0"), (".----", "1"), ("..---", "2"),
        ("...--", "3"), ("....-", "4"), (".....", "5"), ("-....", "6"),
        ("--...", "7"), ("---..", "8"), ("----.", "9")
    ].iter().cloned().collect();

    let sanitized_input = input.trim().replace("   ", " ").replace("  ", " ");
    let words = sanitized_input.split(" / ");
    let mut decoded = String::new();

    for word in words {
        let mut decoded_word = String::new();

        for letter in word.split_whitespace() {
            if let Some(&decoded_letter) = morse_map.get(letter) {
                decoded_word.push_str(decoded_letter);
            } else if letter.chars().all(|c| c == '.' || c == '-') {
                decoded_word.push('?');
                decoded_word.push(' '); // Пробіл після невідомого символу
            }
        }

        decoded.push_str(&decoded_word.trim());
        decoded.push(' '); // Пробіл між словами
    }

    Ok(decoded.trim().to_string())
}

/// Утиліта для перевірки введення
pub fn validate_input(input: &str) -> bool {
    input.chars().all(|c| c == '.' || c == '-' || c == ' ' || c == '/')
}
