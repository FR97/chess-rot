use crate::chess::{BoardState, Move};


#[derive(Debug, Clone)]
pub struct MoveStack {
    initial_state: BoardState,
    move_list: Vec<(Move, BoardState)>,
}

impl MoveStack {

    pub fn with_initial(initial_state: BoardState) -> MoveStack {
        return Self {
            initial_state,
            move_list: Vec::with_capacity(256),
        };
    }

    pub fn initial_state(&self) -> BoardState {
        return self.initial_state;
    }

    pub fn latest_state(&self) -> BoardState {
        if (self.move_list.is_empty()) {
            return self.initial_state;
        }
        return self.move_list.last().unwrap().1;
    }

    pub fn push(&mut self, m: Move, new_state: BoardState) {
        self.move_list.push((m, new_state))
    }

    pub fn pop(&mut self) -> Option<(Move, BoardState)> {
        return self.move_list.pop();
    }
}

impl Default for MoveStack {
    fn default() -> Self {
        return Self {
            initial_state: BoardState::default(),
            move_list: Vec::with_capacity(256),
        };
    }
}