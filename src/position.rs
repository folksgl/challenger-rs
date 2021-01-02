use std::cmp::Ordering;
use std::collections::HashMap;
use std::fmt;

lazy_static! {
    static ref BITBOARD_TO_SQUARE: HashMap<u64, &'static str> = {
        let mut m = HashMap::new();
        m.insert(0, "-");
        m.insert(square_bit(0), "a1");
        m.insert(square_bit(1), "b1");
        m.insert(square_bit(2), "c1");
        m.insert(square_bit(3), "d1");
        m.insert(square_bit(4), "e1");
        m.insert(square_bit(5), "f1");
        m.insert(square_bit(6), "g1");
        m.insert(square_bit(7), "h1");
        m.insert(square_bit(8), "a2");
        m.insert(square_bit(9), "b2");
        m.insert(square_bit(10), "c2");
        m.insert(square_bit(11), "d2");
        m.insert(square_bit(12), "e2");
        m.insert(square_bit(13), "f2");
        m.insert(square_bit(14), "g2");
        m.insert(square_bit(15), "h2");
        m.insert(square_bit(16), "a3");
        m.insert(square_bit(17), "b3");
        m.insert(square_bit(18), "c3");
        m.insert(square_bit(19), "d3");
        m.insert(square_bit(20), "e3");
        m.insert(square_bit(21), "f3");
        m.insert(square_bit(22), "g3");
        m.insert(square_bit(23), "h3");
        m.insert(square_bit(24), "a4");
        m.insert(square_bit(25), "b4");
        m.insert(square_bit(26), "c4");
        m.insert(square_bit(27), "d4");
        m.insert(square_bit(28), "e4");
        m.insert(square_bit(29), "f4");
        m.insert(square_bit(30), "g4");
        m.insert(square_bit(31), "h4");
        m.insert(square_bit(32), "a5");
        m.insert(square_bit(33), "b5");
        m.insert(square_bit(34), "c5");
        m.insert(square_bit(35), "d5");
        m.insert(square_bit(36), "e5");
        m.insert(square_bit(37), "f5");
        m.insert(square_bit(38), "g5");
        m.insert(square_bit(39), "h5");
        m.insert(square_bit(40), "a6");
        m.insert(square_bit(41), "b6");
        m.insert(square_bit(42), "c6");
        m.insert(square_bit(43), "d6");
        m.insert(square_bit(44), "e6");
        m.insert(square_bit(45), "f6");
        m.insert(square_bit(46), "g6");
        m.insert(square_bit(47), "h6");
        m.insert(square_bit(48), "a7");
        m.insert(square_bit(49), "b7");
        m.insert(square_bit(50), "c7");
        m.insert(square_bit(51), "d7");
        m.insert(square_bit(52), "e7");
        m.insert(square_bit(53), "f7");
        m.insert(square_bit(54), "g7");
        m.insert(square_bit(55), "h7");
        m.insert(square_bit(56), "a8");
        m.insert(square_bit(57), "b8");
        m.insert(square_bit(58), "c8");
        m.insert(square_bit(59), "d8");
        m.insert(square_bit(60), "e8");
        m.insert(square_bit(61), "f8");
        m.insert(square_bit(62), "g8");
        m.insert(square_bit(63), "h8");
        m
    };
}

const ONE: u64 = 0x01;
const TWO: u64 = 0x10;
const FOUR: u64 = TWO << ONE;

const fn square_bit(i: isize) -> u64 {
    return ONE << i;
}

// Pre-initialized lookup table for possible knight moves on a given square
const knight_moves: [u64; 64] = [
    0x0000000000020400,
    0x0000000000050800,
    0x00000000000A1100,
    0x0000000000142200,
    0x0000000000284400,
    0x0000000000508800,
    0x0000000000A01000,
    0x0000000000402000, // 1st Rank
    0x0000000002040004,
    0x0000000005080008,
    0x000000000A110011,
    0x0000000014220022,
    0x0000000028440044,
    0x0000000050880088,
    0x00000000A0100010,
    0x0000000040200020, // 2nd Rank
    0x0000000204000402,
    0x0000000508000805,
    0x0000000A1100110A,
    0x0000001422002214,
    0x0000002844004428,
    0x0000005088008850,
    0x000000A0100010A0,
    0x0000004020002040, // 3rd Rank
    0x0000020400040200,
    0x0000050800080500,
    0x00000A1100110A00,
    0x0000142200221400,
    0x0000284400442800,
    0x0000508800885000,
    0x0000A0100010A000,
    0x0000402000204000, // 4th Rank
    0x0002040004020000,
    0x0005080008050000,
    0x000A1100110A0000,
    0x0014220022140000,
    0x0028440044280000,
    0x0050880088500000,
    0x00A0100010A00000,
    0x0040200020400000, // 5th Rank
    0x0204000402000000,
    0x0508000805000000,
    0x0A1100110A000000,
    0x1422002214000000,
    0x2844004428000000,
    0x5088008850000000,
    0xA0100010A0000000,
    0x4020002040000000, // 6th Rank
    0x0400040200000000,
    0x0800080500000000,
    0x1100110A00000000,
    0x2200221400000000,
    0x4400442800000000,
    0x8800885000000000,
    0x100010A000000000,
    0x2000204000000000, // 7th Rank
    0x0004020000000000,
    0x0008050000000000,
    0x00110A0000000000,
    0x0022140000000000,
    0x0044280000000000,
    0x0088500000000000,
    0x0010A00000000000,
    0x0020400000000000, // 8th Rank
];

