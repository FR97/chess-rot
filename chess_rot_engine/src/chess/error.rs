use std::error::Error;
use std::fmt;
use std::fmt::write;

#[derive(Debug)]
pub enum GameError {
    FenFormatError(String),
    OpenAiResponseError(String),
    NoPossibleMoveError,
    InvalidSquareError(String),
    InvalidMoveError,
}

impl fmt::Display for GameError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GameError::FenFormatError(err) => {
                write!(f, "{}", err)
            }
            GameError::OpenAiResponseError(err) => {
                write!(f, "{}", err)
            }
            GameError::NoPossibleMoveError => {
                write!(f, "No more possible moves!")
            }
            GameError::InvalidSquareError(err) => {
                write!(f, "Invalid square: {}", err)
            }
            GameError::InvalidMoveError => {
                write!(f, "Invalid move")
            }
        }
    }
}

impl Error for GameError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        return None;
    }
}
