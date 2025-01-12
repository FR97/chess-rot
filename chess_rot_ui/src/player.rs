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
    pub white_max_time: f32
}

impl Default for PlayerConfig {
    fn default() -> Self {
        Self {
            white_player: Human,
            black_player: Human,
        }
    }
}
