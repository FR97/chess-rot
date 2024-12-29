use std::fmt;
use std::fmt::{Display, Formatter};
use std::os::unix::raw::uid_t;
use crate::bitboard;
use crate::bitboard::BitBoard;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Square {
    index: u64,
}

impl Square {
    pub fn new(index: u64) -> Square {
        debug_assert!(index < 64, "square index can be value between 0 and 63");
        return Square { index };
    }

    pub fn from_usize(index: usize) -> Square {
        debug_assert!(index < 64, "square index can be value between 0 and 63");
        let index_u64 = index as u64;
        return Square { index: index_u64 };
    }

    pub fn from_label(label: SquareLabel) -> Square {
        return Self::new(label as u64);
    }

    pub fn raw(&self) -> u64 {
        return self.index;
    }

    pub fn as_bb(&self) -> BitBoard {
        return BitBoard::from(1 << self.index);
    }

    pub fn as_usize(&self) -> usize {
        return self.index as usize;
    }

}

impl Display for Square {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.index)
    }
}

// this is better approach than Square struct that I created initially
#[repr(u64)]
#[rustfmt::skip]
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum SquareLabel {
    A1, B1, C1, D1, E1, F1, G1, H1,
    A2, B2, C2, D2, E2, F2, G2, H2,
    A3, B3, C3, D3, E3, F3, G3, H3,
    A4, B4, C4, D4, E4, F4, G4, H4,
    A5, B5, C5, D5, E5, F5, G5, H5,
    A6, B6, C6, D6, E6, F6, G6, H6,
    A7, B7, C7, D7, E7, F7, G7, H7,
    A8, B8, C8, D8, E8, F8, G8, H8,
    None,
}

impl SquareLabel {
    pub fn as_u64(self) -> u64 {
        return self as u64;
    }

    pub fn as_usize(self) -> usize {
        return self as usize;
    }

    pub fn to_bb(self) -> BitBoard {
        return BitBoard::SINGLE_BIT_BB[self.index()]
    }

    pub fn index(self) -> usize {
        return self as usize;
    }
}

impl fmt::Display for SquareLabel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
        // or, alternatively:
        // fmt::Debug::fmt(self, f)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_square_labels() {
        let test_square = Square::from_label(SquareLabel::A1);
        debug_assert_eq!(0, test_square.index);
        let test_square = Square::from_label(SquareLabel::H8);
        debug_assert_eq!(63, test_square.index);
    }
}
