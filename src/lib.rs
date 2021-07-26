mod rulesets;
mod enums;
mod state;
mod cards;

use std::{boxed::Box, ops::Deref};

type GameResult = Result<(), enums::GameError>;

pub struct ManaStorage {
    pub white: u8,
    pub blue: u8,
    pub black: u8,
    pub red: u8,
    pub green: u8,
    pub colourless: u8,
}

impl ManaStorage {
    pub fn new() -> Self {
        ManaStorage {white: 0, blue: 0, black: 0, red: 0, green: 0, colourless: 0}
    }

    pub fn get_mut(&mut self) -> &mut Self {
        self
    }

    pub fn inc(&mut self, colours: Vec<enums::CardColours>) {
        for colour in colours.iter() {
            match colour {
                enums::CardColours::Black => self.black += 1,
                enums::CardColours::White => self.white += 1,
                enums::CardColours::Blue => self.black += 1,
                enums::CardColours::Red => self.red += 1,
                enums::CardColours::Green => self.green += 1,
                enums::CardColours::Colourless => self.colourless += 1
            }
        }
    }

    pub fn dec(&mut self, colours: Vec<enums::CardColours>) {
        for colour in colours.iter() {
            match colour {
                enums::CardColours::Black => self.black -= 1,
                enums::CardColours::White => self.white -= 1,
                enums::CardColours::Blue => self.black -= 1,
                enums::CardColours::Red => self.red -= 1,
                enums::CardColours::Green => self.green -= 1,
                enums::CardColours::Colourless => self.colourless -= 1
            }
        }
    }

    pub fn add(&mut self, colours: Vec<enums::CardColours>, ruleset: &dyn rulesets::Ruleset) {
        let colours = ruleset.get_extra_land_mana(colours);
        self.inc(colours);
    }

    pub fn clear(&mut self) {
        self.white = 0;
        self.blue = 0;
        self.black = 0;
        self.red = 0;
        self.green = 0;
        self.colourless = 0;
    }

    pub fn remove_cost(&mut self, card: cards::Card, ruleset: &dyn rulesets::Ruleset) -> GameResult {
        let cost = ruleset.adjust_card_cost(card.0.deref());

        let mut white = self.white as i8;
        let mut blue = self.blue as i8;
        let mut black = self.black as i8;
        let mut red = self.red as i8;
        let mut green = self.green as i8;
        let mut colourless = self.colourless as i8;

        for colour in cost.iter() {
            match colour {
                enums::CardColours::White => {
                    white -= 1;
                    if white < 0 {
                        return Err(enums::GameError::NotEnoughMana(enums::CardColours::White))
                    }
                },
                enums::CardColours::Black => {
                    black -= 1;
                    if black < 0 {
                        return Err(enums::GameError::NotEnoughMana(enums::CardColours::Black))
                    }
                },
                enums::CardColours::Blue => {
                    blue -= 1;
                    if blue < 0 {
                        return Err(enums::GameError::NotEnoughMana(enums::CardColours::Blue))
                    }
                },
                enums::CardColours::Red => {
                    red -= 1;
                    if red < 0 {
                        return Err(enums::GameError::NotEnoughMana(enums::CardColours::Red))
                    }
                },
                enums::CardColours::Green => {
                    green -= 1;
                    if green < 0 {
                        return Err(enums::GameError::NotEnoughMana(enums::CardColours::Green))
                    }
                },
                enums::CardColours::Colourless => {
                    colourless -= 1;
                    if colourless < 0 {
                        return Err(enums::GameError::NotEnoughMana(enums::CardColours::Colourless))
                    }
                },
            }
        }

        self.dec(cost);
        Ok(())
    }
}

pub struct Player {
    pub deck: Vec<Box<cards::Card>>,
    pub mana: ManaStorage,
    pub life: u8
}

impl Player {
    pub fn play_card(&mut self, card: cards::Card, ruleset: &dyn rulesets::Ruleset) -> GameResult {
        todo!()        
    }

    pub fn cast_card(&mut self, card: cards::Card, ruleset: &dyn rulesets::Ruleset) -> GameResult {
        todo!()
    }
}

pub struct GameMetadata<T: rulesets::Ruleset> {
    format: enums::GameFormat,
    ruleset: T
}

pub struct Game<T: rulesets::Ruleset> {
    players: Vec<Player>,
    metadata: GameMetadata<T>
}
