pub struct Position {
    pieces: [u64; 14],
}

impl Position {
    pub fn from() -> Position {
        Position { pieces: [0; 14] }
    }
}
