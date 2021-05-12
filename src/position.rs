// position.rs will primarily serve to expose the Position struct. A Position
// represents a single chess position. It includes fields representing the
// color to move, current piece positions, castling rights, etc. A Position
// serves as a complete snapshot of a point in time of a chess match.

pub struct Position {
    pieces: [u64; 14],
}

impl Position {
    pub fn from() -> Position {
        Position { pieces: [0; 14] }
    }
}
