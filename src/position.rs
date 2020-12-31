pub struct Position {
    pieces: [u64; 14],
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

enum MapNames {
    WPawn,
    WRook,
    WKnight,
    WBishop,
    WQueen,
    WKing,
    WPieces,
    BPawn,
    BRook,
    BKnight,
    BBishop,
    BQueen,
    BKing,
    BPieces,
}

impl Position {
    // Returns a position containing the starting chess position.
    // Fen string: rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1
    pub fn new() -> Position {
        Position {
            pieces: [
                0x00000000FF00,
                0x000000000081,
                0x000000000042,
                0x000000000024,
                0x000000000008,
                0x000000000010,
                0x00000000FFFF,
                0x00FF00000000,
                0x810000000000,
                0x420000000000,
                0x240000000000,
                0x080000000000,
                0x100000000000,
                0xFFFF00000000,
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

        let mut char_iter = passant_sq_str.chars();
        let file = char_iter.next().unwrap();
        let rank = char_iter.next().unwrap();
        let passant_sq = square_bit(get_square_num(file, rank));

        let hlf_clock = iter.next().unwrap().parse().unwrap();
        let full_num: usize = iter.next().unwrap().parse().unwrap();

        let mut pieces_arr = [0; 14];
        let mut square_num: u32 = 63;
        let pieces_by_rank = piece_positions.split("/");
        for rank in pieces_by_rank {
            for piece in rank.chars().rev() {
                let current_piece = match piece {
                    'P' => MapNames::WPawn,
                    'p' => MapNames::BPawn,
                    'R' => MapNames::WRook,
                    'r' => MapNames::BRook,
                    'N' => MapNames::WKnight,
                    'n' => MapNames::BKnight,
                    'B' => MapNames::WBishop,
                    'b' => MapNames::BBishop,
                    'Q' => MapNames::WQueen,
                    'q' => MapNames::BQueen,
                    'K' => MapNames::WKing,
                    'k' => MapNames::BKing,
                    '1'..='8' => {
                        square_num -= piece.to_digit(10).unwrap();
                        continue;
                    }
                    _ => panic!("failed to build position"),
                };
                pieces_arr[current_piece as usize] |= square_bit(square_num);
                square_num -= 1;
            }
        }

        Position {
            pieces: pieces_arr,
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

    pub fn to_string(&self) -> String {
        let mut fen_string = String::new();
        for i in 0..63 {
            for (j, bitboard) in self.pieces.iter().enumerate() {
                if bitboard & square_bit(i) {
                    match j {
                        0 => fen_string = String::from("P") + fen_string,
                    }
                }
            }
        }
        return String::from("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
    }
}

const ONE: u64 = 0x01;

pub fn get_square_num(file: char, rank: char) -> u32 {
    return (file as u32 - 'a' as u32) + ((rank as u32 - '1' as u32) * 8);
}

pub const fn square_bit(i: u32) -> u64 {
    return ONE << i;
}
