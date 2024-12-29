use crate::chess::{BoardState, Color, GameError, Move, MoveType, Piece};

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum GameResult {
    Win(Color),
    Draw,
}

pub struct Game {
    pub result: Option<GameResult>,
    initial_state: BoardState,
    move_history: Vec<(BoardState, Move)>,
}

impl Game {
    const MAX_MOVE_COUNT: usize = 6144;

    pub fn new() -> Game {
        return Game {
            initial_state: BoardState::default(),
            move_history: Vec::with_capacity(256),
            result: None,
        };
    }

    pub fn from_fen(fen: &str) -> Result<Game, GameError> {
        return match BoardState::from_fen(fen) {
            Ok(board_state) => Ok(Game {
                initial_state: board_state,
                move_history: Vec::with_capacity(256),
                result: None,
            }),
            Err(err) => Err(err),
        };
    }

    pub fn is_finished(&self) -> bool {
        return self.result.is_some();
    }

    // pub fn make_move(&mut self, m: Move)  {
    //     if (m.get_type() == MoveType::Invalid) {
    //     } else if (m.get_piece() == Piece::None) {}
    // }


    // pub fn undo_last_move(&mut self) -> Result<(Move, BoardState), Err>{}
}
