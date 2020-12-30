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
        let is_white_move = iter.next().unwrap();
        let castle_rights = iter.next().unwrap();
        let passant_sq = iter.next().unwrap();

        let hlf_clock = iter.next().unwrap().parse().unwrap();
        let full_num: usize = iter.next().unwrap().parse().unwrap();

        let pieces_arr = [0; 14];
        Position {
            pieces: pieces_arr,
            passant_sq: 0,
            moves: vec![],
            w_kingside_castle: castle_rights.contains("K"),
            w_queenside_castle: castle_rights.contains("Q"),
            b_kingside_castle: castle_rights.contains("k"),
            b_queenside_castle: castle_rights.contains("q"),
            eval_score: 0,
            is_white_move: true,
            hlf_clock: hlf_clock,
            full_num: full_num,
        }
    }
}

const ONE: u64 = 0x01;

pub fn get_square_num(file: char, rank: char) -> usize {
    return (file as usize - 'a' as usize) + ((rank as usize - '1' as usize) * 8);
}

pub const fn square_bit(i: usize) -> u64 {
    return ONE << i;
}
