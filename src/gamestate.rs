use crate::position;

pub struct GameState {
    game_position: position::Position,
    pub debug: bool,
}

impl GameState {
    pub fn from() -> GameState {
        GameState {
            game_position: position::Position::from(
                "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
            ),
            debug: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_debug_on() {
        let mut game_state = GameState::from();
        game_state.debug = true;

        assert_eq!(game_state.debug, true);
    }

    #[test]
    fn test_set_debug_off() {
        let mut game_state = GameState::from();
        game_state.debug = false;

        assert_eq!(game_state.debug, false);
    }
}
