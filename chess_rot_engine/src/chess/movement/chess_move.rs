use crate::bitboard::BitBoard;
use crate::chess::{Color, MoveType, Piece, Square};

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


    pub fn new(move_type: MoveType, from: u64, to: u64, piece: Piece, color: Color, target_piece: Piece) -> Self {
        debug_assert!(from < 64, "from must be less than 64");
        debug_assert!(to < 64, "to must be less than 64");
        debug_assert_ne!(piece, Piece::None, "piece to move cannot be none");

        match move_type {
            MoveType::PawnJump => {
                debug_assert_eq!(piece, Piece::Pawn, "only pawn can do pawn jump");
            }
            MoveType::Castling => {
                debug_assert_eq!(piece, Piece::King, "only king can do castling");
            }
            MoveType::EnPassant => {
                debug_assert_eq!(piece, Piece::Pawn, "only pawn can do en passant");
            }
            MoveType::Promotion => {
                debug_assert_eq!(piece, Piece::Pawn, "only pawn can be promoted");
                debug_assert_ne!(target_piece, Piece::None, "target piece cannot be none on promotion");
            }
            _ => {}
        }


        let bb_value = move_type.to_u64()
            | (from << Self::FROM_OFFSET)
            | (to << Self::TO_OFFSET)
            | (piece.to_u64() << Self::PIECE_OFFSET)
            | (color.to_u64() << Self::COLOR_OFFSET)
            | (target_piece.to_u64() << Self::EATEN_PIECE_OFFSET);

        return Self {
            bit_board: BitBoard::from(bb_value)
        };
    }

    pub fn from_to_target(from: u64, to: u64, target_piece: Piece) -> Self {
        let bb_value = MoveType::Invalid.to_u64()
            | (from << Self::FROM_OFFSET)
            | (to << Self::TO_OFFSET)
            | (target_piece.to_u64() << Self::EATEN_PIECE_OFFSET);
        return Self {
            bit_board: BitBoard::from(bb_value)
        };
    }

    pub fn invalid() -> Self {
        let bb_value = MoveType::Invalid.to_u64()
            | (0 << Self::FROM_OFFSET)
            | (0 << Self::TO_OFFSET)
            | (0 << Self::PIECE_OFFSET)
            | (0 << Self::COLOR_OFFSET)
            | (0 << Self::EATEN_PIECE_OFFSET);
        return Self {
            bit_board: BitBoard::from(bb_value)
        };
    }

    pub fn get_type(self) -> MoveType {
        let value = self.bit_board.raw() & Self::MASK_3_BITS;
        return MoveType::try_from(value as usize).unwrap_or(MoveType::Invalid);
    }

    pub fn get_from(self) -> Square {
        return Square::new((self.bit_board.raw() >> Self::FROM_OFFSET) & Self::MASK_6_BITS);
    }

    pub fn get_to(self) -> Square {
        return Square::new((self.bit_board.raw() >> Self::TO_OFFSET) & Self::MASK_6_BITS);
    }

    pub fn get_piece(self) -> Piece {
        let value = (self.bit_board.raw() >> Self::PIECE_OFFSET) & Self::MASK_3_BITS;
        return Piece::try_from(value).unwrap_or(Piece::None);
    }


    pub fn get_color(self) -> Color {
        let value = (self.bit_board.raw() >> Self::COLOR_OFFSET) & Self::MASK_1_BIT;
        return Color::try_from(value).unwrap_or(Color::White);
    }


    pub fn get_target_piece(self) -> Piece {
        let value = (self.bit_board.raw() >> Self::EATEN_PIECE_OFFSET) & Self::MASK_3_BITS;
        return Piece::try_from(value).unwrap_or(Piece::None);
    }

    pub fn to_capture(self, target_piece: Piece) -> Move {
        debug_assert_ne!(target_piece, Piece::None, "target piece cannot be none on capture");
        return Move::new(MoveType::Capture, self.get_from().raw(), self.get_to().raw(), self.get_piece(), self.get_color(), target_piece);
    }
}