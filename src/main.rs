use anyhow::Result;
use morse_code_parser::{parse_morse_code, decode_morse, validate_input};

fn main() -> Result<()> {
    let morse_input = "... --- ... / - .... .. ... / .. ... / - . ... -";

    // Перевірка введення
    if !validate_input(morse_input) {
        println!("Invalid Morse code input!");
        return Ok(());
    }

    // Парсинг коду Морзе
    println!("Parsing Morse code...");
    parse_morse_code(morse_input)?;

    // Декодування в текст
    println!("\nDecoding Morse code...");
    let decoded = decode_morse(morse_input)?;
    println!("Decoded text: {}", decoded);

    Ok(())
}
