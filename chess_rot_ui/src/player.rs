use crate::chess::ai::AiMoveProvider;
use crate::player::Player::Human;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Player {
    Human,
    Minimax,
    LLM,
}

#[derive(Debug, PartialEq, Clone)]
pub struct PlayerConfig {
    pub white_player: Player,
    pub black_player: Player,
    pub white_max_depth: usize,
    pub black_max_depth: usize,
    pub white_max_time: f32,
    pub black_max_time: f32,
    pub white_api_key: String,
    pub black_api_key: String,
    pub white_ai_start: bool,
    pub black_ai_start: bool,
}

impl Default for PlayerConfig {
    fn default() -> Self {
        Self {
            white_player: Human,
            black_player: Human,
            white_max_depth: 5,
            black_max_depth: 5,
            white_max_time: 5.0,
            black_max_time: 5.0,
            white_api_key: "".to_string(),
            black_api_key: "".to_string(),
            white_ai_start: false,
            black_ai_start: false
        }
    }
}
