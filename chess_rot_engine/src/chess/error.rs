use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum GameError {
    FenFormatError(String)
}

impl fmt::Display for GameError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GameError::FenFormatError(err) => {
                write!(f, "FenFormatError[{}]", err)
            }
        }
    }
}

impl Error for GameError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        return None;
    }
}
