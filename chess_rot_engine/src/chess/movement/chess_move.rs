use crate::bitboard::BitBoard;
use crate::chess::{Color, MoveType, Piece};

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct Move {
    bit_board: BitBoard,
}

impl Move {
    const FROM_OFFSET: u64 = 3;
    const TO_OFFSET: u64 = 9;
    const PIECE_OFFSET: u64 = 15;
    const COLOR_OFFSET: u64 = 18;
    const EATEN_PIECE_OFFSET: u64 = 19;

    const MASK_1_BIT: u64 = 0b1;
    const MASK_3_BITS: u64 = 0b111;
    const MASK_6_BITS: u64 = 0b111111;


    pub fn new(move_type: MoveType, from: u64, to: u64, piece: Piece, color: Color, eaten_piece: Piece) -> Self {
        let bb_value = move_type.to_u64()
            | (from << Self::FROM_OFFSET)
            | (to << Self::TO_OFFSET)
            | (piece.to_u64() << Self::PIECE_OFFSET)
            | (color.to_u64() << Self::COLOR_OFFSET)
            | (eaten_piece.to_u64() << Self::EATEN_PIECE_OFFSET);

        return Self {
            bit_board: BitBoard::from(bb_value)
        };
    }

    pub fn get_type(self) -> MoveType {
        let value = self.bit_board.raw_value() & Self::MASK_3_BITS;
        return MoveType::try_from(value as usize).unwrap_or(MoveType::Invalid);
    }

    pub fn get_from(self) -> u64 {
        return (self.bit_board.raw_value() >> Self::FROM_OFFSET) & Self::MASK_6_BITS;
    }

    pub fn get_to(self) -> u64 {
        return (self.bit_board.raw_value() >> Self::TO_OFFSET) & Self::MASK_6_BITS;
    }

    pub fn get_piece(self) -> Piece {
        let value = (self.bit_board.raw_value() >> Self::PIECE_OFFSET) & Self::MASK_3_BITS;
        return Piece::try_from(value).unwrap_or(Piece::None);
    }


    pub fn get_color(self) -> Color {
        let value = (self.bit_board.raw_value() >> Self::COLOR_OFFSET) & Self::MASK_1_BIT;
        return Color::try_from(value).unwrap_or(Color::White);
    }


    pub fn get_eaten_piece(self) -> Piece {
        let value = (self.bit_board.raw_value() >> Self::EATEN_PIECE_OFFSET) & Self::MASK_3_BITS;
        return Piece::try_from(value).unwrap_or(Piece::None);
    }
}