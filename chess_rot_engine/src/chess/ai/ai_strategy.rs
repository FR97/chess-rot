use crate::chess::{BoardState, Move};


pub trait AiStrategy {

    fn find_optimal_move(board: BoardState) -> Option<Move>;

}

#[derive(Debug, PartialEq, Clone)]
pub struct ChatGPTLlm {
    api_key: String,
}

impl ChatGPTLlm {
    pub fn new(api_key: &str) -> Self {
        Self {
            api_key: api_key.to_string(),
        }
    }
}

impl AiStrategy for ChatGPTLlm {
    fn find_optimal_move(board: BoardState) -> Option<Move> {
        todo!()
    }
}


#[derive(Debug, PartialEq, Clone)]
pub struct Minimax {
    max_depth: usize,
    max_time: usize,
}


impl Minimax {

    pub fn new(max_depth: usize, max_time: usize) -> Self {
        Self { max_depth, max_time }
    }

}

impl AiStrategy for Minimax {
    fn find_optimal_move(board: BoardState) -> Option<Move> {
        return None
    }
}
