use crate::bitboard::BitBoard;
use crate::chess::{BoardState, Color, Move, Piece};

#[derive(Debug, PartialEq, Clone)]
pub struct Evaluator {}

impl Evaluator {
    const CASTLING_VALUE: i32 = 65;
    const CHECK_VALUE: i32 = 40;
    const CHECK_MATE_VALUE: i32 = 10000;
    const ISOLATED_PAWN_VALUE: i32 = -15;
    const DOUBLE_BISHOP_VALUE: i32 = 45;

    pub fn new() -> Self {
        return Self {};
    }

    pub fn evaluate(&self, board: &BoardState, legal_moves: &Vec<Move>, depth: usize) -> i32 {
        return Self::calculate_score(Color::White, board, legal_moves, depth)
            - Self::calculate_score(Color::Black, board, legal_moves, depth);
    }

    fn calculate_score(color: Color, board_state: &BoardState, legal_moves: &Vec<Move>, depth: usize) -> i32 {
        let mut score = 0;
        let pieces = board_state.pieces[color.index()];
        for p in Piece::LIST {
            let piece_bb = pieces[p.index()];
            let piece_count = piece_bb.bit_count();
            score += (piece_count * p.value()) as i32;

            if p == Piece::Bishop {
                score += Self::DOUBLE_BISHOP_VALUE;
            }

            if board_state.castling.castled(color) {
                score += Self::CASTLING_VALUE;
            }

            let pawns = pieces[Piece::Pawn.index()];
            let mut iter = pawns.clone();
            let direction = color.factor() * (-1);
            loop {
                if iter.is_empty() { break; }
                let pawn_position = iter.lsb() as i32;
                iter = BitBoard::from(iter.raw() & (iter.raw() - 1));
                if !pawns.is_bit_set((pawn_position + direction * 9) as u64) {
                    score += Self::ISOLATED_PAWN_VALUE;
                }

                if !pawns.is_bit_set((pawn_position + direction * 7) as u64) {
                    score += Self::ISOLATED_PAWN_VALUE;
                }
            }

            score += legal_moves.len() as i32;
        }

        return score;
    }
}