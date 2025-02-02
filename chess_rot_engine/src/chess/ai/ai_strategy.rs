use std::fmt::format;
use std::time::Instant;
use crate::chess::{BoardState, Color, GameError, Move, MoveType, Piece, Square, SquareLabel};

use openai_api_rust::*;
use openai_api_rust::chat::*;
use openai_api_rust::completions::*;
use crate::chess::ai::evaluator::Evaluator;
use crate::chess::move_provider::MoveProvider;

pub trait AiStrategy {
    fn find_optimal_move(&mut self, board: &BoardState, legal_moves: &Vec<Move>) -> Result<Move, GameError>;
}

#[derive(Debug, PartialEq, Clone)]
pub struct OpenAi {
    pub api_key: String,
}

impl OpenAi {
    pub fn new(api_key: &str) -> Self {
        Self {
            api_key: api_key.to_string(),
        }
    }

    fn prepare_body(prompt: &str) -> ChatBody {
        return ChatBody {
            model: "gpt-4o".to_string(),
            max_tokens: Some(7),
            temperature: Some(0_f32),
            top_p: Some(0_f32),
            n: Some(2),
            stream: Some(false),
            stop: None,
            presence_penalty: None,
            frequency_penalty: None,
            logit_bias: None,
            user: None,
            messages: vec![Message { role: Role::User, content: prompt.to_string() }],
        };
    }

    fn extract_response(str: &str) -> Option<(&str, &str)> {
        if str.len() == 4 {
            let square_from = &str[..2];
            let square_to = &str[2..4];
            return Some((square_from, square_to));
        } else if str.len() == 5 {
            let square_from = &str[..2];
            let square_to = &str[3..5];
            return Some((square_from, square_to));
        } else {
            return None;
        }
    }

    fn create_move(board_state: BoardState, from: u64, to: u64, res: &str) -> Result<Move, GameError> {
        let (p_from, c_from) = board_state.get_piece_at(from)
            .ok_or(GameError::OpenAiResponseError(format!("OpenAi Response: {} is not a valid move", res)))?;
        let piece_to = board_state.get_piece_at(to);

        let mut move_type = MoveType::Push;
        let mut p_to = Piece::None;
        if let Some((pto, cto)) = piece_to {
            move_type = MoveType::Capture;
            p_to = pto;
        }

        if p_from == Piece::Pawn {
            let offset = if board_state.on_move() == Color::White { 16 } else { -16 };
            let idx = from as i32 + offset;
            if idx as u64 == to {
                move_type = MoveType::PawnJump;
            }
        }

        let m = Move::new(move_type, from as u64, to as u64, p_from, c_from, p_to);
        println!("created move: {:?}", m);
        if !board_state.can_make_move(m) {
            return Err(GameError::OpenAiResponseError(format!("OpenAi Response: {} is not a valid move", res)));
        }

        println!("Move is valid!");

        return Ok(m);
    }
}

