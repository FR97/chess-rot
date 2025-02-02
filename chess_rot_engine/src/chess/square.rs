use std::fmt;
use std::fmt::{Display, Formatter};
use crate::bitboard;
use crate::bitboard::BitBoard;
use crate::chess::GameError;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Square {
    index: usize,
}

impl Square {
    pub const fn new(index: u64) -> Self {
        debug_assert!(index < 64, "square index can be value between 0 and 63");
        return Self { index: index as usize };
    }

    pub const fn from_usize(index: usize) -> Square {
        debug_assert!(index < 64, "square index can be value between 0 and 63");
        return Self { index };
    }

    pub fn from_string(sq: &str) -> Option<Square> {
        let first = sq.to_lowercase().chars().nth(0)?;
        let second = sq.to_lowercase().chars().nth(1)?;

        let rank = match first {
            'a' => Some(0),
            'b' => Some(1),
            'c' => Some(2),
            'd' => Some(3),
            'e' => Some(4),
            'f' => Some(5),
            'g' => Some(6),
            'h' => Some(7),
            _ => None
        }?;

        let file = match second {
            '1' => Some(0),
            '2' => Some(1),
            '3' => Some(2),
            '4' => Some(3),
            '5' => Some(4),
            '6' => Some(5),
            '7' => Some(6),
            '8' => Some(7),
            _ => None
        }?;


        let square = rank + file * 8;

        println!("Rank {} File {} Index {}", rank, file, sq);
        if (square < 0 || square > 63) {
            return None;
        }

        return Some(Self::from_usize(square as usize));
    }

    pub const fn from_label(label: SquareLabel) -> Square {
        return Self::new(label as u64);
    }

    pub const fn raw(&self) -> u64 {
        return self.index as u64;
    }

    pub const fn as_bb(&self) -> BitBoard {
        return BitBoard::from(1 << self.index);
    }

    pub const fn as_usize(&self) -> usize {
        return self.index as usize;
    }

    #[inline(always)]
    pub const fn file(self) -> usize {
        self.index % 8
    }

    #[inline(always)]
    pub const fn rank(self) -> usize {
        self.index / 8
    }

    pub const A1: Square = Square::new(0);
    pub const B1: Square = Square::new(1);
    pub const C1: Square = Square::new(2);
    pub const D1: Square = Square::new(3);
    pub const E1: Square = Square::new(4);
    pub const F1: Square = Square::new(5);
    pub const G1: Square = Square::new(6);
    pub const H1: Square = Square::new(7);

    pub const A2: Square = Square::new(8);
    pub const B2: Square = Square::new(9);
    pub const C2: Square = Square::new(10);
    pub const D2: Square = Square::new(11);
    pub const E2: Square = Square::new(12);
    pub const F2: Square = Square::new(13);
    pub const G2: Square = Square::new(14);
    pub const H2: Square = Square::new(15);

    pub const A3: Square = Square::new(16);
    pub const B3: Square = Square::new(17);
    pub const C3: Square = Square::new(18);
    pub const D3: Square = Square::new(19);
    pub const E3: Square = Square::new(20);
    pub const F3: Square = Square::new(21);
    pub const G3: Square = Square::new(22);
    pub const H3: Square = Square::new(23);

    pub const A4: Square = Square::new(24);
    pub const B4: Square = Square::new(25);
    pub const C4: Square = Square::new(26);
    pub const D4: Square = Square::new(27);
    pub const E4: Square = Square::new(28);
    pub const F4: Square = Square::new(29);
    pub const G4: Square = Square::new(30);
    pub const H4: Square = Square::new(31);

    pub const A5: Square = Square::new(32);
    pub const B5: Square = Square::new(33);
    pub const C5: Square = Square::new(34);
    pub const D5: Square = Square::new(35);
    pub const E5: Square = Square::new(36);
    pub const F5: Square = Square::new(37);
    pub const G5: Square = Square::new(38);
    pub const H5: Square = Square::new(39);

    pub const A6: Square = Square::new(40);
    pub const B6: Square = Square::new(41);
    pub const C6: Square = Square::new(42);
    pub const D6: Square = Square::new(43);
    pub const E6: Square = Square::new(44);
    pub const F6: Square = Square::new(45);
    pub const G6: Square = Square::new(46);
    pub const H6: Square = Square::new(47);

    pub const A7: Square = Square::new(48);
    pub const B7: Square = Square::new(49);
    pub const C7: Square = Square::new(50);
    pub const D7: Square = Square::new(51);
    pub const E7: Square = Square::new(52);
    pub const F7: Square = Square::new(53);
    pub const G7: Square = Square::new(54);
    pub const H7: Square = Square::new(55);

    pub const A8: Square = Square::new(56);
    pub const B8: Square = Square::new(57);
    pub const C8: Square = Square::new(58);
    pub const D8: Square = Square::new(59);
    pub const E8: Square = Square::new(60);
    pub const F8: Square = Square::new(61);
    pub const G8: Square = Square::new(62);
    pub const H8: Square = Square::new(63);

    #[rustfmt::skip]
    pub const ALL_FIELDS: &'static [Square] = &[
        Square::A1, Square::B1, Square::C1, Square::D1, Square::E1, Square::F1, Square::G1, Square::H1,
        Square::A2, Square::B2, Square::C2, Square::D2, Square::E2, Square::F2, Square::G2, Square::H2,
        Square::A3, Square::B3, Square::C3, Square::D3, Square::E3, Square::F3, Square::G3, Square::H3,
        Square::A4, Square::B4, Square::C4, Square::D4, Square::E4, Square::F4, Square::G4, Square::H4,
        Square::A5, Square::B5, Square::C5, Square::D5, Square::E5, Square::F5, Square::G5, Square::H5,
        Square::A6, Square::B6, Square::C6, Square::D6, Square::E6, Square::F6, Square::G6, Square::H6,
        Square::A7, Square::B7, Square::C7, Square::D7, Square::E7, Square::F7, Square::G7, Square::H7,
        Square::A8, Square::B8, Square::C8, Square::D8, Square::E8, Square::F8, Square::G8, Square::H8,
    ];
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
        return BitBoard::SINGLE_BIT_BB[self.index()];
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
