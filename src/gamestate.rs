use crate::position;

pub struct GameState {
    pub game_position: position::Position,
    pub debug: bool,
}

impl GameState {
    pub fn new() -> GameState {
        GameState {
            game_position: position::Position::new(),
            debug: false,
        }
    }

    pub fn reset_game(&mut self) {
        self.game_position = position::Position::new();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_debug_on() {
        let mut game_state = GameState::new();
        game_state.debug = true;

        assert_eq!(game_state.debug, true);
    }

    #[test]
    fn test_set_debug_off() {
        let mut game_state = GameState::new();
        game_state.debug = false;

        assert_eq!(game_state.debug, false);
    }
}
