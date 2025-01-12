use crate::chess::{BoardState, Move};
use crate::chess::ai::ai_strategy::{ChatGPTLlm, Minimax};

pub enum AiType {
    Minimax,
    LLM,
}


#[derive(Debug, PartialEq, Clone)]
pub struct AiMoveProvider {
    minimax: Minimax,
    llm: ChatGPTLlm,
}

impl AiMoveProvider {

    fn new(max_depth: usize, max_time: usize, api_key: &str) -> Self {
        Self{
            minimax: Minimax::new(max_depth, max_time),
            llm: ChatGPTLlm::new(api_key),
        }
    }

    fn find_optimal_move(self, ai_type: AiType, board: BoardState) -> Option<Move> {
        return None
    }

}