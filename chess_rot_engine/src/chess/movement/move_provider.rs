use crate::bitboard::BitBoard;
use crate::chess::movement::direction::Direction;
use crate::chess::{BoardState, Color, ColoredPiece, Move, MoveType, Piece};
use std::cmp::PartialEq;
use std::ptr::null;

#[derive(Debug, PartialEq, Clone)]
pub struct PreGeneratedMoveProvider {
    // Maximum possible mobility is for queen in the middle of board is 27 so that is maximum number of moves
    cached_moves: [[Vec<Move>; 64]; 12],
}

impl PreGeneratedMoveProvider {
    const MAILBOX: [i32; 120] = [
        -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 0, 1,
        2, 3, 4, 5, 6, 7, -1, -1, 8, 9, 10, 11, 12, 13, 14, 15, -1, -1, 16, 17, 18, 19, 20, 21, 22,
        23, -1, -1, 24, 25, 26, 27, 28, 29, 30, 31, -1, -1, 32, 33, 34, 35, 36, 37, 38, 39, -1, -1,
        40, 41, 42, 43, 44, 45, 46, 47, -1, -1, 48, 49, 50, 51, 52, 53, 54, 55, -1, -1, 56, 57, 58,
        59, 60, 61, 62, 63, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
        -1, -1, -1,
    ];

    const MAILBOX64: [i32; 64] = [
        21, 22, 23, 24, 25, 26, 27, 28, 31, 32, 33, 34, 35, 36, 37, 38, 41, 42, 43, 44, 45, 46, 47,
        48, 51, 52, 53, 54, 55, 56, 57, 58, 61, 62, 63, 64, 65, 66, 67, 68, 71, 72, 73, 74, 75, 76,
        77, 78, 81, 82, 83, 84, 85, 86, 87, 88, 91, 92, 93, 94, 95, 96, 97, 98,
    ];

    const OFFSETS: [[i32; 8]; 6] = [
        [-11, -10, -9, -1, 1, 9, 10, 11],
        [-11, -10, -9, -1, 1, 9, 10, 11],
        [-10, -1, 1, 10, 0, 0, 0, 0],
        [-11, -9, 9, 11, 0, 0, 0, 0],
        [-21, -19, -12, -8, 8, 12, 19, 21],
        [1, 0, 0, 0, 0, 0, 0, 0],
    ];

    fn generate_moves(position: usize, piece: Piece, color: Color) -> Vec<Move> {
        let mut i = 0;
        let mut vec = Vec::new();
        while i < 8 && Self::OFFSETS[piece.index()][i] != 0 {
            match piece {
                Piece::King | Piece::Knight => {
                    let mut possible = Self::MAILBOX
                        [(Self::MAILBOX64[position] + Self::OFFSETS[piece.index()][i]) as usize];
                    if possible != -1 {
                        let m = Move::new(
                            MoveType::Push,
                            position as u64,
                            possible as u64,
                            piece,
                            color,
                            Piece::None,
                        );
                        vec.push(m);
                    }
                }
                Piece::Queen | Piece::Rook | Piece::Bishop => {
                    let mut possible = Self::MAILBOX
                        [(Self::MAILBOX64[position] + Self::OFFSETS[piece.index()][i]) as usize];
                    while possible != -1 {
                        let m = Move::new(
                            MoveType::Push,
                            position as u64,
                            possible as u64,
                            piece,
                            color,
                            Piece::None,
                        );
                        vec.push(m);
                        possible = Self::MAILBOX[(Self::MAILBOX64[possible as usize]
                            + Self::OFFSETS[piece.index()][i])
                            as usize];
                    }
                }
                Piece::Pawn => {
                    let direction = if color == Color::White { 1 } else { -1 };
                    let starting_position_range = if color == Color::White {
                        (8, 15)
                    } else {
                        (48, 55)
                    };

                    // one field ahead
                    let mut possible =
                        Self::MAILBOX[(Self::MAILBOX64[position] + 10 * direction) as usize];
                    if possible != -1 {
                        let m = Move::new(
                            MoveType::Push,
                            position as u64,
                            possible as u64,
                            piece,
                            color,
                            Piece::None,
                        );
                        vec.push(m);
                        // double jump
                        if position >= starting_position_range.0
                            && position <= starting_position_range.1
                        {
                            possible = Self::MAILBOX
                                [(Self::MAILBOX64[position] + 20 * direction) as usize];
                            let m = Move::new(
                                MoveType::PawnJump,
                                position as u64,
                                possible as u64,
                                piece,
                                color,
                                Piece::None,
                            );
                            vec.push(m);
                        }
                    }

                    let mut possible =
                        Self::MAILBOX[(Self::MAILBOX64[position] + 10 * direction - 1) as usize];
                    if possible != -1 {
                        let m = Move::new(
                            MoveType::Capture,
                            position as u64,
                            possible as u64,
                            piece,
                            color,
                            Piece::None,
                        );
                        vec.push(m);
                    }

                    let mut possible =
                        Self::MAILBOX[(Self::MAILBOX64[position] + 10 * direction + 1) as usize];
                    if possible != -1 {
                        let m = Move::new(
                            MoveType::Capture,
                            position as u64,
                            possible as u64,
                            piece,
                            color,
                            Piece::None,
                        );
                        vec.push(m);
                    }
                }
                Piece::None => {}
            }
            i += 1;
        }

        return vec;
    }
}

