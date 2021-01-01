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

const fn square_bit(i: usize) -> u64 {
    return ONE << i;
}

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
    hlf_clock: usize,
    full_num: usize,
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
        let full_num: usize = iter.next().unwrap().parse().unwrap();

        let mut pieces_arr = [0; 14];
        let fen: String = piece_positions.split('/').rev().collect();
        let mut square_num: usize = 0;

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
                    square_num += num as usize;
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
            let mut occupied = false;
            for (j, bitboard) in self.bitboards.iter().enumerate() {
                if (bitboard & square_bit(i)) != 0 {
                    occupied = true;
                    if unoccupied_count != 0 {
                        fen_string += &unoccupied_count.to_string();
                        unoccupied_count = 0;
                    }
                    fen_string += match j {
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
                        _ => panic!("Error calculating fen string on match: {}", j),
                    };
                    println!("{}", fen_string);
                    break; // Don't continue searching bitboards after a match on this square
                }
            }

            if !occupied {
                // No bitboards had a set bit on this square
                unoccupied_count += 1;
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
        fen_string.pop();
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

fn get_square_num(file: char, rank: char) -> usize {
    return (file as usize - 'a' as usize) + ((rank as usize - '1' as usize) * 8);
}

#[cfg(test)]
mod position_tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

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
        let position = Position::from("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
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

    macro_rules! test_fen {
        ($test_name:ident, $fen:literal) => {
            #[test]
            fn $test_name() {
                assert_eq!($fen, Position::from($fen).to_string());
            }
        };
    }

    test_fen!(
        fen_startpos,
        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"
    );

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
}