// Pre-initialized lookup table for possible king moves on a given square
const king_moves: [u64; 64] = [
    0x0000000000000302,
    0x0000000000000705,
    0x0000000000000E0A,
    0x0000000000001C14,
    0x0000000000003828,
    0x0000000000007050,
    0x000000000000E0A0,
    0x000000000000C040, // 1st Rank
    0x0000000000030203,
    0x0000000000070507,
    0x00000000000E0A0E,
    0x00000000001C141C,
    0x0000000000382838,
    0x0000000000705070,
    0x0000000000E0A0E0,
    0x0000000000C040C0, // 2nd Rank
    0x0000000003020300,
    0x0000000007050700,
    0x000000000E0A0E00,
    0x000000001C141C00,
    0x0000000038283800,
    0x0000000070507000,
    0x00000000E0A0E000,
    0x00000000C040C000, // 3rd Rank
    0x0000000302030000,
    0x0000000705070000,
    0x0000000E0A0E0000,
    0x0000001C141C0000,
    0x0000003828380000,
    0x0000007050700000,
    0x000000E0A0E00000,
    0x000000C040C00000, // 4th Rank
    0x0000030203000000,
    0x0000070507000000,
    0x00000E0A0E000000,
    0x00001C141C000000,
    0x0000382838000000,
    0x0000705070000000,
    0x0000E0A0E0000000,
    0x0000C040C0000000, // 5th Rank
    0x0003020300000000,
    0x0007050700000000,
    0x000E0A0E00000000,
    0x001C141C00000000,
    0x0038283800000000,
    0x0070507000000000,
    0x00E0A0E000000000,
    0x00C040C000000000, // 6th Rank
    0x0302030000000000,
    0x0705070000000000,
    0x0E0A0E0000000000,
    0x1C141C0000000000,
    0x3828380000000000,
    0x7050700000000000,
    0xE0A0E00000000000,
    0xC040C00000000000, // 7th Rank
    0x0203000000000000,
    0x0507000000000000,
    0x0A0E000000000000,
    0x141C000000000000,
    0x2838000000000000,
    0x5070000000000000,
    0xA0E0000000000000,
    0x40C0000000000000, // 8th Rank
];

#[derive(Debug)]
pub struct Position {
    bitboards: [u64; 14],
    passant_sq: u64,
    moves: Vec<Position>,
    w_kingside_castle: bool,
    w_queenside_castle: bool,
    b_kingside_castle: bool,
    b_queenside_castle: bool,
    eval_score: usize,
    is_white_move: bool,
    hlf_clock: isize,
    full_num: isize,
}

impl Position {
    // Associated constants for accessing the 'pieces'
    const WPAWN: usize = 0;
    const WROOK: usize = 1;
    const WKNIGHT: usize = 2;
    const WBISHOP: usize = 3;
    const WQUEEN: usize = 4;
    const WKING: usize = 5;
    const WPIECES: usize = 6;
    const BPAWN: usize = 7;
    const BROOK: usize = 8;
    const BKNIGHT: usize = 9;
    const BBISHOP: usize = 10;
    const BQUEEN: usize = 11;
    const BKING: usize = 12;
    const BPIECES: usize = 13;

    const RANK1: u64 = 0x00000000000000FF;
    const RANK2: u64 = 0x000000000000FF00;
    const RANK3: u64 = 0x0000000000FF0000;
    const RANK4: u64 = 0x00000000FF000000;
    const RANK5: u64 = 0x000000FF00000000;
    const RANK6: u64 = 0x0000FF0000000000;
    const RANK7: u64 = 0x00FF000000000000;
    const RANK8: u64 = 0xFF00000000000000;

