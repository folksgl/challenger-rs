use crate::position;

pub struct GameState {
    game_position: position::Position,
    debug: bool,
}

impl GameState {
    pub fn from() -> GameState {
        GameState {
            game_position: position::Position::from(),
            debug: false,
        }
    }
}