pub trait MoveProvider {
    fn get_available_moves(self, state: BoardState, for_position: u64) -> Vec<Move>;
}

impl MoveProvider for PreGeneratedMoveProvider {
    fn get_available_moves(self, state: BoardState, for_position: u64) -> Vec<Move> {
        let piece = state.get_piece_at(for_position as u8);
        if let Some(colored_piece) = piece {
            if colored_piece.0 == Piece::None {
                return Vec::with_capacity(0);
            }

            let piece_index = colored_piece.0.index()
                + if colored_piece.1 == Color::White {
                    0
                } else {
                    6
                };
            let all_possible = self.cached_moves[piece_index][for_position as usize].clone();

            let valid = all_possible
                .into_iter()
                .filter(|m| {
                    if let Some(colored_piece) = state.get_piece_at(m.get_to().raw() as u8) {
                        colored_piece.1 != state.on_move()
                    } else {
                        true
                    }
                })
                .filter(|m| match m.get_type() {
                    MoveType::Push | MoveType::PawnJump => {
                        state.get_piece_at(m.get_to().raw() as u8).is_none()
                    }
                    MoveType::Capture => {
                        if let Some(colored_piece) = state.get_piece_at(m.get_to().raw() as u8) {
                            colored_piece.1 == state.on_move()
                        } else {
                            false
                        }
                    }
                    MoveType::Invalid => false,
                    _ => false,
                })
                .collect();

            return valid;
        }
        return Vec::with_capacity(0);
    }
}

impl Default for PreGeneratedMoveProvider {
    fn default() -> Self {
        let start = std::time::Instant::now();
        println!("Generating pre-generated moves");
        let mut moves: [[Vec<Move>; 64]; 12] =
            core::array::from_fn(|_| core::array::from_fn(|_| Vec::new()));
        for i in 0..6 {
            let piece = Piece::try_from(i).expect("cannot convert index to piece");
            for sqr in 0..64 {
                let white_moves =
                    PreGeneratedMoveProvider::generate_moves(sqr, piece, Color::White);
                let black_moves =
                    PreGeneratedMoveProvider::generate_moves(sqr, piece, Color::Black);
                moves[i][sqr] = white_moves;
                moves[i + 6][sqr] = black_moves;
            }
        }

        print!(
            "Done generating pre-generated moves, total time: {} ms\n",
            start.elapsed().as_millis()
        );

        return Self {
            cached_moves: moves,
        };
    }
}

#[cfg(test)]
mod test {
    use crate::bitboard;
    use crate::chess::move_provider::PreGeneratedMoveProvider;
    use crate::chess::{Color, Piece};

    #[test]
    fn test_generate_moves() {
        let bb = PreGeneratedMoveProvider::generate_moves(0, Piece::Rook, Color::White);
        let bb = PreGeneratedMoveProvider::generate_moves(27, Piece::Rook, Color::Black);
        let bb = PreGeneratedMoveProvider::generate_moves(27, Piece::King, Color::White);
        let bb = PreGeneratedMoveProvider::generate_moves(32, Piece::Queen, Color::Black);
        let bb = PreGeneratedMoveProvider::generate_moves(9, Piece::Pawn, Color::White);

        // let bb = PreGeneratedMoveProvider::generate_moves_in_direction(0, 0, Direction::NorthEast);
        // println!("{}", bb);
    }
}