    // Returns a position containing the starting chess position.
    // Fen string: rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1
    pub fn new() -> Position {
        Position {
            bitboards: [
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
            moves: vec![],
            w_kingside_castle: true,
            w_queenside_castle: true,
            b_kingside_castle: true,
            b_queenside_castle: true,
            eval_score: 0,
            is_white_move: true,
            hlf_clock: 0,
            full_num: 1,
        }
    }

    pub fn from(fen_string: &str) -> Position {
        let mut iter = fen_string.split_whitespace();

        let piece_positions = iter.next().unwrap();
        let is_white_move: bool = iter.next().unwrap() == "w";
        let castle_rights = iter.next().unwrap();

        let passant_sq_str = iter.next().unwrap();
        let mut passant_sq = 0;
        if passant_sq_str != "-" {
            let mut char_iter = passant_sq_str.chars();
            let file = char_iter.next().unwrap();
            let rank = char_iter.next().unwrap();
            passant_sq = square_bit(get_square_num(file, rank));
        }

        let hlf_clock = iter.next().unwrap().parse().unwrap();
        let full_num = iter.next().unwrap().parse().unwrap();

        let mut pieces_arr = [0; 14];
        let fen: String = piece_positions.split('/').rev().collect();
        let mut square_num: isize = 0;

        for piece in fen.chars() {
            let current_piece = match piece {
                'P' => Position::WPAWN,
                'p' => Position::BPAWN,
                'R' => Position::WROOK,
                'r' => Position::BROOK,
                'N' => Position::WKNIGHT,
                'n' => Position::BKNIGHT,
                'B' => Position::WBISHOP,
                'b' => Position::BBISHOP,
                'Q' => Position::WQUEEN,
                'q' => Position::BQUEEN,
                'K' => Position::WKING,
                'k' => Position::BKING,
                '1'..='8' => {
                    let num = piece.to_digit(10).unwrap();
                    square_num += num as isize;
                    continue;
                }
                _ => panic!("failed to build position"),
            };
            pieces_arr[current_piece] |= square_bit(square_num);
            square_num += 1;
        }

        pieces_arr[Position::WPIECES] = pieces_arr[Position::WPAWN]
            | pieces_arr[Position::WROOK]
            | pieces_arr[Position::WKNIGHT]
            | pieces_arr[Position::WBISHOP]
            | pieces_arr[Position::WQUEEN]
            | pieces_arr[Position::WKING];
        pieces_arr[Position::BPIECES] = pieces_arr[Position::BPAWN]
            | pieces_arr[Position::BROOK]
            | pieces_arr[Position::BKNIGHT]
            | pieces_arr[Position::BBISHOP]
            | pieces_arr[Position::BQUEEN]
            | pieces_arr[Position::BKING];

        Position {
            bitboards: pieces_arr,
            passant_sq: passant_sq,
            moves: vec![],
            w_kingside_castle: castle_rights.contains("K"),
            w_queenside_castle: castle_rights.contains("Q"),
            b_kingside_castle: castle_rights.contains("k"),
            b_queenside_castle: castle_rights.contains("q"),
            eval_score: 0,
            is_white_move: is_white_move,
            hlf_clock: hlf_clock,
            full_num: full_num,
        }
    }

    pub fn find_move_from(other: &Self) -> String {
        String::from("a2a4")
    }

    pub fn perform_move(&mut self, move_str: &str) {
        let mut move_chars = move_str.chars();
        let start_square = get_square_num(move_chars.next().unwrap(), move_chars.next().unwrap());
        let dest_square = get_square_num(move_chars.next().unwrap(), move_chars.next().unwrap());
        let promotion = move_chars.next();

        // Check if there is a piece on the dest square and remove if needed.
        let start_square_bit = square_bit(start_square);
        let dest_square_bit = square_bit(dest_square);
        let moving_bits = start_square_bit | dest_square_bit;

        // If a capture is taking place. Zero the destination square and reset halfmove clock.
        if (self.whole_board() & dest_square_bit) != 0 {
            let dest_zero_mask = !dest_square_bit;
            for bitboard in self.bitboards.iter_mut() {
                *bitboard &= dest_zero_mask;
            }
            self.hlf_clock = -1;
        }

        let mut new_passant_sq = 0; // Default is 0, only set on double forward pawn move
        let moving_piece = self
            .bitboards
            .iter()
            .position(|&x| x & square_bit(start_square) != 0)
            .unwrap();

        match moving_piece {
            Position::WPAWN | Position::BPAWN => {
                // Check for en passant capture
                if (dest_square_bit & self.passant_sq) != 0 {
                    let dest_zero = if moving_piece == Position::WPAWN {
                        dest_square - 8
                    } else {
                        dest_square + 8
                    };

                    let mask = !square_bit(dest_zero);

                    self.bitboards[Position::WPIECES] &= mask;
                    self.bitboards[Position::BPIECES] &= mask;
                    self.bitboards[Position::WPAWN] &= mask;
                    self.bitboards[Position::BPAWN] &= mask;
                } else if (dest_square - start_square).abs() == 16 {
                    // Pawn double forward
                    new_passant_sq = square_bit((start_square + dest_square) / 2);
                } else if (dest_square_bit & (Position::RANK1 | Position::RANK8)) != 0 {
                    // Pawn promotion
                    self.bitboards[moving_piece] |= dest_square_bit;
                    match promotion.unwrap() {
                        'Q' => self.bitboards[Position::WQUEEN] |= dest_square_bit,
                        'q' => self.bitboards[Position::BQUEEN] |= dest_square_bit,
                        'R' => self.bitboards[Position::WROOK] |= dest_square_bit,
                        'r' => self.bitboards[Position::BROOK] |= dest_square_bit,
                        'N' => self.bitboards[Position::WKNIGHT] |= dest_square_bit,
                        'n' => self.bitboards[Position::BKNIGHT] |= dest_square_bit,
                        'B' => self.bitboards[Position::WBISHOP] |= dest_square_bit,
                        'b' => self.bitboards[Position::BBISHOP] |= dest_square_bit,
                        _ => panic!("Invalid promotion character"),
                    }
                }
                self.hlf_clock = -1; // Pawn moves reset the halfmove clock.
            }
            Position::WKING => {
                self.w_kingside_castle = false;
                self.w_queenside_castle = false;
                self.hlf_clock = -1;
                if (start_square - dest_square) == 2 {
                    // Queenside Castling
                    self.bitboards[Position::WROOK] ^= 0x0000000000000009;
                    self.bitboards[Position::WPIECES] ^= 0x0000000000000009;
                } else if (start_square - dest_square) == -2 {
                    // Kingside Castling
                    self.bitboards[Position::WROOK] ^= 0x00000000000000A0;
                    self.bitboards[Position::WPIECES] ^= 0x00000000000000A0;
                }
            }
            Position::BKING => {
                self.b_kingside_castle = false;
                self.b_queenside_castle = false;
                self.hlf_clock = -1;
                if (start_square - dest_square) == 2 {
                    // Queenside Castling
                    self.bitboards[Position::BROOK] ^= 0x0900000000000000;
                    self.bitboards[Position::BPIECES] ^= 0x0900000000000000;
                } else if (start_square - dest_square) == -2 {
                    // Kingside Castling
                    self.bitboards[Position::BROOK] ^= 0xA000000000000000;
                    self.bitboards[Position::BPIECES] ^= 0xA000000000000000;
                }
            }
            Position::WROOK | Position::BROOK => {
                match start_square {
                    0 => self.w_queenside_castle = false,
                    7 => self.w_kingside_castle = false,
                    56 => self.b_queenside_castle = false,
                    63 => self.b_kingside_castle = false,
                    _ => {}
                }
                self.hlf_clock = -1;
            }
            _ => {}
        }
        self.passant_sq = new_passant_sq;

        // Set side-to-move's changed bits
        self.bitboards[moving_piece] ^= moving_bits;
        if moving_piece < 6 {
            self.bitboards[Position::WPIECES] ^= moving_bits;
        } else {
            self.bitboards[Position::BPIECES] ^= moving_bits;
        }

        if !self.is_white_move {
            self.full_num += 1;
        }
        self.hlf_clock += 1; // Toggle halfmove clock.
        self.is_white_move = !self.is_white_move; // Toggle active color.
    }

    #[inline]
    fn whole_board(&self) -> u64 {
        self.bitboards[Position::WPIECES] | self.bitboards[Position::BPIECES]
    }
}

fn get_square_num(file: char, rank: char) -> isize {
    return (file as isize - 'a' as isize) + ((rank as isize - '1' as isize) * 8);
}

impl std::cmp::PartialOrd for Position {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.eval_score.cmp(&other.eval_score))
    }
}

