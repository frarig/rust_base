#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GameState {
    InProgress, // В процессе отгадывания
    Won,        // Слово отгадано
    Lost,       // Попытки исчерпаны
}