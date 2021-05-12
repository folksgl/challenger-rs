// position.rs will primarily serve to expose the Position struct. A Position
// represents a single chess position. It includes fields representing the
// color to move, current piece positions, castling rights, etc. A Position
// serves as a complete snapshot of a point in time of a chess match.

const A_FILE: u64 = 0x0101010101010101;
const B_FILE: u64 = 0x0202020202020202;
const C_FILE: u64 = 0x0404040404040404;
const D_FILE: u64 = 0x0808080808080808;
const E_FILE: u64 = 0x1010101010101010;
const F_FILE: u64 = 0x2020202020202020;
const G_FILE: u64 = 0x4040404040404040;
const H_FILE: u64 = 0x8080808080808080;

const RANK_1: u64 = 0x00000000000000FF;
const RANK_2: u64 = 0x000000000000FF00;
const RANK_3: u64 = 0x0000000000FF0000;
const RANK_4: u64 = 0x00000000FF000000;
const RANK_5: u64 = 0x000000FF00000000;
const RANK_6: u64 = 0x0000FF0000000000;
const RANK_7: u64 = 0x00FF000000000000;
const RANK_8: u64 = 0xFF00000000000000;

pub struct Position {
    pieces: [u64; 14],
}

impl Position {
    pub fn from() -> Position {
        Position { pieces: [0; 14] }
    }
}
