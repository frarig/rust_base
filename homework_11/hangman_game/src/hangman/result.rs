#[derive(Debug, PartialEq)]
pub enum GuessResult {
    CorrectLetter,          // Буква угадана
    LetterAlreadyGuessed,   // Буква уже отгадана
    WrongLetter,            // Буква не угадана
    CorrectWord,            // Слово отгадано
    WrongWord,              // Слово не отгадано
}