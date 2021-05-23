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

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Position {
    pieces: [u64; 14], // Bitboards
    passant_sq: u64,   // En Passant square

    // Castling rights
    w_king_castle: bool,
    w_queen_castle: bool,
    b_king_castle: bool,
    b_queen_castle: bool,

    is_white_move: bool, // Side to move
    hlf_clock: u8,       // Halfmove clock
    full_num: u8,        // Fullmove number
}

impl Position {
    pub fn from(fen: &str) -> Position {
        let mut fen_tokens = fen.split_whitespace();

        // Fen string: Piece positions
        let piece_string = fen_tokens
            .next()
            .unwrap()
            .split('/')
            .flat_map(|x| x.chars().rev());

        let mut square_num: isize = 63;
        let mut pieces = [0; 14];

        for piece in piece_string {
            match piece {
                'P' => pieces[W_PAWN] |= 1u64 << square_num,
                'R' => pieces[W_ROOK] |= 1u64 << square_num,
                'N' => pieces[W_KNIGHT] |= 1u64 << square_num,
                'B' => pieces[W_BISHOP] |= 1u64 << square_num,
                'Q' => pieces[W_QUEEN] |= 1u64 << square_num,
                'K' => pieces[W_KING] |= 1u64 << square_num,
                'p' => pieces[B_PAWN] |= 1u64 << square_num,
                'r' => pieces[B_ROOK] |= 1u64 << square_num,
                'n' => pieces[B_KNIGHT] |= 1u64 << square_num,
                'b' => pieces[B_BISHOP] |= 1u64 << square_num,
                'q' => pieces[B_QUEEN] |= 1u64 << square_num,
                'k' => pieces[B_KING] |= 1u64 << square_num,
                '2' => square_num -= 1,
                '3' => square_num -= 2,
                '4' => square_num -= 3,
                '5' => square_num -= 4,
                '6' => square_num -= 5,
                '7' => square_num -= 6,
                '8' => square_num -= 7,
                _ => (),
            }
            square_num -= 1
        }
        for i in 0..6 {
            pieces[W_PIECES] |= pieces[i];
            pieces[B_PIECES] |= pieces[i + 7];
        }

        // Fen string: Active color
        let is_white_move = fen_tokens.next().unwrap() == "w";

        // Fen string: Castling availability
        let castle_rights = fen_tokens.next().unwrap();

        // Fen string: En passant target square
        let passant_sq_str = fen_tokens.next().unwrap();

        // Default to no passant sq
        let mut passant_sq: u64 = 0;
        if passant_sq_str.len() != 1 {
            let mut chars = passant_sq_str.chars();
            passant_sq = sq_to_bitboard(chars.next().unwrap(), chars.next().unwrap());
        }

        // Fen string: Halfmove clock
        let hlf_clock = fen_tokens.next().unwrap().parse().unwrap();

        // Fen string: Fullmove number
        let full_num = fen_tokens.next().unwrap().parse().unwrap();

        Position {
            pieces,
            passant_sq,
            w_king_castle: castle_rights.contains('K'),
            w_queen_castle: castle_rights.contains('Q'),
            b_king_castle: castle_rights.contains('k'),
            b_queen_castle: castle_rights.contains('q'),
            is_white_move,
            hlf_clock,
            full_num,
        }
    }

    pub fn new() -> Position {
        Position::from("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1")
    }

    pub fn play_move(&mut self, move_string: &str) {
        let mut move_chars = move_string.chars();
        let start_square = sq_to_bitboard(move_chars.next().unwrap(), move_chars.next().unwrap());
        let dest_square = sq_to_bitboard(move_chars.next().unwrap(), move_chars.next().unwrap());
        self.pieces = [0; 14];
    }

    pub fn evaluate(self) -> isize {
        if self.pieces[W_KING] == 0 {
            return isize::MIN;
        }
        if self.pieces[B_KING] == 0 {
            return isize::MAX;
        }

        let mut white_evaluation = 0;
        let mut black_evaluation = 0;

        white_evaluation += self.pieces[W_PAWN].count_ones() * 100;
        white_evaluation += self.pieces[W_ROOK].count_ones() * 350;
        white_evaluation += self.pieces[W_KNIGHT].count_ones() * 350;
        white_evaluation += self.pieces[W_BISHOP].count_ones() * 525;
        white_evaluation += self.pieces[W_QUEEN].count_ones() * 1000;

        black_evaluation += self.pieces[B_PAWN].count_ones() * 100;
        black_evaluation += self.pieces[B_ROOK].count_ones() * 350;
        black_evaluation += self.pieces[B_KNIGHT].count_ones() * 350;
        black_evaluation += self.pieces[B_BISHOP].count_ones() * 525;
        black_evaluation += self.pieces[B_QUEEN].count_ones() * 1000;

        white_evaluation as isize - black_evaluation as isize
    }
}

