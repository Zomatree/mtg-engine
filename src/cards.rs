use crate::state::GameState;
use crate::{GameResult, Player};
use crate::enums::CardColours;
use crate::rulesets::Ruleset;
use std::boxed::Box;

macro_rules! not_castable {
    () => {
        fn cast<R: Ruleset>(&self, _state: &mut GameState, _ruleset: &R, _player: &mut Player) -> GameResult {
            Ok(())
        }
        fn castable(&self) -> bool { false }
    };
}

macro_rules! not_playable {
    () => {
        fn play<R: Ruleset>(&self, _state: &mut GameState, _ruleset: &R, _player: &mut Player) -> GameResult {
            Ok(())
        };
        fn playable(&self) -> bool { false };
    };
}

pub trait CardData {
    fn cost(&self) -> Vec<CardColours>;
    fn name(&self) -> &'static str;
    fn tap(&mut self, state: &mut GameState, ruleset: &dyn Ruleset) -> GameResult;
    fn cast<R: Ruleset>(&self, state: &mut GameState, ruleset: &R, player: &mut Player) -> GameResult where Self: Sized;
    fn castable(&self) -> bool;
    fn play<R: Ruleset>(&self, state: &mut GameState, ruleset: &R, player: &mut Player) -> GameResult where Self: Sized;
    fn playable(&self) -> bool;

}

pub struct Card(pub Box<dyn CardData>);

impl<T: CardData + 'static> From<T> for Card {
    fn from(v: T) -> Self {
        Self(Box::new(v))
    }
}

struct Island;
impl CardData for Island {
    fn name(&self) -> &'static str { "Island" }
    fn cost(&self) -> Vec<CardColours> { vec![] }
    not_castable!();
    fn play<R: Ruleset>(&self, state: &mut GameState, ruleset: &R, player: &mut Player) -> GameResult {
        todo!()
    }
    fn playable(&self) -> bool { true }
    fn tap(&mut self, state: &mut GameState, ruleset: &dyn Ruleset) -> GameResult {
        state.current_player.mana.add(vec![CardColours::Blue], ruleset);
        Ok(())
    }
}
