// position.rs will primarily serve to expose the Position struct. A Position
// represents a single chess position. It includes fields representing the
// color to move, current piece positions, castling rights, etc. A Position
// serves as a complete snapshot of a point in time of a chess match.

// FILE constants: bitboards representing their respective files of the board with
// 1's set in the bit positions for the file, and 0's otherwise.
const A_FILE: u64 = 0x0101010101010101;
const B_FILE: u64 = 0x0202020202020202;
const C_FILE: u64 = 0x0404040404040404;
const D_FILE: u64 = 0x0808080808080808;
const E_FILE: u64 = 0x1010101010101010;
const F_FILE: u64 = 0x2020202020202020;
const G_FILE: u64 = 0x4040404040404040;
const H_FILE: u64 = 0x8080808080808080;

// RANK constants: bitboards representing their respective ranks of the board with
// 1's set in the bit positions for the rank, and 0's otherwise.
const RANK_1: u64 = 0x00000000000000FF;
const RANK_2: u64 = 0x000000000000FF00;
const RANK_3: u64 = 0x0000000000FF0000;
const RANK_4: u64 = 0x00000000FF000000;
const RANK_5: u64 = 0x000000FF00000000;
const RANK_6: u64 = 0x0000FF0000000000;
const RANK_7: u64 = 0x00FF000000000000;
const RANK_8: u64 = 0xFF00000000000000;

// Piece constants for indexing the 'pieces' field of a position
const W_PAWN: usize = 0;
const W_ROOK: usize = 1;
const W_KNIGHT: usize = 2;
const W_BISHOP: usize = 3;
const W_QUEEN: usize = 4;
const W_KING: usize = 5;
const W_PIECES: usize = 6;

const B_PAWN: usize = 7;
const B_ROOK: usize = 8;
const B_KNIGHT: usize = 9;
const B_BISHOP: usize = 10;
const B_QUEEN: usize = 11;
const B_KING: usize = 12;
const B_PIECES: usize = 13;

pub struct Position {
    pieces: [u64; 14],
    passant_sq: u64,
    w_kingside_castle: bool,
    w_queenside_castle: bool,
    b_kingside_castle: bool,
    b_queenside_castle: bool,
    is_white_move: bool,
    hlf_clock: u8,
    full_num: u8,
}

impl Position {
    pub fn from() -> Position {
        // Construct the starting position. Equivalent to:
        // from("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1")
        Position {
            pieces: [
                0x000000000000FF00,
                0x0000000000000081,
                0x0000000000000042,
                0x0000000000000024,
                0x0000000000000008,
                0x0000000000000010,
                0x000000000000FFFF,
                0x00FF000000000000,
                0x8100000000000000,
                0x4200000000000000,
                0x2400000000000000,
                0x0800000000000000,
                0x1000000000000000,
                0xFFFF000000000000,
            ],
            passant_sq: 0,
            w_kingside_castle: true,
            w_queenside_castle: true,
            b_kingside_castle: true,
            b_queenside_castle: true,
            is_white_move: true,
            hlf_clock: 0,
            full_num: 1,
        }
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    macro_rules! test_start_position {
        ($test_name:ident, $input_str:expr, $expected:literal) => {
            #[test]
            fn $test_name() {
                assert_eq!(Position::from().pieces[$input_str], $expected);
            }
        };
    }

    test_start_position!(test_start_w_pawn, W_PAWN, 0x000000000000FF00);
    test_start_position!(test_start_w_rook, W_ROOK, 0x0000000000000081);
    test_start_position!(test_start_w_knight, W_KNIGHT, 0x0000000000000042);
    test_start_position!(test_start_w_bishop, W_BISHOP, 0x0000000000000024);
    test_start_position!(test_start_w_queen, W_QUEEN, 0x0000000000000008);
    test_start_position!(test_start_w_king, W_KING, 0x0000000000000010);
    test_start_position!(test_start_w_pieces, W_PIECES, 0x000000000000FFFF);
    test_start_position!(test_start_b_pawn, B_PAWN, 0x00FF000000000000);
    test_start_position!(test_start_b_rook, B_ROOK, 0x8100000000000000);
    test_start_position!(test_start_b_knight, B_KNIGHT, 0x4200000000000000);
    test_start_position!(test_start_b_bishop, B_BISHOP, 0x2400000000000000);
    test_start_position!(test_start_b_queen, B_QUEEN, 0x0800000000000000);
    test_start_position!(test_start_b_king, B_KING, 0x1000000000000000);
    test_start_position!(test_start_b_pieces, B_PIECES, 0xFFFF000000000000);
}
