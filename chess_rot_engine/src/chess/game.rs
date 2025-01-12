use crate::chess::board_state::BoardIterator;
use crate::chess::{BoardState, Color, GameError, Move, MoveType, Piece, Square};
use crate::chess::ai::ai_strategy::AiStrategy;
use crate::chess::move_provider::{MoveProvider, PreGeneratedMoveProvider};

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum GameResult {
    Win(Color),
    Draw,
}

#[derive(Debug, Clone)]
pub struct Game {
    pub result: Option<GameResult>,
    initial_state: BoardState,
    current_state: BoardState,
    move_history: Vec<(BoardState, Move)>,
    move_provider: PreGeneratedMoveProvider,
}

impl Game {
    const MAX_MOVE_COUNT: usize = 6144;

    pub fn new() -> Game {
        return Game {
            initial_state: BoardState::default(),
            current_state: BoardState::default(),
            move_history: Vec::with_capacity(256),
            result: None,
            move_provider: PreGeneratedMoveProvider::default()
        };
    }

    pub fn from_fen(fen: &str) -> Result<Game, GameError> {
        return match BoardState::from_fen(fen) {
            Ok(board_state) => Ok(Game {
                initial_state: board_state,
                current_state: board_state,
                move_history: Vec::with_capacity(256),
                result: None,
                move_provider: PreGeneratedMoveProvider::default()
            }),
            Err(err) => Err(err),
        };
    }

    pub fn is_finished(&self) -> bool {
        return self.result.is_some();
    }

    pub fn board_iter(&self) -> BoardIterator {
        let state = self
            .move_history
            .last()
            .map(|s| s.0)
            .unwrap_or(self.initial_state);
        return BoardIterator::for_state(state);
    }

    pub fn possible_moves_for_position(self, position: u8) -> Vec<Move> {
        let piece = self.current_state.get_piece_at(position);
        if piece.is_none() {
            return Vec::new();
        }

        let mut all_possible = self.move_provider.get_available_moves(self.current_state, position as u64);



        return all_possible;
    }

    // pub fn make_move(&mut self, m: Move)  {
    //     if (m.get_type() == MoveType::Invalid) {
    //     } else if (m.get_piece() == Piece::None) {}
    // }

    // pub fn undo_last_move(&mut self) -> Result<(Move, BoardState), Err>{}
}