impl std::cmp::Ord for Position {
    fn cmp(&self, other: &Self) -> Ordering {
        self.eval_score.cmp(&other.eval_score)
    }
}

impl std::cmp::PartialEq for Position {
    fn eq(&self, other: &Self) -> bool {
        self.bitboards == other.bitboards
            && self.w_kingside_castle == other.w_kingside_castle
            && self.w_queenside_castle == other.w_queenside_castle
            && self.b_kingside_castle == other.b_kingside_castle
            && self.b_queenside_castle == other.b_queenside_castle
            && self.is_white_move == other.is_white_move
    }
}

impl std::cmp::Eq for Position {}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut fen_string = String::new();
        let mut unoccupied_count = 0;

        // Iterate from 64th -> 1st squares and build fen string in logical order.
        // Left->Right fen ordering will be enforced after initial fen is built.
        for i in 0..=63 {
            let piece_position = self.bitboards.iter().position(|&x| x & square_bit(i) != 0);

            match piece_position {
                Some(index) => {
                    if unoccupied_count != 0 {
                        fen_string += &unoccupied_count.to_string();
                        unoccupied_count = 0;
                    }
                    fen_string += match index {
                        Position::WPAWN => "P",
                        Position::WROOK => "R",
                        Position::WKNIGHT => "N",
                        Position::WBISHOP => "B",
                        Position::WQUEEN => "Q",
                        Position::WKING => "K",
                        Position::BPAWN => "p",
                        Position::BROOK => "r",
                        Position::BKNIGHT => "n",
                        Position::BBISHOP => "b",
                        Position::BQUEEN => "q",
                        Position::BKING => "k",
                        _ => panic!("Error calculating fen string on match: {}", index),
                    };
                }
                None => unoccupied_count += 1,
            }

            // Check for next rank (add '/')
            if (i + 1) % 8 == 0 {
                // Make sure any remaining unoccupied squares are added
                if unoccupied_count != 0 {
                    fen_string += &unoccupied_count.to_string();
                    unoccupied_count = 0;
                }
                fen_string += "/";
            }
        }
        // Remove trailing '/' from constructed fen
        fen_string.pop();

        // Reverse the order of the ranks to be from 8..1 instead of 1..8
        fen_string = fen_string.split('/').rev().collect::<Vec<&str>>().join("/");

        let active_color = if self.is_white_move == true { "w" } else { "b" };

        let mut castling = String::new();
        if self.w_kingside_castle {
            castling += "K";
        }
        if self.w_queenside_castle {
            castling += "Q";
        }
        if self.b_kingside_castle {
            castling += "k";
        }
        if self.b_queenside_castle {
            castling += "q";
        }
        if castling.is_empty() {
            castling += "-";
        }
        let passant = BITBOARD_TO_SQUARE.get(&self.passant_sq).unwrap();

        write!(
            f,
            "{} {} {} {} {} {}",
            fen_string, active_color, castling, passant, self.hlf_clock, self.full_num
        )
    }
}

