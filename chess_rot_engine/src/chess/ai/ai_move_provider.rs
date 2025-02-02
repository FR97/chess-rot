use crate::chess::{BoardState, Move};
use crate::chess::ai::ai_strategy::{OpenAi, Minimax};
use crate::chess::ai::evaluator::Evaluator;

pub enum AiType {
    Minimax,
    LLM,
}


#[derive(Debug, PartialEq, Clone)]
pub struct AiMoveProvider {
    minimax: Minimax,
    llm: OpenAi,
}

impl AiMoveProvider {
    fn new(max_depth: usize, max_time: f32, api_key: &str) -> Self {
        Self {
            minimax: Minimax::new(Evaluator::new(), max_depth, max_time),
            llm: OpenAi::new(api_key),
        }
    }

    fn find_optimal_move(self, ai_type: AiType, board: &BoardState) -> Option<Move> {
        return None;
    }
}