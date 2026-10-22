use crate::hangman::result::GuessResult;

#[derive(Debug)]
pub struct Word {
    word: Vec<char>,
    word_hidden: Vec<char>,
    num_letters_guessed: usize,
}

impl Word {
    pub fn new(word: Vec<char>) -> Self {
        let word_len = word.len();

        Self {
            word,
            word_hidden: vec!['-'; word_len],
            num_letters_guessed: 0,
        }
    }

    pub fn update_word_hidden(&mut self, char: char) -> GuessResult {
        if self.is_winner() {
            return GuessResult::CorrectWord
        }

        let mut state = GuessResult::WrongLetter;

        self.word.iter()
            .enumerate()
            .for_each(|(i, &c)| {
                if c == char {
                    if self.word_hidden[i] == c {
                        state = GuessResult::LetterAlreadyGuessed
                    } else {
                        self.num_letters_guessed += 1;
                        self.word_hidden[i] = c;
                        state = GuessResult::CorrectLetter
                    }
                }
            });

        state
    }

    pub fn is_winner(&self) -> bool {
        self.num_letters_guessed == self.word.len()
    }
    
    pub fn get_word_hidden(&self) -> String {
        self.word_hidden.iter().collect()
    }

    pub fn num_letters(&self) -> usize {
        self.word.len()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    fn create_word(word: &str) -> Word {
        Word::new(word.chars().collect())
    }

    #[test]
    fn new_should_initialize_word() {
        let word = create_word("rust");

        assert_eq!(word.num_letters(), 4);
        assert_eq!(word.get_word_hidden(), "----");
        assert!(!word.is_winner());
    }

    #[test]
    fn new_should_support_cyrillic_letters() {
        let word = create_word("привет");

        assert_eq!(word.num_letters(), 6);
        assert_eq!(word.get_word_hidden(), "------");
        assert!(!word.is_winner());
    }

    #[test]
    fn new_with_empty_word_is_immediately_won() {
        let word = Word::new(Vec::new());

        assert_eq!(word.num_letters(), 0);
        assert_eq!(word.get_word_hidden(), "");
        assert!(word.is_winner());
    }

    #[test]
    fn update_should_reveal_correct_letter() {
        let mut word = create_word("rust");

        let result = word.update_word_hidden('r');

        assert_eq!(result, GuessResult::CorrectLetter);
        assert_eq!(word.get_word_hidden(), "r---");
        assert!(!word.is_winner());
    }

    #[test]
    fn update_should_reveal_all_repeated_letters() {
        let mut word = create_word("мама");

        let result = word.update_word_hidden('м');

        assert_eq!(result, GuessResult::CorrectLetter);
        assert_eq!(word.get_word_hidden(), "м-м-");
        assert!(!word.is_winner());
    }

    #[test]
    fn update_should_return_wrong_letter() {
        let mut word = create_word("rust");

        let result = word.update_word_hidden('x');

        assert_eq!(result, GuessResult::WrongLetter);
        assert_eq!(word.get_word_hidden(), "----");
        assert!(!word.is_winner());
    }

    #[test]
    fn update_should_detect_already_guessed_letter() {
        let mut word = create_word("rust");

        word.update_word_hidden('r');
        let result = word.update_word_hidden('r');

        assert_eq!(result, GuessResult::LetterAlreadyGuessed);
        assert_eq!(word.get_word_hidden(), "r---");
        assert!(!word.is_winner());
    }

    #[test]
    fn update_should_return_wrong_for_repeated_wrong_letter() {
        let mut word = create_word("rust");

        let first = word.update_word_hidden('x');
        let second = word.update_word_hidden('x');

        assert_eq!(first, GuessResult::WrongLetter);
        assert_eq!(second, GuessResult::WrongLetter);
        assert_eq!(word.get_word_hidden(), "----");
    }

    #[test]
    fn update_should_not_erase_previously_guessed_letters() {
        let mut word = create_word("rust");

        word.update_word_hidden('r');
        word.update_word_hidden('s');
        word.update_word_hidden('x');

        assert_eq!(word.get_word_hidden(), "r-s-");
        assert!(!word.is_winner());
    }

    #[test]
    fn update_should_win_after_last_correct_letter() {
        let mut word = create_word("кот");

        word.update_word_hidden('к');
        word.update_word_hidden('о');

        assert!(!word.is_winner());

        let result = word.update_word_hidden('т');

        assert_eq!(result, GuessResult::CorrectLetter);
        assert_eq!(word.get_word_hidden(), "кот");
        assert!(word.is_winner());
    }

    #[test]
    fn update_should_win_when_all_letters_are_identical() {
        let mut word = create_word("ааа");

        let result = word.update_word_hidden('а');

        assert_eq!(result, GuessResult::CorrectLetter);
        assert_eq!(word.get_word_hidden(), "ааа");
        assert!(word.is_winner());
    }

    #[test]
    fn update_after_win_should_return_correct_word() {
        let mut word = create_word("кот");

        word.update_word_hidden('к');
        word.update_word_hidden('о');
        word.update_word_hidden('т');

        assert!(word.is_winner());

        let result = word.update_word_hidden('x');

        assert_eq!(result, GuessResult::CorrectWord);
        assert_eq!(word.get_word_hidden(), "кот");
        assert!(word.is_winner());
    }

    #[test]
    fn is_winner_should_be_false_until_all_letters_guessed() {
        let mut word = create_word("abc");

        assert!(!word.is_winner());

        word.update_word_hidden('a');
        assert!(!word.is_winner());

        word.update_word_hidden('b');
        assert!(!word.is_winner());

        word.update_word_hidden('c');
        assert!(word.is_winner());
    }

    #[test]
    fn get_word_hidden_should_reflect_current_progress() {
        let mut word = create_word("abc");

        assert_eq!(word.get_word_hidden(), "---");

        word.update_word_hidden('b');
        assert_eq!(word.get_word_hidden(), "-b-");

        word.update_word_hidden('a');
        assert_eq!(word.get_word_hidden(), "ab-");

        word.update_word_hidden('c');
        assert_eq!(word.get_word_hidden(), "abc");
    }

    #[test]
    fn num_letters_should_not_change_after_guesses() {
        let mut word = create_word("мама");

        assert_eq!(word.num_letters(), 4);

        word.update_word_hidden('м');
        word.update_word_hidden('а');

        assert_eq!(word.num_letters(), 4);
    }
}
