use crate::{Player, GameResult};
use crate::enums::Steps;
use crate::cards::Card;

use std::collections::VecDeque;

pub enum Action {
    Cast(Card),
}

pub struct GameState {
    pub current_player: Player,
    pub current_step: Steps,
    pub stack: VecDeque<Action>
}


impl GameState {
    pub fn add_to_stack(&mut self, action: Action) -> GameResult {
        // todo splitsecond and etc that stop stack from increasing

        Ok(self.stack.push_front(action))
    }
    pub fn pop_from_stack(&mut self) -> Option<Action> {
        self.stack.pop_front()
    }
}
