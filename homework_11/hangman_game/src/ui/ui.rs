use std::io::{stdout, Write, stdin, BufRead, Result};

const DEFAULT_WORD_LENGTH: i32 = 4;

pub fn print_greeting() {
    println!("Игра 'Виселица'");
    print!("Введите длину слова. По умолчанию длина {}: ", DEFAULT_WORD_LENGTH);
    stdout().flush().unwrap();
}

pub fn info_print(num_of_attempts: usize) {
    println!("Слово английское. {} попытки", num_of_attempts);
}

pub fn get_word_length() -> i32 {
    read_word_length(&mut stdin().lock())
        .expect("Failed to read word length")
}

fn read_word_length<R: BufRead>(reader: &mut R) -> Result<i32> {
    let mut input = String::new();

    reader.read_line(&mut input)?;

    let num = input
        .trim()
        .parse::<i32>()
        .unwrap_or(DEFAULT_WORD_LENGTH);

    Ok(num.max(DEFAULT_WORD_LENGTH))
}

pub fn get_letter() -> char {
    read_letter(&mut stdin().lock())
        .expect("Failed to read letter")
}

fn read_letter<R: BufRead>(reader: &mut R) -> Result<char> {
    let mut input = String::new();

    reader.read_line(&mut input)?;

    Ok(input.trim().chars().next().unwrap_or(' '))
}

pub fn enter_letter_print() {
    print!("Введите букву => ");
    stdout().flush().unwrap();
}

pub fn word_hidden_print(word: String) {
    println!("\nСлово: {word}");
}

pub fn num_of_attempts_print(num_of_attempts: usize) {
    println!("Осталось попыток {num_of_attempts}");
}

pub fn guessed_print() {
    println!("Есть такая буква!")
}

pub fn letter_already_guessed_print() {
    println!("Данная буква уже угадана...")
}

pub fn wrong_letter_print() {
    println!("Такой буквы в слове нет.")
}

pub fn won_print(guessed_word: &str) {
    println!("\nПоздравляем! 🎊🎉🎊\nВы угадали слово - {}", guessed_word);
}

pub fn game_over_print() {
    println!("\nПровал 😢\nПопробуйте в следующий раз...");
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn should_parse_valid_word_length() {
        let mut input = Cursor::new(b"12\n");

        let result = read_word_length(&mut input).unwrap();

        assert_eq!(result, 12.max(DEFAULT_WORD_LENGTH));
    }

    #[test]
    fn should_use_default_for_invalid_input() {
        let mut input = Cursor::new(b"abc\n");

        let result = read_word_length(&mut input).unwrap();

        assert_eq!(result, DEFAULT_WORD_LENGTH);
    }

    #[test]
    fn should_use_minimum_length() {
        let mut input = Cursor::new(b"1\n");

        let result = read_word_length(&mut input).unwrap();

        assert_eq!(result, 1.max(DEFAULT_WORD_LENGTH));
    }

    #[test]
    fn should_handle_empty_input() {
        let mut input = Cursor::new(b"\n");

        let result = read_word_length(&mut input).unwrap();

        assert_eq!(result, DEFAULT_WORD_LENGTH);
    }

    #[test]
    fn should_trim_whitespace() {
        let mut input = Cursor::new(b"  12  \n");

        let result = read_word_length(&mut input).unwrap();

        assert_eq!(result, 12.max(DEFAULT_WORD_LENGTH));
    }

    #[test]
    fn should_handle_non_numeric_overflow() {
        let mut input = Cursor::new(
            b"999999999999999999999999999999999\n"
        );

        let result = read_word_length(&mut input).unwrap();

        assert_eq!(result, DEFAULT_WORD_LENGTH);
    }

    #[test]
    fn should_read_english_letter() {
        let mut input = Cursor::new("a\n");

        assert_eq!(read_letter(&mut input).unwrap(), 'a');
    }

    #[test]
    fn should_read_russian_letter() {
        let mut input = Cursor::new("я\n");

        assert_eq!(read_letter(&mut input).unwrap(), 'я');
    }

    #[test]
    fn should_trim_input() {
        let mut input = Cursor::new("  x  \n");

        assert_eq!(read_letter(&mut input).unwrap(), 'x');
    }

    #[test]
    fn should_return_space_for_empty_line() {
        let mut input = Cursor::new("\n");

        assert_eq!(read_letter(&mut input).unwrap(), ' ');
    }

    #[test]
    fn should_return_first_char() {
        let mut input = Cursor::new("hello\n");

        assert_eq!(read_letter(&mut input).unwrap(), 'h');
    }

    #[test]
    fn should_return_space_on_eof() {
        let mut input = Cursor::new("");

        assert_eq!(read_letter(&mut input).unwrap(), ' ');
    }
}