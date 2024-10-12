use crate::bitboard::BitBoard;
use crate::chess::{BoardState, ColoredPiece, Move};

pub struct PreGeneratedMoveProvider {
    // Maximum possible mobility is for queen in the middle of board is 27 so that is maximum number of moves
    cached_moves: [[[Vec<Move>]; 64]; 12],
}

impl PreGeneratedMoveProvider {

    pub fn generate_moves() -> [[Vec<Move>; 64]; 12] {
        todo!()
    }

    pub fn generate_moves_for_rook() -> [Vec<Move>; 64] {
        const NEW_VEC: Vec<Move> = Vec::new();
        let mut moves = [NEW_VEC; 64];

        for rank in 0..8 {
            for file in (0..8) {
                let position = (rank * 8) + file;
                let bb = BitBoard::empty();
            }
        }

        return moves;
    }

}


pub trait MoveProvider {
    fn get_available_moves(self, state: BoardState, for_position: u64) -> Vec<Move>;
}