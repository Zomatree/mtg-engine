pub enum GameFormat {
    Standard,
    Commander
}

pub enum GameError {
    NotEnoughMana(CardColours)
}

pub enum CardColours {
    White,
    Blue,
    Black,
    Red,
    Green,
    Colourless
}

pub enum Steps {
    Begin,
    Untap,
    Upkeep,
    Draw,
    Main1,
    DeclearAttackers,
    DeclearBlockers,
    Damage,
    Main2,
    End,
    Cleanup
}
