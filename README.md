# Morse Code Parser

**Morse Code Parser** — це бібліотека та CLI-інструмент для парсингу, валідації та декодування тексту, закодованого у код Морзе. Реалізовано на мові програмування Rust із використанням граматики [Pest](https://pest.rs).

## 📜 Опис проєкту

Проєкт демонструє створення граматик для парсингу текстових форматів та реалізацію повного циклу обробки даних:
- **Парсинг коду Морзе**: Перетворення закодованого тексту у внутрішнє представлення.
- **Декодування**: Перетворення закодованого тексту у зрозумілий текст (літери A-Z, цифри 0-9).
- **Валідація**: Перевірка введення на наявність помилок або недопустимих символів.

## 🚀 Основні функції

1. **Парсинг**: Перевірка структури коду Морзе відповідно до граматики у `grammar.pest`.
2. **Декодування**: Конвертація закодованого тексту у зрозумілий текст.
3. **Валідація**: Аналіз введення для перевірки допустимості.

## 🛠️ Встановлення та запуск

### Клонування репозиторію
```bash
git clone https://github.com/eyexzy/morse_code_parser.git
cd morse_code_parser
```

### Збірка проєкту
```bash
cargo build
```

### Запуск CLI
```bash
cargo run -- "... --- ..."
```

### Тестування
```bash
cargo test
```

## 🔧 Приклади використання

### Парсинг та декодування
```rust
use morse_code_parser::{parse_morse_code, decode_morse, validate_input};

fn main() {
    let input = "... --- ... / - .... .. ... / .. ... / - . ... -";
    
    // Перевірка валідності
    if validate_input(input) {
        println!("Valid Morse code input!");
    }

    // Парсинг
    parse_morse_code(input).expect("Failed to parse Morse code!");

    // Декодування
    let decoded = decode_morse(input).expect("Failed to decode Morse code!");
    println!("Decoded text: {}", decoded);
}
```

### Приклад результату:
```
Valid Morse code input!
Parsed structure: ...
Decoded text: SOS THIS IS TEST
```

## 📜 Граматика коду Морзе

Файл `grammar.pest`:
```pest
WHITESPACE = _{ " " | "\t" | "\n" }
morse_char = { "." | "-" }
morse_letter = @{ morse_char+ }
morse_word = { morse_letter ~ (" " ~ morse_letter)* }
morse_code = { morse_word ~ (WHITESPACE* ~ "/" ~ WHITESPACE* ~ morse_word)* }
morse_code_with_spaces = { WHITESPACE* ~ morse_code ~ WHITESPACE* }
```

## 📊 Тестове покриття

Проєкт має понад **50 тестів**, що перевіряють:
- **Парсинг**: Вхідні дані різної структури.
- **Декодування**: Валідні, невідомі та некоректні символи.
- **Валідацію**: Допустимість введення.
- **Крайні випадки**: Пусті рядки, великі блоки тексту, невалідні дані.

Для запуску тестів:
```bash
cargo test
```

## 🔗 Посилання

- **Документація (docs.rs)**: [Посилання](https://docs.rs/morse_code_parser)
- **Crates.io**: [Посилання](https://crates.io/crates/morse_code_parser)
- **GitHub Repository**: [Посилання](https://github.com/eyexzy/morse_code_parser)

## 📝 Ліцензія

Проєкт доступний під ліцензією MIT. Деталі можна знайти у файлі [LICENSE](LICENSE).
