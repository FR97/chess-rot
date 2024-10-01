use std::fmt;
use crate::bitboard::BitBoard;

#[repr(u64)]
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Color {
    White = 0,
    Black = 1,
}

impl Color {

    pub fn to_u64(self) -> u64 {
        return self as u64;
    }

    pub fn index(self) -> usize {
        return usize::try_from(self.to_u64()).unwrap_or(6);
    }

    pub fn inverse(&self) -> Color {
        return match self {
            Color::White => Color::Black,
            Color::Black => Color::White,
        };
    }

    pub fn to_char(&self) -> char {
        return match self {
            Color::White => 'w',
            Color::Black => 'b',
        };
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_char())
    }
}

impl TryFrom<u64> for Color {
    type Error = ();

    fn try_from(v: u64) -> Result<Self, Self::Error> {
        match v {
            x if x == Color::White.to_u64() => Ok(Color::White),
            x if x == Color::Black.to_u64() => Ok(Color::Black),
            x => panic!("trying to get color for invalid u64 value {}", x),
        }
    }
}

impl TryFrom<usize> for Color {
    type Error = ();

    fn try_from(v: usize) -> Result<Self, Self::Error> {
        match v {
            x if x == Color::White.index() => Ok(Color::White),
            x if x == Color::Black.index() => Ok(Color::Black),
            x => panic!("trying to get color for invalid usize value {}", x),
        }
    }
}