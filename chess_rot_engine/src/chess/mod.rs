pub mod ai;
mod castling;
mod movement;
mod color;
mod board_state;
mod game;
mod piece;
mod square;
mod error;

pub use self::castling::CastlingRight;
pub use self::movement::chess_move::Move;
pub use self::movement::move_provider;
pub use self::movement::move_type::MoveType;
pub use self::color::Color;
pub use self::board_state::{BoardState, BoardIterator};
pub use self::error::GameError;
pub use self::piece::Piece;
pub use self::piece::ColoredPiece;
pub use self::square::{Square, SquareLabel};
pub use self::game::{Game, GameResult};
