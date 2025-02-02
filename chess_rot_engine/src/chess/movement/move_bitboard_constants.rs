use crate::bitboard::BitBoard;
use crate::chess::Square;

/// Magic bitboard constants
/// See: [ChessProgramming Magic Bitboards](https://www.chessprogramming.org/Magic_Bitboards)

pub const MAGIC_A1H8: [BitBoard; 15] = [
    BitBoard::from(0x0),
    BitBoard::from(0x0),
    BitBoard::from(0x0101010101010100),
    BitBoard::from(0x0101010101010100),
    BitBoard::from(0x0101010101010100),
    BitBoard::from(0x0101010101010100),
    BitBoard::from(0x0101010101010100),
    BitBoard::from(0x0101010101010100),
    BitBoard::from(0x8080808080808000),
    BitBoard::from(0x4040404040400000),
    BitBoard::from(0x2020202020000000),
    BitBoard::from(0x1010101000000000),
    BitBoard::from(0x0808080000000000),
    BitBoard::from(0x0),
    BitBoard::from(0x0),
];

pub const MAGIC_A8H1: [BitBoard; 15] = [
    BitBoard::from(0x0),
    BitBoard::from(0x0),
    BitBoard::from(0x0101010101010100),
    BitBoard::from(0x0101010101010100),
    BitBoard::from(0x0101010101010100),
    BitBoard::from(0x0101010101010100),
    BitBoard::from(0x0101010101010100),
    BitBoard::from(0x0101010101010100),
    BitBoard::from(0x0080808080808080),
    BitBoard::from(0x0040404040404040),
    BitBoard::from(0x0020202020202020),
    BitBoard::from(0x0010101010101010),
    BitBoard::from(0x0008080808080808),
    BitBoard::from(0x0),
    BitBoard::from(0x0),
];

pub const MAGIC_FILE: [BitBoard; 8] = [
    BitBoard::from(0x8040201008040200),
    BitBoard::from(0x4020100804020100),
    BitBoard::from(0x2010080402010080),
    BitBoard::from(0x1008040201008040),
    BitBoard::from(0x0804020100804020),
    BitBoard::from(0x0402010080402010),
    BitBoard::from(0x0201008040201008),
    BitBoard::from(0x0100804020100804),
];

pub const WHITE_CASTLING_KING_SIDE_REQUIRED_EMPTY: BitBoard = BitBoard::from(Square::F1.as_bb().raw() | Square::G1.as_bb().raw());
pub const WHITE_CASTLING_KING_SIDE_ATTACK_MASK: BitBoard =
    BitBoard::from(Square::E1.as_bb().raw() | Square::F1.as_bb().raw() | Square::G1.as_bb().raw());
pub const WHITE_CASTLING_QUEEN_SIDE_REQUIRED_EMPTY: BitBoard =
    BitBoard::from(Square::B1.as_bb().raw() | Square::C1.as_bb().raw() | Square::D1.as_bb().raw());
pub const WHITE_CASTLING_QUEEN_SIDE_ATTACK_MASK: BitBoard =
    BitBoard::from(Square::C1.as_bb().raw() | Square::D1.as_bb().raw() | Square::E1.as_bb().raw());
pub const BLACK_CASTLING_KING_SIDE_REQUIRED_EMPTY: BitBoard = BitBoard::from(Square::F8.as_bb().raw() | Square::G8.as_bb().raw());
pub const BLACK_CASTLING_KING_SIDE_ATTACK_MASK: BitBoard =
    BitBoard::from(Square::E8.as_bb().raw() | Square::F8.as_bb().raw() | Square::G8.as_bb().raw());
pub const BLACK_CASTLING_QUEEN_SIDE_REQUIRED_EMPTY: BitBoard =
    BitBoard::from(Square::B8.as_bb().raw() | Square::C8.as_bb().raw() | Square::D8.as_bb().raw());
pub const BLACK_CASTLING_QUEEN_SIDE_ATTACK_MASK: BitBoard =
    BitBoard::from(Square::C8.as_bb().raw() | Square::D8.as_bb().raw() | Square::E8.as_bb().raw());