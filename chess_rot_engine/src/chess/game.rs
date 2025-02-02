use crate::chess::board_state::BoardIterator;
use crate::chess::{BoardState, Color, GameError, Move, MoveType, Piece, Square};
use crate::chess::ai::ai_strategy::AiStrategy;
use crate::chess::move_provider::{MoveProvider, PreGeneratedMoveProvider};

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum GameResult {
    Win(Color),
    Draw,
}

//#[derive(Debug, Clone)]
pub struct Game {
    pub result: Option<GameResult>,
    pub current_state: BoardState,
    pub generating_moves: bool,
    initial_state: BoardState,
    move_history: Vec<(BoardState, Move)>,
}

impl Game {
    const MAX_MOVE_COUNT: usize = 6144;

    pub fn new() -> Game {
        return Game {
            initial_state: BoardState::default(),
            current_state: BoardState::default(),
            generating_moves: false,
            move_history: Vec::with_capacity(256),
            result: None,
        };
    }

    pub fn from_fen(fen: &str) -> Result<Game, GameError> {
        return match BoardState::from_fen(fen) {
            Ok(board_state) => Ok(Game {
                initial_state: board_state,
                current_state: board_state,
                generating_moves: false,
                move_history: Vec::with_capacity(256),
                result: None,
            }),
            Err(err) => Err(err),
        };
    }

    pub fn to_fen(&self) -> String {
        self.current_state.to_fen()
    }

    pub fn is_finished(&self) -> bool {
        return self.result.is_some();
    }

    pub fn board_iter(&self) -> BoardIterator {
        return BoardIterator::for_state(self.current_state);
    }

    pub fn possible_moves_for_position(&self, position: u64) -> Vec<Move> {
        let piece = self.current_state.get_piece_at(position);
        if piece.is_none() {
            return Vec::new();
        }

        // let mut all_possible = self.move_provider.get_available_moves(self.current_state, position as u64);


        return Vec::new();
    }

    pub fn generate_legal_moves(&self) -> Vec<Move> {
        return MoveProvider::INSTANCE.legal_moves(&self.current_state);
    }

    pub fn make_move(&mut self, m: Move) -> Option<GameError> {
        // if (m.get_type() == MoveType::Invalid) {
        //     return Some(GameError::InvalidMoveError);
        // } else if (m.get_piece() == Piece::None) {
        //     return Some(GameError::InvalidMoveError);
        // }
        self.move_history.push((self.current_state.clone(), m));
        self.current_state = self.current_state.make_move(m);
        return None;
    }

    // pub fn undo_last_move(&mut self) -> Result<(Move, BoardState), Err>{}
}


