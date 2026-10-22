mod ui;
mod random_word;
mod hangman;

use crate::ui::ui::*;
use crate::random_word::random_word_api::Word;
use crate::hangman::game::{Game, GameState};

fn main() {
    print_greeting();

    let word_length = get_word_length();
    let word = get_word(word_length);

    let mut game = Game::new(word);
    
    info_print(game.num_of_attempts());

    game.resetting_num_of_attempts();

    loop {
        match game.state() {
            GameState::Won => {
                won_print(&game.get_word());
                break
            },
            GameState::Lost => {
                game_over_print();
                break
            },
            _ => {}
        };

        word_hidden_print(game.get_word());
        num_of_attempts_print(game.num_of_attempts());
        enter_letter_print();

        game.next_step();
    }
}

fn get_word(word_length: i32) -> Vec<char> {
    Word::new()
        .get_word_by_word_len(&word_length)
        .first()
        .map(String::as_str)
        .unwrap_or("rust")
        .chars()
        .collect()
}