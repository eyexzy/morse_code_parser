use pest::Parser;
use pest_derive::Parser;
use std::collections::HashMap;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum MorseError {
    #[error("Invalid Morse code character")]
    InvalidCharacter,
    #[error("Empty input provided")]
    EmptyInput,
    #[error("Parsing error: {0}")]
    ParseError(String),
    #[error("Unknown decoding error")]
    Unknown,
}

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct MorseCodeParser;

pub fn parse_morse_code(input: &str) -> Result<(), MorseError> {
    let trimmed_input = input.trim();
    if trimmed_input.is_empty() {
        return Err(MorseError::EmptyInput);
    }

    MorseCodeParser::parse(Rule::morse_code_with_spaces, trimmed_input)
        .map_err(|e| MorseError::ParseError(e.to_string()))?;

    if trimmed_input.contains("//") {
        return Err(MorseError::InvalidCharacter);
    }

    Ok(())
}

pub fn decode_morse(input: &str) -> Result<String, MorseError> {
    if input.trim().is_empty() {
        return Err(MorseError::EmptyInput);
    }

    let morse_map: HashMap<&str, &str> = [
        (".-", "A"),
        ("-...", "B"),
        ("-.-.", "C"),
        ("-..", "D"),
        (".", "E"),
        ("..-.", "F"),
        ("--.", "G"),
        ("....", "H"),
        ("..", "I"),
        (".---", "J"),
        ("-.-", "K"),
        (".-..", "L"),
        ("--", "M"),
        ("-.", "N"),
        ("---", "O"),
        (".--.", "P"),
        ("--.-", "Q"),
        (".-.", "R"),
        ("...", "S"),
        ("-", "T"),
        ("..-", "U"),
        ("...-", "V"),
        (".--", "W"),
        ("-..-", "X"),
        ("-.--", "Y"),
        ("--..", "Z"),
        ("-----", "0"),
        (".----", "1"),
        ("..---", "2"),
        ("...--", "3"),
        ("....-", "4"),
        (".....", "5"),
        ("-....", "6"),
        ("--...", "7"),
        ("---..", "8"),
        ("----.", "9"),
    ]
    .iter()
    .cloned()
    .collect();

    let sanitized_input = input.trim().replace("   ", " / ").replace("  ", " ");
    let words = sanitized_input.split(" / ");
    let mut decoded = String::new();

    for word in words {
        let mut decoded_word = String::new();

        for letter in word.split_whitespace() {
            if let Some(&decoded_letter) = morse_map.get(letter) {
                decoded_word.push_str(decoded_letter);
            } else if letter.chars().all(|c| c == '.' || c == '-') {
                if !decoded_word.is_empty() {
                    decoded_word.push(' ');
                }
                decoded_word.push('?');
            } else {
                return Err(MorseError::InvalidCharacter);
            }
        }

        if !decoded_word.is_empty() {
            if !decoded.is_empty() {
                decoded.push(' ');
            }
            decoded.push_str(&decoded_word);
        }
    }

    Ok(decoded)
}

pub fn validate_input(input: &str) -> bool {
    !input.trim().is_empty()
        && input
            .chars()
            .all(|c| c == '.' || c == '-' || c == ' ' || c == '/')
}
