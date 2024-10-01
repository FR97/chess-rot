use std::fmt;
use std::io::SeekFrom;
use crate::bitboard::BitBoard;
use crate::chess::Color;

#[repr(u64)]
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Piece {
    King = 0,
    Queen = 1,
    Rook = 2,
    Bishop = 3,
    Knight = 4,
    Pawn = 5,
    None = 6,
}

impl Piece {
    const DEFAULT_WHITE_PIECES: u64 = 0b0000000000000000000000000000000000000000000000001111111111111111;
    const DEFAULT_BLACK_PIECES: u64 = 0b1111111111111111000000000000000000000000000000000000000000000000;

    const DEFAULT_WHITE_PAWNS: u64 = 0b0000000000000000000000000000000000000000000000001111111100000000;
    const DEFAULT_BLACK_PAWNS: u64 = 0b0000000011111111000000000000000000000000000000000000000000000000;

    const DEFAULT_WHITE_ROOK: u64 = 0b0000000000000000000000000000000000000000000000000000000010000001;
    const DEFAULT_BLACK_ROOK: u64 = 0b1000000100000000000000000000000000000000000000000000000000000000;

    const DEFAULT_WHITE_KNIGHT: u64 = 0b0000000000000000000000000000000000000000000000000000000001000010;
    const DEFAULT_BLACK_KNIGHT: u64 = 0b0100001000000000000000000000000000000000000000000000000000000000;

    const DEFAULT_WHITE_BISHOP: u64 = 0b0000000000000000000000000000000000000000000000000000000000100100;
    const DEFAULT_BLACK_BISHOP: u64 = 0b0010010000000000000000000000000000000000000000000000000000000000;

    const DEFAULT_WHITE_QUEEN: u64 = 0b0000000000000000000000000000000000000000000000000000000000001000;
    const DEFAULT_BLACK_QUEEN: u64 = 0b0000100000000000000000000000000000000000000000000000000000000000;

    const DEFAULT_WHITE_KING: u64 = 0b0000000000000000000000000000000000000000000000000000000000010000;
    const DEFAULT_BLACK_KING: u64 = 0b0001000000000000000000000000000000000000000000000000000000000000;

    const DEFAULT_PIECE_POSITIONS: [[u64; 6]; 2] = [
        [Self::DEFAULT_WHITE_KING,
            Self::DEFAULT_WHITE_QUEEN,
            Self::DEFAULT_WHITE_ROOK,
            Self::DEFAULT_WHITE_BISHOP,
            Self::DEFAULT_WHITE_KNIGHT,
            Self::DEFAULT_WHITE_PAWNS,
        ],
        [
            Self::DEFAULT_BLACK_KING,
            Self::DEFAULT_BLACK_QUEEN,
            Self::DEFAULT_BLACK_ROOK,
            Self::DEFAULT_BLACK_BISHOP,
            Self::DEFAULT_BLACK_KNIGHT,
            Self::DEFAULT_BLACK_PAWNS,
        ],
    ];

    pub fn default_bitboard_for_color(color: Color) -> BitBoard {
        return match color {
            Color::White => BitBoard::from(Self::DEFAULT_WHITE_PIECES),
            Color::Black => BitBoard::from(Self::DEFAULT_BLACK_PIECES),
        };
    }

    pub fn default_bitboard_for_color_and_type(color: Color, piece: Piece) -> BitBoard {
        return BitBoard::from(Self::DEFAULT_PIECE_POSITIONS[color.index()][piece.index()]);
    }


    pub fn to_u64(self) -> u64 {
        return self as u64;
    }

    pub fn index(self) -> usize {
        return usize::try_from(self.to_u64()).unwrap_or(6);
    }

    pub fn value(self) -> usize {
        match self {
            Piece::King => 30000,
            Piece::Queen => 900,
            Piece::Rook => 500,
            Piece::Bishop => 330,
            Piece::Knight => 325,
            Piece::Pawn => 100,
            Piece::None => 0,
        }
    }

    pub fn to_char(self) -> char {
        return match self {
            Piece::King => 'k',
            Piece::Queen => 'q',
            Piece::Rook => 'r',
            Piece::Bishop => 'b',
            Piece::Knight => 'n',
            Piece::Pawn => 'p',
            Piece::None => '_',
        };
    }

    pub fn to_char_representation(self, color: Color) -> char {
        return match color {
            Color::White => self.to_char().to_ascii_uppercase(),
            Color::Black => self.to_char(),
        };
    }
}

impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_char())
    }
}

impl TryFrom<u64> for Piece {
    type Error = ();

    fn try_from(v: u64) -> Result<Self, Self::Error> {
        match v {
            x if x == Piece::King.to_u64() => Ok(Piece::King),
            x if x == Piece::Queen.to_u64() => Ok(Piece::Queen),
            x if x == Piece::Rook.to_u64() => Ok(Piece::Rook),
            x if x == Piece::Bishop.to_u64() => Ok(Piece::Bishop),
            x if x == Piece::Knight.to_u64() => Ok(Piece::Knight),
            x if x == Piece::Pawn.to_u64() => Ok(Piece::Pawn),
            x if x == Piece::None.to_u64() => Ok(Piece::None),
            x => panic!("trying to get piece type for invalid u64 value {}", x),
        }
    }
}

impl TryFrom<usize> for Piece {
    type Error = ();

    fn try_from(v: usize) -> Result<Self, Self::Error> {
        match v {
            x if x == Piece::King.index() => Ok(Piece::King),
            x if x == Piece::Queen.index() => Ok(Piece::Queen),
            x if x == Piece::Rook.index() => Ok(Piece::Rook),
            x if x == Piece::Bishop.index() => Ok(Piece::Bishop),
            x if x == Piece::Knight.index() => Ok(Piece::Knight),
            x if x == Piece::Pawn.index() => Ok(Piece::Pawn),
            x if x == Piece::None.index() => Ok(Piece::None),
            x => panic!("trying to get piece type for invalid usize value {}", x),
        }
    }
}
