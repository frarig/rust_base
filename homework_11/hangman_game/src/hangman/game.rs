use crate::hangman::result::GuessResult;
pub(crate) use crate::hangman::state::GameState;
use crate::hangman::word::Word;
use crate::ui::ui::*;

#[derive(Debug)]
pub struct Game {
    word: Word,
    state: GameState,
    num_of_attempts: usize,
}

impl Game {
    pub fn new(word: Vec<char>) -> Self {
        Self {
            word: Word::new(word),
            state: GameState::InProgress,
            num_of_attempts: 3,
        }
    }

    pub fn num_of_attempts(&self) -> usize {
        self.num_of_attempts
    }

    pub fn resetting_num_of_attempts(&mut self) {
        if self.num_of_attempts > self.word.num_letters() {
            self.num_of_attempts = self.word.num_letters()
        }
    }

    pub fn state(&self) -> GameState {
        self.state
    }

    pub fn next_step(&mut self) {
        let letter = get_letter();
        let result = self.process_letter(letter);

        match result {
            GuessResult::CorrectLetter => guessed_print(),
            GuessResult::LetterAlreadyGuessed => letter_already_guessed_print(),
            GuessResult::WrongLetter => wrong_letter_print(),
            _ => ()
        }
    }

    fn process_letter(&mut self, letter: char) -> GuessResult {
        let result = self.word.update_word_hidden(letter);

        match result {
            GuessResult::CorrectWord => {
                self.state = GameState::Won
            },
            GuessResult::WrongWord => {
                self.state = GameState::Lost
            },
            GuessResult::CorrectLetter => {
                if self.word.is_winner() {
                    self.state = GameState::Won;
                }
            },
            GuessResult::WrongLetter => {
                self.num_of_attempts -= 1;

                if self.num_of_attempts == 0 {
                    self.state = GameState::Lost;
                }
            },
            GuessResult::LetterAlreadyGuessed => {},
        }

        result
    }

    pub fn get_word(&self) -> String {
        self.word.get_word_hidden()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    fn create_game(word: &str) -> Game {
        Game::new(word.chars().collect())
    }

    #[test]
    fn new_should_initialize_game() {
        let game = create_game("rust");

        assert_eq!(game.num_of_attempts(), 3);
        assert_eq!(game.get_word(), "----");
        assert_eq!(game.state(),GameState::InProgress);
    }

    #[test]
    fn new_should_not_limit_attempts_automatically() {
        let game = create_game("ab");

        assert_eq!(game.num_of_attempts(), 3);
        assert_eq!(game.get_word(), "--");
    }

    #[test]
    fn num_of_attempts_should_return_current_value() {
        let mut game = create_game("rust");
        assert_eq!(game.num_of_attempts(), 3);

        game.num_of_attempts = 2;
        assert_eq!(game.num_of_attempts(), 2);
    }

    #[test]
    fn reset_should_limit_attempts_to_word_length() {
        let mut game = create_game("ab");

        game.resetting_num_of_attempts();

        assert_eq!(game.num_of_attempts(), 2);
    }

    #[test]
    fn reset_should_not_change_attempts_for_long_word() {
        let mut game = create_game("rust");

        game.resetting_num_of_attempts();

        assert_eq!(game.num_of_attempts(), 3);
    }

    #[test]
    fn reset_should_not_change_equal_attempts() {
        let mut game = create_game("abc");

        game.resetting_num_of_attempts();

        assert_eq!(game.num_of_attempts(), 3);
    }

    #[test]
    fn reset_should_be_idempotent() {
        let mut game = create_game("ab");

        game.resetting_num_of_attempts();
        game.resetting_num_of_attempts();

        assert_eq!(game.num_of_attempts(), 2);
    }

    #[test]
    fn reset_should_not_modify_game_state() {
        let mut game = create_game("ab");

        game.resetting_num_of_attempts();

        assert_eq!(game.state(),GameState::InProgress);
        assert_eq!(game.get_word(), "--");
    }

    #[test]
    fn state_should_return_current_state() {
        let mut game = create_game("rust");

        assert_eq!(game.state(), GameState::InProgress);

        game.state = GameState::Won;
        assert_eq!(game.state(), GameState::Won);

        game.state = GameState::Lost;
        assert_eq!(game.state(),GameState::Lost);
    }

    #[test]
    fn get_word_should_return_hidden_word() {
        let game = create_game("rust");

        assert_eq!(game.get_word(), "----");
    }

    #[test]
    fn get_word_should_return_partially_guessed_word() {
        let mut game = create_game("rust");

        game.word.update_word_hidden('r');
        game.word.update_word_hidden('s');

        assert_eq!(game.get_word(), "r-s-");
    }

    #[test]
    fn get_word_should_return_fully_guessed_word() {
        let mut game = create_game("rust");

        for letter in "rust".chars() {
            game.word.update_word_hidden(letter);
        }

        assert_eq!(game.get_word(), "rust");
    }

    #[test]
    fn process_letter_should_not_reduce_attempts_if_correct() {
        let mut game = create_game("rust");

        let result = game.process_letter('r');

        assert_eq!(result, GuessResult::CorrectLetter);
        assert_eq!(game.num_of_attempts(), 3);
        assert_eq!(game.get_word(), "r---");
        assert_eq!(game.state(), GameState::InProgress);
    }

    #[test]
    fn process_letter_should_reduce_attempts_if_wrong() {
        let mut game = create_game("rust");

        let result = game.process_letter('x');

        assert_eq!(result, GuessResult::WrongLetter);
        assert_eq!(game.num_of_attempts(), 2);
        assert_eq!(game.state(), GameState::InProgress);
    }

    #[test]
    fn process_letter_should_not_reduce_attempts_if_repeated() {
        let mut game = create_game("rust");

        game.process_letter('r');
        let result = game.process_letter('r');

        assert_eq!(result, GuessResult::LetterAlreadyGuessed);
        assert_eq!(game.num_of_attempts(), 3);
        assert_eq!(game.get_word(), "r---");
    }

    #[test]
    fn process_letter_should_immediately_set_won() {
        let mut game = create_game("кот");

        game.process_letter('к');
        game.process_letter('о');

        assert_eq!(game.state(), GameState::InProgress);

        let result = game.process_letter('т');

        assert_eq!(result, GuessResult::CorrectLetter);
        assert_eq!(game.state(), GameState::Won);
        assert_eq!(game.get_word(), "кот");
    }

    #[test]
    fn process_letter_should_immediately_set_lost() {
        let mut game = create_game("rust");

        game.process_letter('x');
        game.process_letter('y');

        assert_eq!(game.num_of_attempts(), 1);

        let result = game.process_letter('z');

        assert_eq!(result, GuessResult::WrongLetter);
        assert_eq!(game.num_of_attempts(), 0);
        assert_eq!(game.state(), GameState::Lost);
    }
}