impl AiStrategy for OpenAi {
    fn find_optimal_move(&mut self, board: &BoardState, legal_moves: &Vec<Move>) -> Result<Move, GameError> {
        let fen = board.to_fen();
        let initial_prompt = format!("I would like play chess with you where I will send current chess game state in FEN format and I want you to give me next optimal move in format square from square to,\
        Please only give optimal move without explanation. FEN position: {}", fen);

        let invalid_response_prompt = format!("Move that you have suggested is invalid please suggest new valid chess move for FEN position: {}, make sure that your response is in correct format like a1b2", fen);

        let mut msg = "".to_string();
        for i in 0..5 {
            println!("OpenAI Sending FEN: {}", fen);
            let auth = Auth::new(&self.api_key);
            let openai = OpenAI::new(auth, "https://api.openai.com/v1/");
            let prompt = if i == 0 { &initial_prompt } else { &invalid_response_prompt };
            let body = Self::prepare_body(prompt);
            let rs = openai.chat_completion_create(&body);
            let choice = rs.unwrap().choices;
            let message = &choice[0].message.as_ref().unwrap().content;
            msg = message.clone();
            println!("OpenAI Response: {}", message);

            if let Some((square_from, square_to)) = Self::extract_response(&message) {
                println!("From {} to {}", square_from, square_to);
                let from = Square::from_string(square_from)
                    .ok_or(GameError::OpenAiResponseError(format!("OpenAi Response: {}", message)))?;
                let to = Square::from_string(square_to)
                    .ok_or(GameError::OpenAiResponseError(format!("OpenAi Response: {}", message)))?;

                let mut valid_move = legal_moves.iter()
                    .find(|m| m.get_from() == from && m.get_to() == to);
                if valid_move.is_some() {
                    return Ok(valid_move.unwrap().clone());
                }
            }
        }

        return Err(GameError::OpenAiResponseError(format!("OpenAi Response: {}", msg)));
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Minimax {
    evaluator: Evaluator,
    processed_states_counter: u64,
    max_depth: usize,
    max_time: f32,
}

impl Minimax {
    const MAX: i32 = 500000;
    const MIN: i32 = -500000;

    pub const fn new(evaluator: Evaluator, max_depth: usize, max_time: f32) -> Self {
        Self { evaluator, processed_states_counter: 0, max_depth, max_time }
    }

    fn min(&mut self, board_state: &BoardState, depth: usize, alpha: i32, beta: i32) -> i32 {
        let legal_moves = MoveProvider::INSTANCE.legal_moves(board_state);
        if depth == 0 {
            self.processed_states_counter += 1;
            return self.evaluator.evaluate(board_state, &Vec::new(), depth);
        }

        let mut worst = Self::MAX;
        let mut _beta = beta;
        for m in legal_moves {
            let next_state = board_state.make_move(m);
            let current = self.max(&next_state, depth - 1, alpha, _beta);
            if current <= worst {
                worst = current;
            }

            if current <= beta {
                _beta = current;
            }

            if _beta < alpha {
                break;
            }
        }

        return worst;
    }

    fn max(&mut self, board_state: &BoardState, depth: usize, alpha: i32, beta: i32) -> i32 {
        let legal_moves = MoveProvider::INSTANCE.legal_moves(board_state);
        if depth == 0 {
            self.processed_states_counter += 1;
            return self.evaluator.evaluate(board_state, &Vec::new(), depth);
        }

        let mut best = Self::MIN;
        let mut _alpha = alpha;
        for m in legal_moves {
            let next_state = board_state.make_move(m);
            let current = self.min(&next_state, depth - 1, _alpha, beta);
            if current >= best {
                best = current;
            }

            if current >= alpha {
                _alpha = current;
            }

            if beta < _alpha {
                break;
            }
        }

        return best;
    }
}

impl AiStrategy for Minimax {
    fn find_optimal_move(&mut self, board: &BoardState, legal_moves: &Vec<Move>) -> Result<Move, GameError> {
        let mut best = Self::MIN;
        let mut worst = Self::MAX;
        let mut current = 0;
        let starting_moves = MoveProvider::INSTANCE.legal_moves(board);
        let move_count = starting_moves.len();
        let counter = 1;
        let mut best_move = None;
        let start = Instant::now();
        for m in starting_moves {
            println!("Looking for move {} ({} to {}) out of {}", counter, m.get_from(), m.get_to(), move_count);
            let next_state = board.make_move(m);
            current = match board.color_on_move {
                Color::White => self.min(&next_state, self.max_depth - 1, best, worst),
                Color::Black => self.max(&next_state, self.max_depth - 1, best, worst),
            };

            if board.color_on_move == Color::White && current >= best {
                best_move = Some(m);
                best = current;
            } else if board.color_on_move == Color::Black && current <= best {
                best_move = Some(m);
                worst = current;
            }
        }

        if let Some(m) = best_move {
            println!("Found best move {} {} in {}ms", m.get_from(), m.get_to(), Instant::now().duration_since(start).as_millis());
        }

        return best_move.ok_or(GameError::NoPossibleMoveError);
    }
}

#[cfg(test)]
mod test {
    use crate::chess::ai::ai_strategy::{AiStrategy, OpenAi};
    use crate::chess::BoardState;

    #[test]
    fn openai_test() {
        // let open_ai = OpenAi::new("");
        // let m = open_ai.find_optimal_move(BoardState::default(), &Vec::new());
    }
}
