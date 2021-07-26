use crate::state::GameState;
use crate::enums::CardColours;
use crate::cards::CardData;
use crate::{Player, GameResult};

pub trait Ruleset {
    fn new(state: &GameState) -> Self where Self: Sized;
    fn get_deck_limit(&self) -> (u8, u8);
    fn get_hand_limit(&self) -> Option<u8>;
    fn get_extra_land_mana(&self, colour: Vec<CardColours>) -> Vec<CardColours>;
    fn adjust_card_cost(&self, card: &dyn CardData) -> Vec<CardColours>;
    fn can_cast(&self, card: &dyn CardData, player: &Player) -> GameResult where Self: Sized;
}
