use std::fmt;
use crate::chess::Color;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct CastlingRight {
    value: u8,
}

impl CastlingRight {
    const DEFAULT: u8 = 0b00001111;
    const NO_CASTLING: u8 = 0;

    const WHITE_KING_SIDE_MASK: u8 = 0b00000001;
    const WHITE_QUEEN_SIDE_MASK: u8 = 0b00000010;
    const BLACK_KING_SIDE_MASK: u8 = 0b00000100;
    const BLACK_QUEEN_SIDE_MARK: u8 = 0b00001000;

    pub fn default() -> CastlingRight {
        return CastlingRight { value: CastlingRight::DEFAULT };
    }

    pub fn from_raw(value: u8) -> CastlingRight {
        return CastlingRight { value };
    }

    pub fn is_white_king_side_allowed(&self) -> bool {
        return self.value & CastlingRight::WHITE_KING_SIDE_MASK != 0;
    }

    pub fn is_white_queen_side_allowed(&self) -> bool {
        return self.value & CastlingRight::WHITE_QUEEN_SIDE_MASK != 0;
    }

    pub fn is_black_king_side_allowed(&self) -> bool {
        return self.value & CastlingRight::BLACK_KING_SIDE_MASK != 0;
    }

    pub fn is_black_queen_side_allowed(&self) -> bool {
        return self.value & CastlingRight::BLACK_QUEEN_SIDE_MARK != 0;
    }

    pub fn remove_king_side_castle(&self, color: Color) -> CastlingRight {
        return match color {
            Color::White => CastlingRight::from_raw(self.value ^ CastlingRight::WHITE_KING_SIDE_MASK),
            Color::Black => CastlingRight::from_raw(self.value ^ CastlingRight::BLACK_KING_SIDE_MASK),
        };
    }

    pub fn remove_queen_side_castle(&self, color: Color) -> CastlingRight {
        return match color {
            Color::White => CastlingRight::from_raw(self.value ^ CastlingRight::WHITE_QUEEN_SIDE_MASK),
            Color::Black => CastlingRight::from_raw(self.value ^ CastlingRight::BLACK_QUEEN_SIDE_MARK),
        };
    }
}

impl fmt::Display for CastlingRight {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut str = String::new();

        if(self.is_white_king_side_allowed()){
            str.push('K');
        }

        if(self.is_white_queen_side_allowed()){
            str.push('Q');
        }

        if(self.is_black_king_side_allowed()){
            str.push('k');
        }

        if(self.is_black_queen_side_allowed()){
            str.push('q');
        }

        if str.is_empty() {
            str.push('-')
        }

        write!(f, "{}", str)
    }
}
