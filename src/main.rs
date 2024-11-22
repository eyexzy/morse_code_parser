use anyhow::Result;
use clap::{Arg, Command};
use morse_code_parser::{decode_morse, parse_morse_code, validate_input, MorseError};

fn main() -> Result<()> {
    let matches = Command::new("Morse Code Parser")
        .version("0.1.0")
        .author("Your Name <eyexzy@gmail.com>")
        .about("Parses and decodes Morse code")
        .arg(
            Arg::new("input")
                .help("The Morse code to decode")
                .required(false)
                .index(1),
        )
        .get_matches();

    let morse_input = matches
        .get_one::<String>("input")
        .map(|s| s.as_str())
        .unwrap_or("... --- ... / - .... .. ... / .. ... / - . ... -");

    if !validate_input(morse_input) {
        println!("Invalid Morse code input!");
        return Ok(());
    }

    println!("Parsing Morse code...");
    if let Err(e) = parse_morse_code(morse_input) {
        eprintln!("Error during parsing: {}", e);
        return Ok(());
    }

    println!("\nDecoding Morse code...");
    match decode_morse(morse_input) {
        Ok(decoded) => println!("Decoded text: {}", decoded),
        Err(MorseError::InvalidCharacter) => eprintln!("Error: Invalid character in Morse code."),
        Err(MorseError::EmptyInput) => eprintln!("Error: Input is empty."),
        Err(MorseError::ParseError(msg)) => eprintln!("Error: Failed to parse Morse code: {}", msg),
        Err(_) => eprintln!("Error: Unknown decoding error."),
    }

    Ok(())
}