fn sq_to_bitboard(file: char, rank: char) -> u64 {
    let file_squares = file as u32 - 'a' as u32;
    let rank_squares = (rank as u32 - '1' as u32) * 8;
    1u64 << (rank_squares + file_squares)
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    // Test fen piece placement of Position construction
    macro_rules! test_pieces {
        ($test_name:ident, $fen:expr, $piece:expr, $expected:literal) => {
            #[test]
            fn $test_name() {
                assert_eq!(Position::from($fen).pieces[$piece], $expected);
            }
        };
    }

    const startpos: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

    test_pieces!(startpos_w_pawn, startpos, W_PAWN, 0x000000000000FF00);
    test_pieces!(startpos_w_rook, startpos, W_ROOK, 0x0000000000000081);
    test_pieces!(startpos_w_knight, startpos, W_KNIGHT, 0x0000000000000042);
    test_pieces!(startpos_w_bishop, startpos, W_BISHOP, 0x0000000000000024);
    test_pieces!(startpos_w_queen, startpos, W_QUEEN, 0x0000000000000008);
    test_pieces!(startpos_w_king, startpos, W_KING, 0x0000000000000010);
    test_pieces!(startpos_w_pieces, startpos, W_PIECES, 0x000000000000FFFF);
    test_pieces!(startpos_b_pawn, startpos, B_PAWN, 0x00FF000000000000);
    test_pieces!(startpos_b_rook, startpos, B_ROOK, 0x8100000000000000);
    test_pieces!(startpos_b_knight, startpos, B_KNIGHT, 0x4200000000000000);
    test_pieces!(startpos_b_bishop, startpos, B_BISHOP, 0x2400000000000000);
    test_pieces!(startpos_b_queen, startpos, B_QUEEN, 0x0800000000000000);
    test_pieces!(startpos_b_king, startpos, B_KING, 0x1000000000000000);
    test_pieces!(startpos_b_pieces, startpos, B_PIECES, 0xFFFF000000000000);

    const empty_pos: &str = "8/8/8/8/8/8/8/8 w KQkq - 0 1";

    test_pieces!(empty_w_pawn, empty_pos, W_PAWN, 0);
    test_pieces!(empty_w_rook, empty_pos, W_PAWN, 0);
    test_pieces!(empty_w_knight, empty_pos, W_PAWN, 0);
    test_pieces!(empty_w_bishop, empty_pos, W_PAWN, 0);
    test_pieces!(empty_w_queen, empty_pos, W_PAWN, 0);
    test_pieces!(empty_w_king, empty_pos, W_PAWN, 0);
    test_pieces!(empty_w_pieces, empty_pos, W_PAWN, 0);
    test_pieces!(empty_b_pawn, empty_pos, W_PAWN, 0);
    test_pieces!(empty_b_rook, empty_pos, W_PAWN, 0);
    test_pieces!(empty_b_knight, empty_pos, W_PAWN, 0);
    test_pieces!(empty_b_bishop, empty_pos, W_PAWN, 0);
    test_pieces!(empty_b_queen, empty_pos, W_PAWN, 0);
    test_pieces!(empty_b_king, empty_pos, W_PAWN, 0);
    test_pieces!(empty_b_pieces, empty_pos, W_PAWN, 0);

    // 'complex_pos' positions are found here: https://www.chessprogramming.org/Perft_Results
    const complex_pos_2: &str =
        "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1";

    test_pieces!(complex2_w_pawn, complex_pos_2, W_PAWN, 0x81000E700);
    test_pieces!(complex2_w_rook, complex_pos_2, W_ROOK, 0x81);
    test_pieces!(complex2_w_knight, complex_pos_2, W_KNIGHT, 0x1000040000);
    test_pieces!(complex2_w_bishop, complex_pos_2, W_BISHOP, 0x1800);
    test_pieces!(complex2_w_queen, complex_pos_2, W_QUEEN, 0x200000);
    test_pieces!(complex2_w_king, complex_pos_2, W_KING, 0x10);
    test_pieces!(complex2_w_pieces, complex_pos_2, W_PIECES, 0x181024FF91);
    test_pieces!(complex2_b_pawn, complex_pos_2, B_PAWN, 0x2D500002800000);
    test_pieces!(complex2_b_rook, complex_pos_2, B_ROOK, 0x8100000000000000);
    test_pieces!(complex2_b_knight, complex_pos_2, B_KNIGHT, 0x220000000000);
    test_pieces!(complex2_b_bishop, complex_pos_2, B_BISHOP, 0x40010000000000);
    test_pieces!(complex2_b_queen, complex_pos_2, B_QUEEN, 0x0010000000000000);
    test_pieces!(complex2_b_king, complex_pos_2, B_KING, 0x1000000000000000);
    test_pieces!(
        complex2_b_pieces,
        complex_pos_2,
        B_PIECES,
        0x917D730002800000
    );

    const complex_pos_3: &str = "8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1";

    test_pieces!(complex3_w_pawn, complex_pos_3, W_PAWN, 0x200005000);
    test_pieces!(complex3_w_rook, complex_pos_3, W_ROOK, 0x2000000);
    test_pieces!(complex3_w_knight, complex_pos_3, W_KNIGHT, 0);
    test_pieces!(complex3_w_bishop, complex_pos_3, W_BISHOP, 0);
    test_pieces!(complex3_w_queen, complex_pos_3, W_QUEEN, 0);
    test_pieces!(complex3_w_king, complex_pos_3, W_KING, 0x100000000);
    test_pieces!(complex3_w_pieces, complex_pos_3, W_PIECES, 0x302005000);
    test_pieces!(complex3_b_pawn, complex_pos_3, B_PAWN, 0x4080020000000);
    test_pieces!(complex3_b_rook, complex_pos_3, B_ROOK, 0x8000000000);
    test_pieces!(complex3_b_knight, complex_pos_3, B_KNIGHT, 0);
    test_pieces!(complex3_b_bishop, complex_pos_3, B_BISHOP, 0);
    test_pieces!(complex3_b_queen, complex_pos_3, B_QUEEN, 0);
    test_pieces!(complex3_b_king, complex_pos_3, B_KING, 0x80000000);
    test_pieces!(complex3_b_pieces, complex_pos_3, B_PIECES, 0x40880A0000000);

    const complex_pos_4: &str = "r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1";

    test_pieces!(complex4_w_pawn, complex_pos_4, W_PAWN, 0x100021400C900);
    test_pieces!(complex4_w_rook, complex_pos_4, W_ROOK, 0x21);
    test_pieces!(complex4_w_knight, complex_pos_4, W_KNIGHT, 0x800000200000);
    test_pieces!(complex4_w_bishop, complex_pos_4, W_BISHOP, 0x3000000);
    test_pieces!(complex4_w_queen, complex_pos_4, W_QUEEN, 0x8);
    test_pieces!(complex4_w_king, complex_pos_4, W_KING, 0x40);
    test_pieces!(complex4_w_pieces, complex_pos_4, W_PIECES, 0x180021720C969);
    test_pieces!(complex4_b_pawn, complex_pos_4, B_PAWN, 0xEE000000000200);
    test_pieces!(complex4_b_rook, complex_pos_4, B_ROOK, 0x8100000000000000);
    test_pieces!(complex4_b_knight, complex_pos_4, B_KNIGHT, 0x200100000000);
    test_pieces!(complex4_b_bishop, complex_pos_4, B_BISHOP, 0x420000000000);
    test_pieces!(complex4_b_queen, complex_pos_4, B_QUEEN, 0x10000);
    test_pieces!(complex4_b_king, complex_pos_4, B_KING, 0x1000000000000000);
    test_pieces!(
        complex4_b_pieces,
        complex_pos_4,
        B_PIECES,
        0x91EE620100010200
    );

    const complex_pos_5: &str = "rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8";

    test_pieces!(complex5_w_pawn, complex_pos_5, W_PAWN, 0x800000000C700);
    test_pieces!(complex5_w_rook, complex_pos_5, W_ROOK, 0x81);
    test_pieces!(complex5_w_knight, complex_pos_5, W_KNIGHT, 0x1002);
    test_pieces!(complex5_w_bishop, complex_pos_5, W_BISHOP, 0x4000004);
    test_pieces!(complex5_w_queen, complex_pos_5, W_QUEEN, 0x8);
    test_pieces!(complex5_w_king, complex_pos_5, W_KING, 0x10);
    test_pieces!(complex5_w_pieces, complex_pos_5, W_PIECES, 0x800000400D79F);
    test_pieces!(complex5_b_pawn, complex_pos_5, B_PAWN, 0xE3040000000000);
    test_pieces!(complex5_b_rook, complex_pos_5, B_ROOK, 0x8100000000000000);
    test_pieces!(
        complex5_b_knight,
        complex_pos_5,
        B_KNIGHT,
        0x200000000002000
    );
    test_pieces!(
        complex5_b_bishop,
        complex_pos_5,
        B_BISHOP,
        0x410000000000000
    );
    test_pieces!(complex5_b_queen, complex_pos_5, B_QUEEN, 0x800000000000000);
    test_pieces!(complex5_b_king, complex_pos_5, B_KING, 0x2000000000000000);
    test_pieces!(
        complex5_b_pieces,
        complex_pos_5,
        B_PIECES,
        0xAFF3040000002000
    );

    const complex_pos_6: &str =
        "r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10";

    test_pieces!(complex6_w_pawn, complex_pos_6, W_PAWN, 0x1009E600);
    test_pieces!(complex6_w_rook, complex_pos_6, W_ROOK, 0x21);
    test_pieces!(complex6_w_knight, complex_pos_6, W_KNIGHT, 0x240000);
    test_pieces!(complex6_w_bishop, complex_pos_6, W_BISHOP, 0x4004000000);
    test_pieces!(complex6_w_queen, complex_pos_6, W_QUEEN, 0x1000);
    test_pieces!(complex6_w_king, complex_pos_6, W_KING, 0x40);
    test_pieces!(complex6_w_pieces, complex_pos_6, W_PIECES, 0x40142DF661);
    test_pieces!(complex6_b_pawn, complex_pos_6, B_PAWN, 0xE6091000000000);
    test_pieces!(complex6_b_rook, complex_pos_6, B_ROOK, 0x2100000000000000);
    test_pieces!(complex6_b_knight, complex_pos_6, B_KNIGHT, 0x240000000000);
    test_pieces!(complex6_b_bishop, complex_pos_6, B_BISHOP, 0x440000000);
    test_pieces!(complex6_b_queen, complex_pos_6, B_QUEEN, 0x10000000000000);
    test_pieces!(complex6_b_king, complex_pos_6, B_KING, 0x4000000000000000);
    test_pieces!(
        complex6_b_pieces,
        complex_pos_6,
        B_PIECES,
        0x61F62D1440000000
    );

    // Test castling availability of Position construction
    macro_rules! test_castle {
        ($test_name:ident, $castle_rights:expr, $position_member:ident, $expected:literal) => {
            #[test]
            fn $test_name() {
                let fen = concat!("8/8/8/8/8/8/8/8 w ", $castle_rights, " - 0 1");
                assert_eq!(Position::from(&fen).$position_member, $expected);
            }
        };
    }

    // Since castling availability is finite and small, test all possible combniations
    test_castle!(castling_none_w_king, "-", w_king_castle, false);
    test_castle!(caslting_none_w_queen, "-", w_queen_castle, false);
    test_castle!(caslting_none_b_king, "-", b_king_castle, false);
    test_castle!(caslting_none_b_queen, "-", b_queen_castle, false);

    test_castle!(castling_K_w_king, "K", w_king_castle, true);
    test_castle!(caslting_K_w_queen, "K", w_queen_castle, false);
    test_castle!(caslting_K_b_king, "K", b_king_castle, false);
    test_castle!(caslting_K_b_queen, "K", b_queen_castle, false);

    test_castle!(castling_k_w_king, "k", w_king_castle, false);
    test_castle!(castling_k_w_queen, "k", w_queen_castle, false);
    test_castle!(castling_k_b_king, "k", b_king_castle, true);
    test_castle!(castling_k_b_queen, "k", b_queen_castle, false);

    test_castle!(castling_Q_w_king, "Q", w_king_castle, false);
    test_castle!(castling_Q_w_queen, "Q", w_queen_castle, true);
    test_castle!(castling_Q_b_king, "Q", b_king_castle, false);
    test_castle!(castling_Q_b_queen, "Q", b_queen_castle, false);

    test_castle!(castling_q_w_king, "q", w_king_castle, false);
    test_castle!(castling_q_w_queen, "q", w_queen_castle, false);
    test_castle!(castling_q_b_king, "q", b_king_castle, false);
    test_castle!(castling_q_b_queen, "q", b_queen_castle, true);

    test_castle!(castling_KQ_w_king, "KQ", w_king_castle, true);
    test_castle!(caslting_KQ_w_queen, "KQ", w_queen_castle, true);
    test_castle!(caslting_KQ_b_king, "KQ", b_king_castle, false);
    test_castle!(caslting_KQ_b_queen, "KQ", b_queen_castle, false);

    test_castle!(castling_Kk_w_king, "Kk", w_king_castle, true);
    test_castle!(castling_Kk_w_queen, "Kk", w_queen_castle, false);
    test_castle!(castling_Kk_b_king, "Kk", b_king_castle, true);
    test_castle!(castling_Kk_b_queen, "Kk", b_queen_castle, false);

    test_castle!(castling_Kq_w_king, "Kq", w_king_castle, true);
    test_castle!(castling_Kq_w_queen, "Kq", w_queen_castle, false);
    test_castle!(castling_Kq_b_king, "Kq", b_king_castle, false);
    test_castle!(castling_Kq_b_queen, "Kq", b_queen_castle, true);

    test_castle!(castling_Qk_w_king, "Qk", w_king_castle, false);
    test_castle!(castling_Qk_w_queen, "Qk", w_queen_castle, true);
    test_castle!(castling_Qk_b_king, "Qk", b_king_castle, true);
    test_castle!(castling_Qk_b_queen, "Qk", b_queen_castle, false);

    test_castle!(castling_Qq_w_king, "Qq", w_king_castle, false);
    test_castle!(castling_Qq_w_queen, "Qq", w_queen_castle, true);
    test_castle!(castling_Qq_b_king, "Qq", b_king_castle, false);
    test_castle!(castling_Qq_b_queen, "Qq", b_queen_castle, true);

    test_castle!(castling_qk_w_king, "qk", w_king_castle, false);
    test_castle!(castling_qk_w_queen, "qk", w_queen_castle, false);
    test_castle!(castling_qk_b_king, "qk", b_king_castle, true);
    test_castle!(castling_qk_b_queen, "qk", b_queen_castle, true);

    test_castle!(castling_KQk_w_king, "KQk", w_king_castle, true);
    test_castle!(castling_KQk_w_queen, "KQk", w_queen_castle, true);
    test_castle!(castling_KQk_b_king, "KQk", b_king_castle, true);
    test_castle!(castling_KQk_b_queen, "KQk", b_queen_castle, false);

    test_castle!(castling_KQq_w_king, "KQq", w_king_castle, true);
    test_castle!(castling_KQq_w_queen, "KQq", w_queen_castle, true);
    test_castle!(castling_KQq_b_king, "KQq", b_king_castle, false);
    test_castle!(castling_KQq_b_queen, "KQq", b_queen_castle, true);

    test_castle!(castling_Kkq_w_king, "Kkq", w_king_castle, true);
    test_castle!(castling_Kkq_w_queen, "Kkq", w_queen_castle, false);
    test_castle!(castling_Kkq_b_king, "Kkq", b_king_castle, true);
    test_castle!(castling_Kkq_b_queen, "Kkq", b_queen_castle, true);

    test_castle!(castling_Qkq_w_king, "Qkq", w_king_castle, false);
    test_castle!(castling_Qkq_w_queen, "Qkq", w_queen_castle, true);
    test_castle!(castling_Qkq_b_king, "Qkq", b_king_castle, true);
    test_castle!(castling_Qkq_b_queen, "Qkq", b_queen_castle, true);

    test_castle!(castling_KQkq_w_king, "KQkq", w_king_castle, true);
    test_castle!(castling_KQkq_w_queen, "KQkq", w_queen_castle, true);
    test_castle!(castling_KQkq_b_king, "KQkq", b_king_castle, true);
    test_castle!(castling_KQkq_b_queen, "KQkq", b_queen_castle, true);

    // Test active color of Position construction
    #[test]
    fn active_color_w() {
        let fen = "8/8/8/8/8/8/8/8 w - - 0 1";
        assert_eq!(Position::from(fen).is_white_move, true);
    }

    #[test]
    fn active_color_b() {
        let fen = "8/8/8/8/8/8/8/8 b - - 0 1";
        assert_eq!(Position::from(fen).is_white_move, false);
    }

    // Test en passant square of Position construction
    macro_rules! test_passant {
        ($test_name:ident, $passant_sq:expr, $expected:expr) => {
            #[test]
            fn $test_name() {
                let fen = concat!("8/8/8/8/8/8/8/8 w - ", $passant_sq, " 0 1");
                assert_eq!(Position::from(&fen).passant_sq, $expected);
            }
        };
    }

    // Since passant square is finite and small, test all possible combniations
    test_passant!(passant_none, "-", 0);

    test_passant!(passant_a1, "a1", A_FILE & RANK_1);
    test_passant!(passant_b1, "b1", B_FILE & RANK_1);
    test_passant!(passant_c1, "c1", C_FILE & RANK_1);
    test_passant!(passant_d1, "d1", D_FILE & RANK_1);
    test_passant!(passant_e1, "e1", E_FILE & RANK_1);
    test_passant!(passant_f1, "f1", F_FILE & RANK_1);
    test_passant!(passant_g1, "g1", G_FILE & RANK_1);
    test_passant!(passant_h1, "h1", H_FILE & RANK_1);

    test_passant!(passant_a2, "a2", A_FILE & RANK_2);
    test_passant!(passant_b2, "b2", B_FILE & RANK_2);
    test_passant!(passant_c2, "c2", C_FILE & RANK_2);
    test_passant!(passant_d2, "d2", D_FILE & RANK_2);
    test_passant!(passant_e2, "e2", E_FILE & RANK_2);
    test_passant!(passant_f2, "f2", F_FILE & RANK_2);
    test_passant!(passant_g2, "g2", G_FILE & RANK_2);
    test_passant!(passant_h2, "h2", H_FILE & RANK_2);

    test_passant!(passant_a3, "a3", A_FILE & RANK_3);
    test_passant!(passant_b3, "b3", B_FILE & RANK_3);
    test_passant!(passant_c3, "c3", C_FILE & RANK_3);
    test_passant!(passant_d3, "d3", D_FILE & RANK_3);
    test_passant!(passant_e3, "e3", E_FILE & RANK_3);
    test_passant!(passant_f3, "f3", F_FILE & RANK_3);
    test_passant!(passant_g3, "g3", G_FILE & RANK_3);
    test_passant!(passant_h3, "h3", H_FILE & RANK_3);

    test_passant!(passant_a4, "a4", A_FILE & RANK_4);
    test_passant!(passant_b4, "b4", B_FILE & RANK_4);
    test_passant!(passant_c4, "c4", C_FILE & RANK_4);
    test_passant!(passant_d4, "d4", D_FILE & RANK_4);
    test_passant!(passant_e4, "e4", E_FILE & RANK_4);
    test_passant!(passant_f4, "f4", F_FILE & RANK_4);
    test_passant!(passant_g4, "g4", G_FILE & RANK_4);
    test_passant!(passant_h4, "h4", H_FILE & RANK_4);

    test_passant!(passant_a5, "a5", A_FILE & RANK_5);
    test_passant!(passant_b5, "b5", B_FILE & RANK_5);
    test_passant!(passant_c5, "c5", C_FILE & RANK_5);
    test_passant!(passant_d5, "d5", D_FILE & RANK_5);
    test_passant!(passant_e5, "e5", E_FILE & RANK_5);
    test_passant!(passant_f5, "f5", F_FILE & RANK_5);
    test_passant!(passant_g5, "g5", G_FILE & RANK_5);
    test_passant!(passant_h5, "h5", H_FILE & RANK_5);

    test_passant!(passant_a6, "a6", A_FILE & RANK_6);
    test_passant!(passant_b6, "b6", B_FILE & RANK_6);
    test_passant!(passant_c6, "c6", C_FILE & RANK_6);
    test_passant!(passant_d6, "d6", D_FILE & RANK_6);
    test_passant!(passant_e6, "e6", E_FILE & RANK_6);
    test_passant!(passant_f6, "f6", F_FILE & RANK_6);
    test_passant!(passant_g6, "g6", G_FILE & RANK_6);
    test_passant!(passant_h6, "h6", H_FILE & RANK_6);

    test_passant!(passant_a7, "a7", A_FILE & RANK_7);
    test_passant!(passant_b7, "b7", B_FILE & RANK_7);
    test_passant!(passant_c7, "c7", C_FILE & RANK_7);
    test_passant!(passant_d7, "d7", D_FILE & RANK_7);
    test_passant!(passant_e7, "e7", E_FILE & RANK_7);
    test_passant!(passant_f7, "f7", F_FILE & RANK_7);
    test_passant!(passant_g7, "g7", G_FILE & RANK_7);
    test_passant!(passant_h7, "h7", H_FILE & RANK_7);

    test_passant!(passant_a8, "a8", A_FILE & RANK_8);
    test_passant!(passant_b8, "b8", B_FILE & RANK_8);
    test_passant!(passant_c8, "c8", C_FILE & RANK_8);
    test_passant!(passant_d8, "d8", D_FILE & RANK_8);
    test_passant!(passant_e8, "e8", E_FILE & RANK_8);
    test_passant!(passant_f8, "f8", F_FILE & RANK_8);
    test_passant!(passant_g8, "g8", G_FILE & RANK_8);
    test_passant!(passant_h8, "h8", H_FILE & RANK_8);

    // Test halfmove clock of Position construction
    macro_rules! test_half_clock {
        ($test_name:ident, $hlf_clock:expr, $expected:expr) => {
            #[test]
            fn $test_name() {
                let fen = concat!("8/8/8/8/8/8/8/8 w - - ", $hlf_clock, " 1");
                assert_eq!(Position::from(&fen).hlf_clock, $expected);
            }
        };
    }

    test_half_clock!(half_clock_1, "1", 1);
    test_half_clock!(half_clock_2, "100", 100);
    test_half_clock!(half_clock_3, "255", 255);
    test_half_clock!(half_clock_4, "0", 0);
    test_half_clock!(half_clock_5, "2", 2);
    test_half_clock!(half_clock_6, "4", 4);
    test_half_clock!(half_clock_7, "8", 8);
    test_half_clock!(half_clock_8, "16", 16);

    // Test fullmove number of Position construction
    macro_rules! test_full_number {
        ($test_name:ident, $full_num:expr, $expected:expr) => {
            #[test]
            fn $test_name() {
                let fen = concat!("8/8/8/8/8/8/8/8 w - - 0 ", $full_num);
                assert_eq!(Position::from(&fen).full_num, $expected);
            }
        };
    }

    test_full_number!(full_number_1, "2", 2);
    test_full_number!(full_number_2, "101", 101);
    test_full_number!(full_number_3, "254", 254);
    test_full_number!(full_number_4, "0", 0);
    test_full_number!(full_number_5, "3", 3);
    test_full_number!(full_number_6, "5", 5);
    test_full_number!(full_number_7, "9", 9);
    test_full_number!(full_number_8, "17", 17);

    // Test sq_to_bitboard
    macro_rules! test_sq_to_bb {
        ($test_name:ident, $file:expr, $rank:expr, $expected:expr) => {
            #[test]
            fn $test_name() {
                assert_eq!(sq_to_bitboard($file, $rank), $expected);
            }
        };
    }

    test_sq_to_bb!(sq_to_bitboard_a1, 'a', '1', A_FILE & RANK_1);
    test_sq_to_bb!(sq_to_bitboard_b2, 'b', '2', B_FILE & RANK_2);
    test_sq_to_bb!(sq_to_bitboard_c3, 'c', '3', C_FILE & RANK_3);
    test_sq_to_bb!(sq_to_bitboard_d4, 'd', '4', D_FILE & RANK_4);
    test_sq_to_bb!(sq_to_bitboard_e5, 'e', '5', E_FILE & RANK_5);
    test_sq_to_bb!(sq_to_bitboard_f6, 'f', '6', F_FILE & RANK_6);
    test_sq_to_bb!(sq_to_bitboard_g7, 'g', '7', G_FILE & RANK_7);
    test_sq_to_bb!(sq_to_bitboard_h8, 'h', '8', H_FILE & RANK_8);

    // Position::new test
    #[test]
    fn new_returns_startpos() {
        let start_position = Position::new();
        let expected = Position {
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
            passant_sq: 0, // En Passant square

            w_king_castle: true,
            w_queen_castle: true,
            b_king_castle: true,
            b_queen_castle: true,

            is_white_move: true,
            hlf_clock: 0,
            full_num: 1,
        };
        assert_eq!(start_position, expected);
    }

    // Evaluation testing
    #[test]
    fn evaluate_startpos() {
        let pos = Position::new();
        assert_eq!(pos.evaluate(), 0);
    }

    #[test]
    fn evaluate_no_w_king() {
        let pos = Position::from("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQ1BNR w KQkq - 0 1");
        assert_eq!(pos.evaluate(), isize::MIN);
    }

    #[test]
    fn evaluate_no_b_king() {
        let pos = Position::from("rnbq1bnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
        assert_eq!(pos.evaluate(), isize::MAX);
    }

    // Position::play_move() testing
    macro_rules! test_play_move {
        ($test_name:ident, $starting_position:expr, $move:expr, $expected:expr) => {
            #[test]
            fn $test_name() {
                let mut starting_position = Position::from($starting_position);
                let expected_position = Position::from($expected);
                starting_position.play_move($move);
                assert_eq!(starting_position, expected_position);
            }
        };
    }

    // Basic movement tests
    const startpos_b: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR b KQkq - 0 1";

    test_play_move!(
        play_startpos_a2a3,
        startpos,
        "a2a3",
        "rnbqkbnr/pppppppp/8/8/8/P7/1PPPPPPP/RNBQKBNR b KQkq - 0 1"
    ); // startpos w_pawn advance 1
    test_play_move!(
        play_startpos_a2a4,
        startpos,
        "a2a4",
        "rnbqkbnr/pppppppp/8/8/P7/8/1PPPPPPP/RNBQKBNR b KQkq a3 0 1"
    ); // startpos w_pawn advance 2
    test_play_move!(
        play_startpos_a7a6,
        startpos_b,
        "a7a6",
        "rnbqkbnr/1ppppppp/p7/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 2"
    ); // startpos b_pawn advance 1
    test_play_move!(
        play_startpos_a7a5,
        startpos_b,
        "a7a5",
        "rnbqkbnr/1ppppppp/8/p7/8/8/PPPPPPPP/RNBQKBNR w KQkq a6 0 2"
    ); // startpos b_pawn advance 2
    test_play_move!(
        play_random_w_pawn_forward_1,
        "nKn5/8/Q2P1bpp/NP2P3/bR6/4pP2/7k/8 w - - 0 1",
        "d6d7",
        "nKn5/3P4/Q4bpp/NP2P3/bR6/4pP2/7k/8 b - - 0 1"
    ); // Random position w_pawn advance 1
    test_play_move!(
        play_random_b_pawn_forward_1,
        "7Q/4k1pb/1P1p4/4r3/pP1Rp1K1/5pb1/4P2p/8 b - - 0 1",
        "d6d5",
        "7Q/4k1pb/1P6/3pr3/pP1Rp1K1/5pb1/4P2p/8 w - - 0 2"
    ); // Random position b_pawn advance 1
    test_play_move!(
        play_random_w_pawn_forward_2,
        "4Q3/P6p/7p/pPk4P/p7/2P3Pp/q4P2/3b2K1 w - - 0 1",
        "f2f4",
        "4Q3/P6p/7p/pPk4P/p4P2/2P3Pp/q7/3b2K1 b - f3 0 1"
    ); // Random position w_pawn advance 2
    test_play_move!(
        play_random_b_pawn_forward_2,
        "3b4/2B1p3/2p5/Pq1N3p/p2N2k1/p3K2p/5P2/2R5 b - - 0 1",
        "e7e5",
        "3b4/2B5/2p5/Pq1Np2p/p2N2k1/p3K2p/5P2/2R5 w - e6 0 2"
    ); // Random position b_pawn advance 2
    test_play_move!(
        play_startpos_b1c3,
        startpos,
        "b1c3",
        "rnbqkbnr/pppppppp/8/8/8/2N5/PPPPPPPP/R1BQKBNR b KQkq - 1 1"
    ); // w_knight initial move
    test_play_move!(
        play_startpos_b8c6,
        startpos_b,
        "b8c6",
        "r1bqkbnr/pppppppp/2n5/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 1 2"
    ); // b_knight initial move
    test_play_move!(
        play_w_bishop_move,
        "rnbqkbnr/pppppppp/8/8/8/5B2/PPPPPPPP/RNBQK1NR w KQkq - 0 1",
        "f3d5",
        "rnbqkbnr/pppppppp/8/3B4/8/8/PPPPPPPP/RNBQK1NR b KQkq - 1 1"
    ); // w_bishop move
    test_play_move!(
        play_b_bishop_move,
        "rnbqk1nr/pppppppp/5b2/8/8/8/PPPPPPPP/RNBQKBNR b KQkq - 0 1",
        "f6d4",
        "rnbqk1nr/pppppppp/8/8/3b4/8/PPPPPPPP/RNBQKBNR w KQkq - 1 2"
    ); // b_bishop move
    test_play_move!(
        play_w_rook_move,
        "rnbqkbnr/pppppppp/8/8/7R/8/PPPPPPPP/RNBQKBN1 w KQkq - 0 1",
        "h4a4",
        "rnbqkbnr/pppppppp/8/8/R7/8/PPPPPPPP/RNBQKBN1 b KQkq - 1 1"
    ); // w_rook move
    test_play_move!(
        play_b_rook_move,
        "rnbqkbn1/pppppppp/8/7r/8/8/PPPPPPPP/RNBQKBNR b KQkq - 0 1",
        "h5a5",
        "rnbqkbn1/pppppppp/8/r7/8/8/PPPPPPPP/RNBQKBNR w KQkq - 1 2"
    ); // b_rook move
    test_play_move!(
        play_w_queen_move,
        "rnbqkbnr/pppppppp/8/8/8/3Q4/PPPPPPPP/RNB1KBNR w KQkq - 0 1",
        "d3h3",
        "rnbqkbnr/pppppppp/8/8/8/7Q/PPPPPPPP/RNB1KBNR b KQkq - 1 1"
    ); // w_queen move
    test_play_move!(
        play_b_queen_move,
        "rnb1kbnr/pppppppp/3q4/8/8/8/PPPPPPPP/RNBQKBNR b KQkq - 0 1",
        "d6h6",
        "rnb1kbnr/pppppppp/7q/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 1 2"
    ); // b_queen move
    test_play_move!(
        play_w_king_move,
        "rnbqkbnr/pppppppp/8/8/4K3/8/PPPPPPPP/RNBQ1BNR w KQkq - 0 1",
        "e4f5",
        "rnbqkbnr/pppppppp/8/5K2/8/8/PPPPPPPP/RNBQ1BNR b KQkq - 1 1"
    ); // w_king move
    test_play_move!(
        play_b_king_move,
        "rnbq1bnr/pppppppp/8/4k3/8/8/PPPPPPPP/RNBQKBNR b KQkq - 0 1",
        "e5f4",
        "rnbq1bnr/pppppppp/8/8/5k2/8/PPPPPPPP/RNBQKBNR w KQkq - 1 2"
    ); // b_king move

    // Capture tests
    test_play_move!(
        play_basic_w_pawn_capture,
        "rnbqkbnr/pppppppp/4P3/8/8/8/PPPP1PPP/RNBQKBNR w KQkq - 0 1",
        "e6f7",
        "rnbqkbnr/pppppPpp/8/8/8/8/PPPP1PPP/RNBQKBNR b KQkq - 0 1"
    );
    test_play_move!(
        play_basic_b_pawn_capture,
        "rnbqkbnr/pppp1ppp/8/8/8/4p3/PPPPPPPP/RNBQKBNR b KQkq - 0 1",
        "e3f2",
        "rnbqkbnr/pppp1ppp/8/8/8/8/PPPPPpPP/RNBQKBNR w KQkq - 0 2"
    );
    test_play_move!(
        play_w_pawn_capture_passant,
        "rnbqkbnr/ppppp1pp/8/4Pp2/8/8/PPPP1PPP/RNBQKBNR w KQkq f6 0 1",
        "e5f6",
        "rnbqkbnr/ppppp1pp/5P2/8/8/8/PPPP1PPP/RNBQKBNR b KQkq - 0 1"
    );
    test_play_move!(
        play_b_pawn_capture_passant,
        "rnbqkbnr/pppp1ppp/8/8/4pP2/8/PPPPP1PP/RNBQKBNR b KQkq f3 0 1",
        "e4f3",
        "rnbqkbnr/pppp1ppp/8/8/8/5p2/PPPPP1PP/RNBQKBNR w KQkq - 0 2"
    );
    test_play_move!(
        play_basic_w_knight_capture,
        "rnbqkb1r/pppppppp/5n2/8/4N3/8/PPPPPPPP/RNBQKB1R w KQkq - 0 1",
        "e4f6",
        "rnbqkb1r/pppppppp/5N2/8/8/8/PPPPPPPP/RNBQKB1R b KQkq - 0 1"
    );
    test_play_move!(
        play_basic_b_knight_capture,
        "rnbqkb1r/pppppppp/5n2/8/4N3/8/PPPPPPPP/RNBQKB1R b KQkq - 0 1",
        "f6e4",
        "rnbqkb1r/pppppppp/8/8/4n3/8/PPPPPPPP/RNBQKB1R w KQkq - 0 2"
    );
    test_play_move!(
        play_basic_w_bishop_capture,
        "rn1qkbnr/pppppppp/2b5/8/8/5B2/PPPPPPPP/RNBQK1NR w KQkq - 0 1",
        "f3c6",
        "rn1qkbnr/pppppppp/2B5/8/8/8/PPPPPPPP/RNBQK1NR b KQkq - 0 1"
    );
    test_play_move!(
        play_basic_b_bishop_capture,
        "rn1qkbnr/pppppppp/2b5/8/8/5B2/PPPPPPPP/RNBQK1NR b KQkq - 0 1",
        "c6f3",
        "rn1qkbnr/pppppppp/8/8/8/5b2/PPPPPPPP/RNBQK1NR w KQkq - 0 2"
    );
    test_play_move!(
        play_basic_w_rook_capture,
        "rnbqkbn1/pppppppp/7r/8/8/7R/PPPPPPPP/RNBQKBN1 w KQkq - 0 1",
        "h3h6",
        "rnbqkbn1/pppppppp/7R/8/8/8/PPPPPPPP/RNBQKBN1 b KQkq - 0 1"
    );
    test_play_move!(
        play_basic_b_rook_capture,
        "rnbqkbn1/pppppppp/7r/8/8/7R/PPPPPPPP/RNBQKBN1 b KQkq - 0 1",
        "h6h3",
        "rnbqkbn1/pppppppp/8/8/8/7r/PPPPPPPP/RNBQKBN1 w KQkq - 0 2"
    );
    test_play_move!(
        play_basic_w_queen_capture,
        "rnb1kbnr/pppppppp/5q2/8/8/2Q5/PPPPPPPP/RNB1KBNR w KQkq - 0 1",
        "c3f6",
        "rnb1kbnr/pppppppp/5Q2/8/8/8/PPPPPPPP/RNB1KBNR b KQkq - 0 1"
    );
    test_play_move!(
        play_basic_b_queen_capture,
        "rnb1kbnr/pppppppp/5q2/8/8/2Q5/PPPPPPPP/RNB1KBNR b KQkq - 0 1",
        "f6c3",
        "rnb1kbnr/pppppppp/8/8/8/2q5/PPPPPPPP/RNB1KBNR w KQkq - 0 2"
    );
    test_play_move!(
        play_basic_w_king_capture,
        "rnbq1bnr/pppppppp/8/4k3/4K3/8/PPPPPPPP/RNBQ1BNR w KQkq - 0 1",
        "e4e5",
        "rnbq1bnr/pppppppp/8/4K3/8/8/PPPPPPPP/RNBQ1BNR b KQkq - 0 1"
    );
    test_play_move!(
        play_basic_b_king_capture,
        "rnbq1bnr/pppppppp/8/4k3/4K3/8/PPPPPPPP/RNBQ1BNR b KQkq - 0 1",
        "e5e4",
        "rnbq1bnr/pppppppp/8/8/4k3/8/PPPPPPPP/RNBQ1BNR w KQkq - 0 2"
    );

    // Castling tests
    test_play_move!(
        play_castle_w_kingside,
        "r3k2r/pppppppp/8/8/8/8/PPPPPPPP/R3K2R w KQkq - 0 1",
        "e1g1",
        "r3k2r/pppppppp/8/8/8/8/PPPPPPPP/R4RK1 b kq - 1 1"
    );
    test_play_move!(
        play_castle_w_queenside,
        "r3k2r/pppppppp/8/8/8/8/PPPPPPPP/R3K2R w KQkq - 0 1",
        "e1c1",
        "r3k2r/pppppppp/8/8/8/8/PPPPPPPP/2KR3R b kq - 1 1"
    );
    test_play_move!(
        play_castle_b_kingside,
        "r3k2r/pppppppp/8/8/8/8/PPPPPPPP/R3K2R b KQkq - 0 1",
        "e8g8",
        "r4rk1/pppppppp/8/8/8/8/PPPPPPPP/R3K2R w KQ - 1 1"
    );
    test_play_move!(
        play_castle_b_queenside,
        "r3k2r/pppppppp/8/8/8/8/PPPPPPPP/R3K2R b KQkq - 0 1",
        "e8c8",
        "2kr3r/pppppppp/8/8/8/8/PPPPPPPP/R3K2R w KQ - 1 1"
    );
}