#[cfg(test)]
mod position_tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    const STARTPOS: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

    // The 6 "complex" positions here are directly from https://www.chessprogramming.org/Perft_Results
    // and are especially useful postions for debugging.
    const COMPLEX_POS_2: &str =
        "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1";
    const COMPLEX_POS_3: &str = "8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1";
    const COMPLEX_POS_4A: &str = "r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1";
    const COMPLEX_POS_4B: &str = "r2q1rk1/pP1p2pp/Q4n2/bbp1p3/Np6/1B3NBn/pPPP1PPP/R3K2R b KQ - 0 1";
    const COMPLEX_POS_5: &str = "rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8";
    const COMPLEX_POS_6: &str =
        "r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10";

    macro_rules! test_square_num {
        ($test_name:ident, $file:literal, $rank:literal, $expected:expr) => {
            #[test]
            fn $test_name() {
                assert_eq!($expected, get_square_num($file, $rank));
            }
        };
    }

    // Valid uci
    test_square_num!(square_num_a1, 'a', '1', 0);
    test_square_num!(square_num_b1, 'b', '1', 1);
    test_square_num!(square_num_c1, 'c', '1', 2);
    test_square_num!(square_num_d1, 'd', '1', 3);
    test_square_num!(square_num_e1, 'e', '1', 4);
    test_square_num!(square_num_f1, 'f', '1', 5);
    test_square_num!(square_num_g1, 'g', '1', 6);
    test_square_num!(square_num_h1, 'h', '1', 7);
    test_square_num!(square_num_a2, 'a', '2', 8);
    test_square_num!(square_num_b2, 'b', '2', 9);
    test_square_num!(square_num_c2, 'c', '2', 10);
    test_square_num!(square_num_d2, 'd', '2', 11);
    test_square_num!(square_num_e2, 'e', '2', 12);
    test_square_num!(square_num_f2, 'f', '2', 13);
    test_square_num!(square_num_g2, 'g', '2', 14);
    test_square_num!(square_num_h2, 'h', '2', 15);
    test_square_num!(square_num_a3, 'a', '3', 16);
    test_square_num!(square_num_b3, 'b', '3', 17);
    test_square_num!(square_num_c3, 'c', '3', 18);
    test_square_num!(square_num_d3, 'd', '3', 19);
    test_square_num!(square_num_e3, 'e', '3', 20);
    test_square_num!(square_num_f3, 'f', '3', 21);
    test_square_num!(square_num_g3, 'g', '3', 22);
    test_square_num!(square_num_h3, 'h', '3', 23);
    test_square_num!(square_num_a4, 'a', '4', 24);
    test_square_num!(square_num_b4, 'b', '4', 25);
    test_square_num!(square_num_c4, 'c', '4', 26);
    test_square_num!(square_num_d4, 'd', '4', 27);
    test_square_num!(square_num_e4, 'e', '4', 28);
    test_square_num!(square_num_f4, 'f', '4', 29);
    test_square_num!(square_num_g4, 'g', '4', 30);
    test_square_num!(square_num_h4, 'h', '4', 31);
    test_square_num!(square_num_a5, 'a', '5', 32);
    test_square_num!(square_num_b5, 'b', '5', 33);
    test_square_num!(square_num_c5, 'c', '5', 34);
    test_square_num!(square_num_d5, 'd', '5', 35);
    test_square_num!(square_num_e5, 'e', '5', 36);
    test_square_num!(square_num_f5, 'f', '5', 37);
    test_square_num!(square_num_g5, 'g', '5', 38);
    test_square_num!(square_num_h5, 'h', '5', 39);
    test_square_num!(square_num_a6, 'a', '6', 40);
    test_square_num!(square_num_b6, 'b', '6', 41);
    test_square_num!(square_num_c6, 'c', '6', 42);
    test_square_num!(square_num_d6, 'd', '6', 43);
    test_square_num!(square_num_e6, 'e', '6', 44);
    test_square_num!(square_num_f6, 'f', '6', 45);
    test_square_num!(square_num_g6, 'g', '6', 46);
    test_square_num!(square_num_h6, 'h', '6', 47);
    test_square_num!(square_num_a7, 'a', '7', 48);
    test_square_num!(square_num_b7, 'b', '7', 49);
    test_square_num!(square_num_c7, 'c', '7', 50);
    test_square_num!(square_num_d7, 'd', '7', 51);
    test_square_num!(square_num_e7, 'e', '7', 52);
    test_square_num!(square_num_f7, 'f', '7', 53);
    test_square_num!(square_num_g7, 'g', '7', 54);
    test_square_num!(square_num_h7, 'h', '7', 55);
    test_square_num!(square_num_a8, 'a', '8', 56);
    test_square_num!(square_num_b8, 'b', '8', 57);
    test_square_num!(square_num_c8, 'c', '8', 58);
    test_square_num!(square_num_d8, 'd', '8', 59);
    test_square_num!(square_num_e8, 'e', '8', 60);
    test_square_num!(square_num_f8, 'f', '8', 61);
    test_square_num!(square_num_g8, 'g', '8', 62);
    test_square_num!(square_num_h8, 'h', '8', 63);

    #[test]
    fn test_new_constructor_bitboards() {
        let position = Position::new();
        assert_eq!(0x000000000000FF00, position.bitboards[Position::WPAWN]);
        assert_eq!(0x00FF000000000000, position.bitboards[Position::BPAWN]);
        assert_eq!(0x0000000000000081, position.bitboards[Position::WROOK]);
        assert_eq!(0x8100000000000000, position.bitboards[Position::BROOK]);
        assert_eq!(0x0000000000000042, position.bitboards[Position::WKNIGHT]);
        assert_eq!(0x4200000000000000, position.bitboards[Position::BKNIGHT]);
        assert_eq!(0x0000000000000024, position.bitboards[Position::WBISHOP]);
        assert_eq!(0x2400000000000000, position.bitboards[Position::BBISHOP]);
        assert_eq!(0x0000000000000008, position.bitboards[Position::WQUEEN]);
        assert_eq!(0x0800000000000000, position.bitboards[Position::BQUEEN]);
        assert_eq!(0x0000000000000010, position.bitboards[Position::WKING]);
        assert_eq!(0x1000000000000000, position.bitboards[Position::BKING]);

        let boards = position.bitboards;
        let manual_w_pieces = boards[Position::WPAWN]
            | boards[Position::WROOK]
            | boards[Position::WKNIGHT]
            | boards[Position::WBISHOP]
            | boards[Position::WQUEEN]
            | boards[Position::WKING];

        assert_eq!(0x000000000000FFFF, manual_w_pieces);
        assert_eq!(0x000000000000FFFF, boards[Position::WPIECES]);

        let manual_b_pieces = boards[Position::BPAWN]
            | boards[Position::BROOK]
            | boards[Position::BKNIGHT]
            | boards[Position::BBISHOP]
            | boards[Position::BQUEEN]
            | boards[Position::BKING];

        assert_eq!(0xFFFF000000000000, manual_b_pieces);
        assert_eq!(0xFFFF000000000000, boards[Position::BPIECES]);
    }

    #[test]
    fn test_from_constructor_bitboards_startpos() {
        let position = Position::from(STARTPOS);
        assert_eq!(0x000000000000FF00, position.bitboards[Position::WPAWN]);
        assert_eq!(0x00FF000000000000, position.bitboards[Position::BPAWN]);
        assert_eq!(0x0000000000000081, position.bitboards[Position::WROOK]);
        assert_eq!(0x8100000000000000, position.bitboards[Position::BROOK]);
        assert_eq!(0x0000000000000042, position.bitboards[Position::WKNIGHT]);
        assert_eq!(0x4200000000000000, position.bitboards[Position::BKNIGHT]);
        assert_eq!(0x0000000000000024, position.bitboards[Position::WBISHOP]);
        assert_eq!(0x2400000000000000, position.bitboards[Position::BBISHOP]);
        assert_eq!(0x0000000000000008, position.bitboards[Position::WQUEEN]);
        assert_eq!(0x0800000000000000, position.bitboards[Position::BQUEEN]);
        assert_eq!(0x0000000000000010, position.bitboards[Position::WKING]);
        assert_eq!(0x1000000000000000, position.bitboards[Position::BKING]);

        let boards = position.bitboards;
        let manual_w_pieces = boards[Position::WPAWN]
            | boards[Position::WROOK]
            | boards[Position::WKNIGHT]
            | boards[Position::WBISHOP]
            | boards[Position::WQUEEN]
            | boards[Position::WKING];

        assert_eq!(0x000000000000FFFF, manual_w_pieces);
        assert_eq!(0x000000000000FFFF, boards[Position::WPIECES]);

        let manual_b_pieces = boards[Position::BPAWN]
            | boards[Position::BROOK]
            | boards[Position::BKNIGHT]
            | boards[Position::BBISHOP]
            | boards[Position::BQUEEN]
            | boards[Position::BKING];

        assert_eq!(0xFFFF000000000000, manual_b_pieces);
        assert_eq!(0xFFFF000000000000, boards[Position::BPIECES]);
    }

    #[test]
    fn test_from_constructor_bitboards_1() {
        let position = Position::from("1R6/1P6/k7/1R2K3/1p1P4/1pnr1bP1/BP1b4/3r4 w - - 0 1");
        assert_eq!(0x0002000008400200, position.bitboards[Position::WPAWN]);
        assert_eq!(0x0000000002020000, position.bitboards[Position::BPAWN]);
        assert_eq!(0x0200000200000000, position.bitboards[Position::WROOK]);
        assert_eq!(0x0000000000080008, position.bitboards[Position::BROOK]);
        assert_eq!(0x0000000000000000, position.bitboards[Position::WKNIGHT]);
        assert_eq!(0x0000000000040000, position.bitboards[Position::BKNIGHT]);
        assert_eq!(0x0000000000000100, position.bitboards[Position::WBISHOP]);
        assert_eq!(0x0000000000200800, position.bitboards[Position::BBISHOP]);
        assert_eq!(0x0000000000000000, position.bitboards[Position::WQUEEN]);
        assert_eq!(0x0000000000000000, position.bitboards[Position::BQUEEN]);
        assert_eq!(0x0000001000000000, position.bitboards[Position::WKING]);
        assert_eq!(0x0000010000000000, position.bitboards[Position::BKING]);

        let boards = position.bitboards;
        let manual_w_pieces = boards[Position::WPAWN]
            | boards[Position::WROOK]
            | boards[Position::WKNIGHT]
            | boards[Position::WBISHOP]
            | boards[Position::WQUEEN]
            | boards[Position::WKING];

        assert_eq!(0x0202001208400300, manual_w_pieces);
        assert_eq!(0x0202001208400300, boards[Position::WPIECES]);

        let manual_b_pieces = boards[Position::BPAWN]
            | boards[Position::BROOK]
            | boards[Position::BKNIGHT]
            | boards[Position::BBISHOP]
            | boards[Position::BQUEEN]
            | boards[Position::BKING];

        assert_eq!(0x00000100022E0808, manual_b_pieces);
        assert_eq!(0x00000100022E0808, boards[Position::BPIECES]);
    }

    #[test]
    fn test_position_ordering() {
        let mut position_vec = vec![Position::new(), Position::new(), Position::new()];
        position_vec[0].eval_score = 10;
        position_vec[1].eval_score = 5;
        position_vec[2].eval_score = 15;

        position_vec.sort();

        assert_eq!(position_vec[0].eval_score, 5);
        assert_eq!(position_vec[1].eval_score, 10);
        assert_eq!(position_vec[2].eval_score, 15);

        position_vec[0].eval_score = 10;
        position_vec[1].eval_score = 10;
        position_vec[2].eval_score = 11;

        position_vec.sort();

        assert_eq!(position_vec[0].eval_score, 10);
        assert_eq!(position_vec[1].eval_score, 10);
        assert_eq!(position_vec[2].eval_score, 11);
    }

    // Test that generated fen string matches the fen used to construct the position
    macro_rules! test_fen {
        ($test_name:ident, $fen:expr) => {
            #[test]
            fn $test_name() {
                assert_eq!($fen, Position::from($fen).to_string());
            }
        };
    }

    test_fen!(fen_startpos, STARTPOS);

    test_fen!(
        fen_startpos_e4,
        "rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1"
    );

    test_fen!(
        fen_startpos_e4_c5,
        "rnbqkbnr/pp1ppppp/8/2p5/4P3/8/PPPP1PPP/RNBQKBNR w KQkq c6 0 2"
    );

    test_fen!(
        fen_complex_pos,
        "1kb3r1/prp1bppp/1p1p1R1P/1Nn1p1Bq/3PnQ2/2P1PNP1/PPB2P2/2KR4 w - - 0 1"
    );

    test_fen!(fen_empty_pos, "8/8/8/8/8/8/8/8 w KQkq - 0 1");
    test_fen!(fen_complex_2, COMPLEX_POS_2);
    test_fen!(fen_complex_3, COMPLEX_POS_3);
    test_fen!(fen_complex_4a, COMPLEX_POS_4A);
    test_fen!(fen_complex_4b, COMPLEX_POS_4B);
    test_fen!(fen_complex_5, COMPLEX_POS_5);
    test_fen!(fen_complex_6, COMPLEX_POS_6);

    // Test castling rights after position construction using Position::from
    #[test]
    fn test_from_constructor_castling_w_kingside() {
        let position = Position::from("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w K - 0 1");
        assert!(position.w_kingside_castle);
        assert!(!position.w_queenside_castle);
        assert!(!position.b_kingside_castle);
        assert!(!position.b_queenside_castle);
    }

    #[test]
    fn test_from_constructor_castling_w_queenside() {
        let position = Position::from("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w Q - 0 1");
        assert!(!position.w_kingside_castle);
        assert!(position.w_queenside_castle);
        assert!(!position.b_kingside_castle);
        assert!(!position.b_queenside_castle);
    }

    #[test]
    fn test_from_constructor_castling_b_kingside() {
        let position = Position::from("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w k - 0 1");
        assert!(!position.w_kingside_castle);
        assert!(!position.w_queenside_castle);
        assert!(position.b_kingside_castle);
        assert!(!position.b_queenside_castle);
    }

    #[test]
    fn test_from_constructor_castling_b_queenside() {
        let position = Position::from("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w q - 0 1");
        assert!(!position.w_kingside_castle);
        assert!(!position.w_queenside_castle);
        assert!(!position.b_kingside_castle);
        assert!(position.b_queenside_castle);
    }

    #[test]
    fn test_from_constructor_castling_w() {
        let position = Position::from("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQ - 0 1");
        assert!(position.w_kingside_castle);
        assert!(position.w_queenside_castle);
        assert!(!position.b_kingside_castle);
        assert!(!position.b_queenside_castle);
    }

    #[test]
    fn test_from_constructor_castling_b() {
        let position = Position::from("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w kq - 0 1");
        assert!(!position.w_kingside_castle);
        assert!(!position.w_queenside_castle);
        assert!(position.b_kingside_castle);
        assert!(position.b_queenside_castle);
    }

    #[test]
    fn test_move_a2a4() {
        let mut position = Position::new();
        position.perform_move("a2a4");
        assert_eq!(
            "rnbqkbnr/pppppppp/8/8/P7/8/1PPPPPPP/RNBQKBNR b KQkq a3 0 1",
            position.to_string()
        );
    }
    macro_rules! test_moves {
        ($test_name:ident, $moves:expr, $start_fen:expr, $end_fen:expr) => {
            #[test]
            fn $test_name() {
                let mut position = Position::from($start_fen);
                for move_str in $moves {
                    position.perform_move(move_str);
                }
                assert_eq!($end_fen, position.to_string());
            }
        };
    }

    test_moves!(
        test_moves_a2a4,
        vec!["a2a4"],
        STARTPOS,
        "rnbqkbnr/pppppppp/8/8/P7/8/1PPPPPPP/RNBQKBNR b KQkq a3 0 1"
    );
    test_moves!(
        test_moves_b1c3,
        vec!["b1c3"],
        STARTPOS,
        "rnbqkbnr/pppppppp/8/8/8/2N5/PPPPPPPP/R1BQKBNR b KQkq - 1 1"
    );
    test_moves!(
        test_moves_startpos_to_w_king_castling,
        vec!["e2e4", "b8c6", "f1d3", "h7h6", "g1h3", "a8b8", "e1g1"],
        STARTPOS,
        "1rbqkbnr/ppppppp1/2n4p/8/4P3/3B3N/PPPP1PPP/RNBQ1RK1 b k - 0 4"
    );
}
