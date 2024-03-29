// position.rs will primarily serve to expose the Position struct. A Position
// represents a single chess position. It includes fields representing the
// color to move, current piece positions, castling rights, etc. A Position
// serves as a complete snapshot of a point in time of a chess match.

// FILE constants: bitboards representing their respective files of the board with
// 1's set in the bit positions for the file, and 0's otherwise.

use std::fmt;

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

const CORNERS: u64 = (RANK_1 | RANK_8) & (A_FILE | H_FILE);

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

// The representation of a 'Move' is a 16-bit integer.
// This implementation choice is inspired by
// https://www.chessprogramming.org/Encoding_Moves as well as Stockfish's own move implementation.
//
// bit '0' of challenger's Move will represent the least significant bit position.
// bit  0- 5: origin square (from 0 to 63)
// bit  6-11: destination square (from 0 to 63)
// bit 12-13: promotion piece type - 2 (from KNIGHT-2 to QUEEN-2)
// bit 14-15: special move flag: promotion (1), en passant (2), castling (3)
// SPECIAL CASE: To represent pawn double forward moves, the promotion bits will
// all be set but the special move flag will be 0 (normal move).

type Move = u16;
const ORIGIN_SQ_BITS: u16 = 0x3F;

const DEST_BITS_OFFSET: u32 = ORIGIN_SQ_BITS.count_ones();
const DEST_SQ_BITS: u16 = ORIGIN_SQ_BITS << DEST_BITS_OFFSET;

const PROMOTION_PIECE_BITS_OFFSET: u32 = DEST_BITS_OFFSET + DEST_SQ_BITS.count_ones();

const TWO_BITS: u16 = 0x3;
const PROMOTION_PIECE_BITS: u16 = TWO_BITS << PROMOTION_PIECE_BITS_OFFSET;

const SPECIAL_MOVE_BITS_OFFSET: u32 =
    PROMOTION_PIECE_BITS_OFFSET + PROMOTION_PIECE_BITS.count_ones();
const SPECIAL_MOVE_BITS: u16 = TWO_BITS << SPECIAL_MOVE_BITS_OFFSET;

// Special move types
const PROMOTION: Move = 0x1 << SPECIAL_MOVE_BITS_OFFSET;
const ENPASSANT: Move = 0x2 << SPECIAL_MOVE_BITS_OFFSET;
const CASTLING: Move = 0x3 << SPECIAL_MOVE_BITS_OFFSET;
const PAWN_DOUBLE_FWD: Move = 0x3 << PROMOTION_PIECE_BITS_OFFSET;

pub fn str_to_move(move_string: &str, position: Position) -> Move {
    let mut move_bits: Move = 0;
    let mut move_chars = move_string.chars();

    let start_sq_num = sq_num(move_chars.next().unwrap(), move_chars.next().unwrap());
    let dest_sq_num = sq_num(move_chars.next().unwrap(), move_chars.next().unwrap());
    let promotion = move_chars.next();
    let is_king_move =
        (position.pieces[W_KING] & position.pieces[B_KING]) & (1u64 << start_sq_num) != 0;
    let sq_diff = start_sq_num as isize - dest_sq_num as isize;

    move_bits |= start_sq_num as u16;
    move_bits |= (dest_sq_num as u16) << DEST_BITS_OFFSET;

    if promotion.is_some() {
        match promotion.unwrap() {
            'Q' | 'q' => move_bits |= 3 << 12,
            'R' | 'r' => move_bits |= 2 << 12,
            'B' | 'b' => move_bits |= 1 << 12,
            _ => (), // Since knights are 0's, no need to do anything
        }
        move_bits |= PROMOTION;
    } else if 1u64 << dest_sq_num == position.passant_sq {
        move_bits |= ENPASSANT;
    } else if is_king_move && sq_diff == 2 {
        move_bits |= CASTLING;
    }

    return move_bits;
}

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

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "pieces: [")?;
        for (i, elem) in self.pieces.iter().enumerate() {
            if i != 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}", elem)?;
        }
        write!(f, "], ")?;
        write!(f, "passant: {}, ", self.passant_sq)?;
        write!(f, "w_king_castle {}, ", self.w_king_castle)?;
        write!(f, "w_queen_castle {}, ", self.w_queen_castle)?;
        write!(f, "b_king_castle {}, ", self.b_king_castle)?;
        write!(f, "b_queen_castle {}, ", self.b_queen_castle)?;
        write!(f, "is_white_move {}, ", self.is_white_move)?;
        write!(f, "hlf_clock {}, ", self.hlf_clock)?;
        write!(f, "full_num {}", self.full_num)
    }
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

    // The play_move() function attempts to play the requested move and apply the rules
    // of chess to the board. It will not consider the legality of the move it is given,
    // and will instead just apply regular chess logic to that move. For example, a king
    // *could* jump across the board and capture a friendly piece with this function,
    // however the castling rights for the side to play would still be removed, and the
    // side to play would be toggled.
    //
    // The focus of the play_move function is speed instead of legality, as challenger
    // has a strictly legal move generator. Moves from stdin could still supply the
    // engine with illegal moves, in which case the engine will gladly play them.
    pub fn play_move(&mut self, move_bits: Move) {
        // Increment halfmove clock early. Resets will happen based on move played
        self.hlf_clock += 1;
        self.full_num += !self.is_white_move as u8;

        let self_offset: usize = (!self.is_white_move as usize) * 7;
        self.is_white_move = !self.is_white_move;

        let start_sq_num = move_bits & 0x3F;
        let dest_sq_num = (move_bits >> 6) & 0x3F;
        let start_square = 1u64 << start_sq_num;
        let dest_square = 1u64 << dest_sq_num;
        let sq_diff = start_sq_num as isize - dest_sq_num as isize;

        let promotion_piece = (move_bits >> 12) & 3;

        let moving_bits = start_square | dest_square;

        // If a capture is taking place, zero out the destination square
        if (self.pieces[W_PIECES] | self.pieces[B_PIECES]) & dest_square != 0 {
            let dest_zero_mask = !dest_square;
            for piece in &mut self.pieces {
                *piece &= dest_zero_mask;
            }

            self.hlf_clock = 0; // Reset halfmove clock on a capture
        }

        let moving_piece = self
            .pieces
            .iter()
            .position(|&x| x & start_square != 0)
            .unwrap();

        let passant_prev = self.passant_sq;
        self.passant_sq = 0;

        match moving_piece {
            W_PAWN | B_PAWN => {
                if dest_square & passant_prev != 0 {
                    let dest_zero = if moving_piece == W_PAWN {
                        !(dest_square >> 8)
                    } else {
                        !(dest_square << 8)
                    };
                    self.pieces[W_PIECES] &= dest_zero;
                    self.pieces[B_PIECES] &= dest_zero;
                    self.pieces[W_PAWN] &= dest_zero;
                    self.pieces[B_PAWN] &= dest_zero;
                } else if sq_diff.abs() == 16 {
                    self.passant_sq = 1u64 << ((start_sq_num + dest_sq_num) / 2);
                } else if dest_square & (RANK_1 | RANK_8) != 0 {
                    // Set the destination square bit in the pawn bitboard. It will
                    // be unset when the moving_bits xor operation occurs.
                    self.pieces[moving_piece] |= dest_square;

                    // Set the promoted piece
                    match promotion_piece {
                        3 => self.pieces[W_QUEEN + self_offset] |= dest_square,
                        2 => self.pieces[W_ROOK + self_offset] |= dest_square,
                        1 => self.pieces[W_BISHOP + self_offset] |= dest_square,
                        0 => self.pieces[W_KNIGHT + self_offset] |= dest_square,
                        _ => (),
                    }
                }
                self.hlf_clock = 0;
            }
            W_KING => {
                self.w_king_castle = false;
                self.w_queen_castle = false;
                if sq_diff == 2 {
                    // Queenside Castling
                    self.pieces[W_ROOK] ^= 0x0000000000000009;
                    self.pieces[W_PIECES] ^= 0x0000000000000009;
                } else if sq_diff == -2 {
                    // Kingside Castling
                    self.pieces[W_ROOK] ^= 0x00000000000000A0;
                    self.pieces[W_PIECES] ^= 0x00000000000000A0;
                }
            }
            B_KING => {
                self.b_king_castle = false;
                self.b_queen_castle = false;
                if sq_diff == 2 {
                    // Queenside Castling
                    self.pieces[B_ROOK] ^= 0x0900000000000000;
                    self.pieces[B_PIECES] ^= 0x0900000000000000;
                } else if sq_diff == -2 {
                    // Kingside Castling
                    self.pieces[B_ROOK] ^= 0xA000000000000000;
                    self.pieces[B_PIECES] ^= 0xA000000000000000;
                }
            }
            W_ROOK | B_ROOK if start_square & CORNERS != 0 => match start_sq_num {
                1 => self.w_queen_castle = false,
                7 => self.w_king_castle = false,
                56 => self.b_queen_castle = false,
                63 => self.b_king_castle = false,
                _ => (),
            },
            _ => (),
        }

        self.pieces[moving_piece] ^= moving_bits;
        if moving_piece < 6 {
            self.pieces[W_PIECES] ^= moving_bits;
        } else {
            self.pieces[B_PIECES] ^= moving_bits;
        }
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

    // Generate moves that can be performed from the current position
    pub fn moves(self) -> Vec<Move> {
        return self.generate_knight_moves();
    }

    fn generate_knight_moves(self) -> Vec<Move> {
        let mut moves: Vec<Move> = Vec::new();

        let mut knights;
        let friendly_pieces;
        if self.is_white_move {
            knights = self.pieces[W_KNIGHT];
            friendly_pieces = self.pieces[W_PIECES];
        } else {
            knights = self.pieces[B_KNIGHT];
            friendly_pieces = self.pieces[B_PIECES];
        };

        // Add knight moves
        while knights != 0 {
            let index = knights.trailing_zeros();
            moves.extend(KNIGHT_MOVES[index as usize].iter());
            knights ^= 1 << index;
        }

        moves = moves
            .into_iter()
            .filter(|&x| {
                let dest_sq_index = (x & DEST_SQ_BITS) >> DEST_BITS_OFFSET;
                let dest_sq = 1u64 << dest_sq_index;
                dest_sq & friendly_pieces == 0
            })
            .collect();

        return moves;
    }
}

lazy_static! {
    static ref KNIGHT_MOVES: Vec<Vec<Move>> = vec![
        vec![640, 1088,],
        vec![705, 1025, 1153,],
        vec![514, 770, 1090, 1218,],
        vec![579, 835, 1155, 1283,],
        vec![644, 900, 1220, 1348,],
        vec![709, 965, 1285, 1413,],
        vec![774, 1350, 1478,],
        vec![839, 1415,],
        vec![136, 1160, 1608,],
        vec![201, 1225, 1545, 1673,],
        vec![10, 266, 1034, 1290, 1610, 1738,],
        vec![75, 331, 1099, 1355, 1675, 1803,],
        vec![140, 396, 1164, 1420, 1740, 1868,],
        vec![205, 461, 1229, 1485, 1805, 1933,],
        vec![270, 1294, 1870, 1998,],
        vec![335, 1359, 1935,],
        vec![80, 656, 1680, 2128,],
        vec![17, 145, 721, 1745, 2065, 2193,],
        vec![82, 210, 530, 786, 1554, 1810, 2130, 2258,],
        vec![147, 275, 595, 851, 1619, 1875, 2195, 2323,],
        vec![212, 340, 660, 916, 1684, 1940, 2260, 2388,],
        vec![277, 405, 725, 981, 1749, 2005, 2325, 2453,],
        vec![342, 470, 790, 1814, 2390, 2518,],
        vec![407, 855, 1879, 2455,],
        vec![600, 1176, 2200, 2648,],
        vec![537, 665, 1241, 2265, 2585, 2713,],
        vec![602, 730, 1050, 1306, 2074, 2330, 2650, 2778,],
        vec![667, 795, 1115, 1371, 2139, 2395, 2715, 2843,],
        vec![732, 860, 1180, 1436, 2204, 2460, 2780, 2908,],
        vec![797, 925, 1245, 1501, 2269, 2525, 2845, 2973,],
        vec![862, 990, 1310, 2334, 2910, 3038,],
        vec![927, 1375, 2399, 2911,],
        vec![1120, 1696, 2720, 3168,],
        vec![1057, 1185, 1761, 2785, 3105, 3233,],
        vec![1122, 1250, 1570, 1826, 2594, 2850, 3170, 3298,],
        vec![1187, 1315, 1635, 1891, 2659, 2915, 3235, 3363,],
        vec![1252, 1380, 1700, 1956, 2724, 2980, 3300, 3428,],
        vec![1317, 1445, 1765, 2021, 2789, 3045, 3365, 3493,],
        vec![1382, 1510, 1830, 2854, 3430, 3558,],
        vec![1447, 1895, 2919, 3495,],
        vec![1640, 2216, 3240, 3688,],
        vec![1577, 1705, 2281, 3305, 3625, 3753,],
        vec![1642, 1770, 2090, 2346, 3114, 3370, 3690, 3818,],
        vec![1707, 1835, 2155, 2411, 3179, 3435, 3755, 3883,],
        vec![1772, 1900, 2220, 2476, 3244, 3500, 3820, 3948,],
        vec![1837, 1965, 2285, 2541, 3309, 3565, 3885, 4013,],
        vec![1902, 2030, 2350, 3374, 3950, 4078,],
        vec![1967, 2415, 3439, 4015,],
        vec![2160, 2736, 3760,],
        vec![2097, 2225, 2801, 3825,],
        vec![2162, 2290, 2610, 2866, 3634, 3890,],
        vec![2227, 2355, 2675, 2931, 3699, 3955,],
        vec![2292, 2420, 2740, 2996, 3764, 4020,],
        vec![2357, 2485, 2805, 3061, 3829, 4085,],
        vec![2422, 2550, 2870, 3894,],
        vec![2487, 2935, 3959,],
        vec![2680, 3256,],
        vec![2617, 2745, 3321,],
        vec![2682, 2810, 3130, 3386,],
        vec![2747, 2875, 3195, 3451,],
        vec![2812, 2940, 3260, 3516,],
        vec![2877, 3005, 3325, 3581,],
        vec![2942, 3070, 3390,],
        vec![2943, 3455,]
    ];
    static ref KING_MOVES: Vec<Vec<Move>> = vec![
        vec![64, 512, 576,],
        vec![1, 129, 513, 577, 641,],
        vec![66, 194, 578, 642, 706,],
        vec![131, 259, 643, 707, 771,],
        vec![196, 324, 708, 772, 836,],
        vec![261, 389, 773, 837, 901,],
        vec![326, 454, 838, 902, 966,],
        vec![327, 903, 967,],
        vec![8, 72, 584, 1032, 1096,],
        vec![9, 73, 137, 521, 649, 1033, 1097, 1161,],
        vec![74, 138, 202, 586, 714, 1098, 1162, 1226,],
        vec![139, 203, 267, 651, 779, 1163, 1227, 1291,],
        vec![204, 268, 332, 716, 844, 1228, 1292, 1356,],
        vec![269, 333, 397, 781, 909, 1293, 1357, 1421,],
        vec![334, 398, 462, 846, 974, 1358, 1422, 1486,],
        vec![335, 463, 911, 1423, 1487,],
        vec![528, 592, 1104, 1552, 1616,],
        vec![529, 593, 657, 1041, 1169, 1553, 1617, 1681,],
        vec![594, 658, 722, 1106, 1234, 1618, 1682, 1746,],
        vec![659, 723, 787, 1171, 1299, 1683, 1747, 1811,],
        vec![724, 788, 852, 1236, 1364, 1748, 1812, 1876,],
        vec![789, 853, 917, 1301, 1429, 1813, 1877, 1941,],
        vec![854, 918, 982, 1366, 1494, 1878, 1942, 2006,],
        vec![919, 983, 1431, 1943, 2007,],
        vec![1048, 1112, 1624, 2072, 2136,],
        vec![1049, 1113, 1177, 1561, 1689, 2073, 2137, 2201,],
        vec![1114, 1178, 1242, 1626, 1754, 2138, 2202, 2266,],
        vec![1179, 1243, 1307, 1691, 1819, 2203, 2267, 2331,],
        vec![1244, 1308, 1372, 1756, 1884, 2268, 2332, 2396,],
        vec![1309, 1373, 1437, 1821, 1949, 2333, 2397, 2461,],
        vec![1374, 1438, 1502, 1886, 2014, 2398, 2462, 2526,],
        vec![1439, 1503, 1951, 2463, 2527,],
        vec![1568, 1632, 2144, 2592, 2656,],
        vec![1569, 1633, 1697, 2081, 2209, 2593, 2657, 2721,],
        vec![1634, 1698, 1762, 2146, 2274, 2658, 2722, 2786,],
        vec![1699, 1763, 1827, 2211, 2339, 2723, 2787, 2851,],
        vec![1764, 1828, 1892, 2276, 2404, 2788, 2852, 2916,],
        vec![1829, 1893, 1957, 2341, 2469, 2853, 2917, 2981,],
        vec![1894, 1958, 2022, 2406, 2534, 2918, 2982, 3046,],
        vec![1959, 2023, 2471, 2983, 3047,],
        vec![2088, 2152, 2664, 3112, 3176,],
        vec![2089, 2153, 2217, 2601, 2729, 3113, 3177, 3241,],
        vec![2154, 2218, 2282, 2666, 2794, 3178, 3242, 3306,],
        vec![2219, 2283, 2347, 2731, 2859, 3243, 3307, 3371,],
        vec![2284, 2348, 2412, 2796, 2924, 3308, 3372, 3436,],
        vec![2349, 2413, 2477, 2861, 2989, 3373, 3437, 3501,],
        vec![2414, 2478, 2542, 2926, 3054, 3438, 3502, 3566,],
        vec![2479, 2543, 2927, 3503, 3567,],
        vec![2608, 2672, 3184, 3632, 3696,],
        vec![2609, 2673, 2737, 3121, 3249, 3633, 3697, 3761,],
        vec![2674, 2738, 2802, 3186, 3314, 3698, 3762, 3826,],
        vec![2739, 2803, 2867, 3251, 3379, 3763, 3827, 3891,],
        vec![2804, 2868, 2932, 3316, 3444, 3828, 3892, 3956,],
        vec![2869, 2933, 2997, 3381, 3509, 3893, 3957, 4021,],
        vec![2934, 2998, 3062, 3446, 3574, 3958, 4022, 4086,],
        vec![2999, 3063, 3511, 4023, 4087,],
        vec![3128, 3192, 3704,],
        vec![3129, 3193, 3257, 3641, 3769,],
        vec![3194, 3258, 3322, 3706, 3834,],
        vec![3259, 3323, 3387, 3771, 3899,],
        vec![3324, 3388, 3452, 3836, 3964,],
        vec![3389, 3453, 3517, 3901, 4029,],
        vec![3454, 3518, 3582, 3966, 4094,],
        vec![3519, 3583, 4031,],
    ];
}

pub fn sq_num(file: char, rank: char) -> u32 {
    (file as u32 - 'a' as u32) + ((rank as u32 - '1' as u32) * 8)
}

pub fn sq_to_bitboard(file: char, rank: char) -> u64 {
    1u64 << sq_num(file, rank)
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

    const STARTPOS: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

    test_pieces!(startpos_w_pawn, STARTPOS, W_PAWN, 0x000000000000FF00);
    test_pieces!(startpos_w_rook, STARTPOS, W_ROOK, 0x0000000000000081);
    test_pieces!(startpos_w_knight, STARTPOS, W_KNIGHT, 0x0000000000000042);
    test_pieces!(startpos_w_bishop, STARTPOS, W_BISHOP, 0x0000000000000024);
    test_pieces!(startpos_w_queen, STARTPOS, W_QUEEN, 0x0000000000000008);
    test_pieces!(startpos_w_king, STARTPOS, W_KING, 0x0000000000000010);
    test_pieces!(startpos_w_pieces, STARTPOS, W_PIECES, 0x000000000000FFFF);
    test_pieces!(startpos_b_pawn, STARTPOS, B_PAWN, 0x00FF000000000000);
    test_pieces!(startpos_b_rook, STARTPOS, B_ROOK, 0x8100000000000000);
    test_pieces!(startpos_b_knight, STARTPOS, B_KNIGHT, 0x4200000000000000);
    test_pieces!(startpos_b_bishop, STARTPOS, B_BISHOP, 0x2400000000000000);
    test_pieces!(startpos_b_queen, STARTPOS, B_QUEEN, 0x0800000000000000);
    test_pieces!(startpos_b_king, STARTPOS, B_KING, 0x1000000000000000);
    test_pieces!(startpos_b_pieces, STARTPOS, B_PIECES, 0xFFFF000000000000);

    const EMPTY_POS: &str = "8/8/8/8/8/8/8/8 w KQkq - 0 1";

    test_pieces!(empty_w_pawn, EMPTY_POS, W_PAWN, 0);
    test_pieces!(empty_w_rook, EMPTY_POS, W_PAWN, 0);
    test_pieces!(empty_w_knight, EMPTY_POS, W_PAWN, 0);
    test_pieces!(empty_w_bishop, EMPTY_POS, W_PAWN, 0);
    test_pieces!(empty_w_queen, EMPTY_POS, W_PAWN, 0);
    test_pieces!(empty_w_king, EMPTY_POS, W_PAWN, 0);
    test_pieces!(empty_w_pieces, EMPTY_POS, W_PAWN, 0);
    test_pieces!(empty_b_pawn, EMPTY_POS, W_PAWN, 0);
    test_pieces!(empty_b_rook, EMPTY_POS, W_PAWN, 0);
    test_pieces!(empty_b_knight, EMPTY_POS, W_PAWN, 0);
    test_pieces!(empty_b_bishop, EMPTY_POS, W_PAWN, 0);
    test_pieces!(empty_b_queen, EMPTY_POS, W_PAWN, 0);
    test_pieces!(empty_b_king, EMPTY_POS, W_PAWN, 0);
    test_pieces!(empty_b_pieces, EMPTY_POS, W_PAWN, 0);

    // 'complex_pos' positions are found here: https://www.chessprogramming.org/Perft_Results
    const COMPLEX_POS_2: &str =
        "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1";
    const COMPLEX_POS_2_B: &str =
        "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R b KQkq - 0 1";

    test_pieces!(complex2_w_pawn, COMPLEX_POS_2, W_PAWN, 0x81000E700);
    test_pieces!(complex2_w_rook, COMPLEX_POS_2, W_ROOK, 0x81);
    test_pieces!(complex2_w_knight, COMPLEX_POS_2, W_KNIGHT, 0x1000040000);
    test_pieces!(complex2_w_bishop, COMPLEX_POS_2, W_BISHOP, 0x1800);
    test_pieces!(complex2_w_queen, COMPLEX_POS_2, W_QUEEN, 0x200000);
    test_pieces!(complex2_w_king, COMPLEX_POS_2, W_KING, 0x10);
    test_pieces!(complex2_w_pieces, COMPLEX_POS_2, W_PIECES, 0x181024FF91);
    test_pieces!(complex2_b_pawn, COMPLEX_POS_2, B_PAWN, 0x2D500002800000);
    test_pieces!(complex2_b_rook, COMPLEX_POS_2, B_ROOK, 0x8100000000000000);
    test_pieces!(complex2_b_knight, COMPLEX_POS_2, B_KNIGHT, 0x220000000000);
    test_pieces!(complex2_b_bishop, COMPLEX_POS_2, B_BISHOP, 0x40010000000000);
    test_pieces!(complex2_b_queen, COMPLEX_POS_2, B_QUEEN, 0x0010000000000000);
    test_pieces!(complex2_b_king, COMPLEX_POS_2, B_KING, 0x1000000000000000);
    test_pieces!(
        complex2_b_pieces,
        COMPLEX_POS_2,
        B_PIECES,
        0x917D730002800000
    );

    const COMPLEX_POS_3: &str = "8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1";
    const COMPLEX_POS_3_B: &str = "8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 b - - 0 1";

    test_pieces!(complex3_w_pawn, COMPLEX_POS_3, W_PAWN, 0x200005000);
    test_pieces!(complex3_w_rook, COMPLEX_POS_3, W_ROOK, 0x2000000);
    test_pieces!(complex3_w_knight, COMPLEX_POS_3, W_KNIGHT, 0);
    test_pieces!(complex3_w_bishop, COMPLEX_POS_3, W_BISHOP, 0);
    test_pieces!(complex3_w_queen, COMPLEX_POS_3, W_QUEEN, 0);
    test_pieces!(complex3_w_king, COMPLEX_POS_3, W_KING, 0x100000000);
    test_pieces!(complex3_w_pieces, COMPLEX_POS_3, W_PIECES, 0x302005000);
    test_pieces!(complex3_b_pawn, COMPLEX_POS_3, B_PAWN, 0x4080020000000);
    test_pieces!(complex3_b_rook, COMPLEX_POS_3, B_ROOK, 0x8000000000);
    test_pieces!(complex3_b_knight, COMPLEX_POS_3, B_KNIGHT, 0);
    test_pieces!(complex3_b_bishop, COMPLEX_POS_3, B_BISHOP, 0);
    test_pieces!(complex3_b_queen, COMPLEX_POS_3, B_QUEEN, 0);
    test_pieces!(complex3_b_king, COMPLEX_POS_3, B_KING, 0x80000000);
    test_pieces!(complex3_b_pieces, COMPLEX_POS_3, B_PIECES, 0x40880A0000000);

    const COMPLEX_POS_4: &str = "r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1";
    const COMPLEX_POS_4_B: &str =
        "r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 b kq - 0 1";

    test_pieces!(complex4_w_pawn, COMPLEX_POS_4, W_PAWN, 0x100021400C900);
    test_pieces!(complex4_w_rook, COMPLEX_POS_4, W_ROOK, 0x21);
    test_pieces!(complex4_w_knight, COMPLEX_POS_4, W_KNIGHT, 0x800000200000);
    test_pieces!(complex4_w_bishop, COMPLEX_POS_4, W_BISHOP, 0x3000000);
    test_pieces!(complex4_w_queen, COMPLEX_POS_4, W_QUEEN, 0x8);
    test_pieces!(complex4_w_king, COMPLEX_POS_4, W_KING, 0x40);
    test_pieces!(complex4_w_pieces, COMPLEX_POS_4, W_PIECES, 0x180021720C969);
    test_pieces!(complex4_b_pawn, COMPLEX_POS_4, B_PAWN, 0xEE000000000200);
    test_pieces!(complex4_b_rook, COMPLEX_POS_4, B_ROOK, 0x8100000000000000);
    test_pieces!(complex4_b_knight, COMPLEX_POS_4, B_KNIGHT, 0x200100000000);
    test_pieces!(complex4_b_bishop, COMPLEX_POS_4, B_BISHOP, 0x420000000000);
    test_pieces!(complex4_b_queen, COMPLEX_POS_4, B_QUEEN, 0x10000);
    test_pieces!(complex4_b_king, COMPLEX_POS_4, B_KING, 0x1000000000000000);
    test_pieces!(
        complex4_b_pieces,
        COMPLEX_POS_4,
        B_PIECES,
        0x91EE620100010200
    );

    const COMPLEX_POS_5: &str = "rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8";
    const COMPLEX_POS_5_B: &str = "rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R b KQ - 1 8";

    test_pieces!(complex5_w_pawn, COMPLEX_POS_5, W_PAWN, 0x800000000C700);
    test_pieces!(complex5_w_rook, COMPLEX_POS_5, W_ROOK, 0x81);
    test_pieces!(complex5_w_knight, COMPLEX_POS_5, W_KNIGHT, 0x1002);
    test_pieces!(complex5_w_bishop, COMPLEX_POS_5, W_BISHOP, 0x4000004);
    test_pieces!(complex5_w_queen, COMPLEX_POS_5, W_QUEEN, 0x8);
    test_pieces!(complex5_w_king, COMPLEX_POS_5, W_KING, 0x10);
    test_pieces!(complex5_w_pieces, COMPLEX_POS_5, W_PIECES, 0x800000400D79F);
    test_pieces!(complex5_b_pawn, COMPLEX_POS_5, B_PAWN, 0xE3040000000000);
    test_pieces!(complex5_b_rook, COMPLEX_POS_5, B_ROOK, 0x8100000000000000);
    test_pieces!(
        complex5_b_knight,
        COMPLEX_POS_5,
        B_KNIGHT,
        0x200000000002000
    );
    test_pieces!(
        complex5_b_bishop,
        COMPLEX_POS_5,
        B_BISHOP,
        0x410000000000000
    );
    test_pieces!(complex5_b_queen, COMPLEX_POS_5, B_QUEEN, 0x800000000000000);
    test_pieces!(complex5_b_king, COMPLEX_POS_5, B_KING, 0x2000000000000000);
    test_pieces!(
        complex5_b_pieces,
        COMPLEX_POS_5,
        B_PIECES,
        0xAFF3040000002000
    );

    const COMPLEX_POS_6: &str =
        "r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10";
    const COMPLEX_POS_6_B: &str =
        "r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 b - - 0 10";

    test_pieces!(complex6_w_pawn, COMPLEX_POS_6, W_PAWN, 0x1009E600);
    test_pieces!(complex6_w_rook, COMPLEX_POS_6, W_ROOK, 0x21);
    test_pieces!(complex6_w_knight, COMPLEX_POS_6, W_KNIGHT, 0x240000);
    test_pieces!(complex6_w_bishop, COMPLEX_POS_6, W_BISHOP, 0x4004000000);
    test_pieces!(complex6_w_queen, COMPLEX_POS_6, W_QUEEN, 0x1000);
    test_pieces!(complex6_w_king, COMPLEX_POS_6, W_KING, 0x40);
    test_pieces!(complex6_w_pieces, COMPLEX_POS_6, W_PIECES, 0x40142DF661);
    test_pieces!(complex6_b_pawn, COMPLEX_POS_6, B_PAWN, 0xE6091000000000);
    test_pieces!(complex6_b_rook, COMPLEX_POS_6, B_ROOK, 0x2100000000000000);
    test_pieces!(complex6_b_knight, COMPLEX_POS_6, B_KNIGHT, 0x240000000000);
    test_pieces!(complex6_b_bishop, COMPLEX_POS_6, B_BISHOP, 0x440000000);
    test_pieces!(complex6_b_queen, COMPLEX_POS_6, B_QUEEN, 0x10000000000000);
    test_pieces!(complex6_b_king, COMPLEX_POS_6, B_KING, 0x4000000000000000);
    test_pieces!(
        complex6_b_pieces,
        COMPLEX_POS_6,
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

    test_castle!(castling_0_w_king, "K", w_king_castle, true);
    test_castle!(caslting_0_w_queen, "K", w_queen_castle, false);
    test_castle!(caslting_0_b_king, "K", b_king_castle, false);
    test_castle!(caslting_0_b_queen, "K", b_queen_castle, false);

    test_castle!(castling_1_w_king, "k", w_king_castle, false);
    test_castle!(castling_1_w_queen, "k", w_queen_castle, false);
    test_castle!(castling_1_b_king, "k", b_king_castle, true);
    test_castle!(castling_1_b_queen, "k", b_queen_castle, false);

    test_castle!(castling_2_w_king, "Q", w_king_castle, false);
    test_castle!(castling_2_w_queen, "Q", w_queen_castle, true);
    test_castle!(castling_2_b_king, "Q", b_king_castle, false);
    test_castle!(castling_2_b_queen, "Q", b_queen_castle, false);

    test_castle!(castling_3_w_king, "q", w_king_castle, false);
    test_castle!(castling_3_w_queen, "q", w_queen_castle, false);
    test_castle!(castling_3_b_king, "q", b_king_castle, false);
    test_castle!(castling_3_b_queen, "q", b_queen_castle, true);

    test_castle!(castling_4_w_king, "KQ", w_king_castle, true);
    test_castle!(caslting_4_w_queen, "KQ", w_queen_castle, true);
    test_castle!(caslting_4_b_king, "KQ", b_king_castle, false);
    test_castle!(caslting_4_b_queen, "KQ", b_queen_castle, false);

    test_castle!(castling_5_w_king, "Kk", w_king_castle, true);
    test_castle!(castling_5_w_queen, "Kk", w_queen_castle, false);
    test_castle!(castling_5_b_king, "Kk", b_king_castle, true);
    test_castle!(castling_5_b_queen, "Kk", b_queen_castle, false);

    test_castle!(castling_6_w_king, "Kq", w_king_castle, true);
    test_castle!(castling_6_w_queen, "Kq", w_queen_castle, false);
    test_castle!(castling_6_b_king, "Kq", b_king_castle, false);
    test_castle!(castling_6_b_queen, "Kq", b_queen_castle, true);

    test_castle!(castling_7_w_king, "Qk", w_king_castle, false);
    test_castle!(castling_7_w_queen, "Qk", w_queen_castle, true);
    test_castle!(castling_7_b_king, "Qk", b_king_castle, true);
    test_castle!(castling_7_b_queen, "Qk", b_queen_castle, false);

    test_castle!(castling_8_w_king, "Qq", w_king_castle, false);
    test_castle!(castling_8_w_queen, "Qq", w_queen_castle, true);
    test_castle!(castling_8_b_king, "Qq", b_king_castle, false);
    test_castle!(castling_8_b_queen, "Qq", b_queen_castle, true);

    test_castle!(castling_9_w_king, "qk", w_king_castle, false);
    test_castle!(castling_9_w_queen, "qk", w_queen_castle, false);
    test_castle!(castling_9_b_king, "qk", b_king_castle, true);
    test_castle!(castling_9_b_queen, "qk", b_queen_castle, true);

    test_castle!(castling_10_w_king, "KQk", w_king_castle, true);
    test_castle!(castling_10_w_queen, "KQk", w_queen_castle, true);
    test_castle!(castling_10_b_king, "KQk", b_king_castle, true);
    test_castle!(castling_10_b_queen, "KQk", b_queen_castle, false);

    test_castle!(castling_11_w_king, "KQq", w_king_castle, true);
    test_castle!(castling_11_w_queen, "KQq", w_queen_castle, true);
    test_castle!(castling_11_b_king, "KQq", b_king_castle, false);
    test_castle!(castling_11_b_queen, "KQq", b_queen_castle, true);

    test_castle!(castling_12_w_king, "Kkq", w_king_castle, true);
    test_castle!(castling_12_w_queen, "Kkq", w_queen_castle, false);
    test_castle!(castling_12_b_king, "Kkq", b_king_castle, true);
    test_castle!(castling_12_b_queen, "Kkq", b_queen_castle, true);

    test_castle!(castling_13_w_king, "Qkq", w_king_castle, false);
    test_castle!(castling_13_w_queen, "Qkq", w_queen_castle, true);
    test_castle!(castling_13_b_king, "Qkq", b_king_castle, true);
    test_castle!(castling_13_b_queen, "Qkq", b_queen_castle, true);

    test_castle!(castling_14_w_king, "KQkq", w_king_castle, true);
    test_castle!(castling_14_w_queen, "KQkq", w_queen_castle, true);
    test_castle!(castling_14_b_king, "KQkq", b_king_castle, true);
    test_castle!(castling_14_b_queen, "KQkq", b_queen_castle, true);

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
                let mov = str_to_move($move, starting_position);
                starting_position.play_move(mov);
                assert_eq!(starting_position, expected_position);
            }
        };
    }

    // Basic movement tests
    const STARTPOS_B: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR b KQkq - 0 1";

    test_play_move!(
        play_startpos_a2a3,
        STARTPOS,
        "a2a3",
        "rnbqkbnr/pppppppp/8/8/8/P7/1PPPPPPP/RNBQKBNR b KQkq - 0 1"
    ); // startpos w_pawn advance 1
    test_play_move!(
        play_startpos_a2a4,
        STARTPOS,
        "a2a4",
        "rnbqkbnr/pppppppp/8/8/P7/8/1PPPPPPP/RNBQKBNR b KQkq a3 0 1"
    ); // startpos w_pawn advance 2
    test_play_move!(
        play_startpos_a7a6,
        STARTPOS_B,
        "a7a6",
        "rnbqkbnr/1ppppppp/p7/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 2"
    ); // startpos b_pawn advance 1
    test_play_move!(
        play_startpos_a7a5,
        STARTPOS_B,
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
        STARTPOS,
        "b1c3",
        "rnbqkbnr/pppppppp/8/8/8/2N5/PPPPPPPP/R1BQKBNR b KQkq - 1 1"
    ); // w_knight initial move
    test_play_move!(
        play_startpos_b8c6,
        STARTPOS_B,
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
        "rnbqkbnr/pppppppp/8/5K2/8/8/PPPPPPPP/RNBQ1BNR b kq - 1 1"
    ); // w_king move
    test_play_move!(
        play_b_king_move,
        "rnbq1bnr/pppppppp/8/4k3/8/8/PPPPPPPP/RNBQKBNR b KQkq - 0 1",
        "e5f4",
        "rnbq1bnr/pppppppp/8/8/5k2/8/PPPPPPPP/RNBQKBNR w KQ - 1 2"
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
        "rnbq1bnr/pppppppp/8/4K3/8/8/PPPPPPPP/RNBQ1BNR b kq - 0 1"
    );
    test_play_move!(
        play_basic_b_king_capture,
        "rnbq1bnr/pppppppp/8/4k3/4K3/8/PPPPPPPP/RNBQ1BNR b KQkq - 0 1",
        "e5e4",
        "rnbq1bnr/pppppppp/8/8/4k3/8/PPPPPPPP/RNBQ1BNR w KQ - 0 2"
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
        "r4rk1/pppppppp/8/8/8/8/PPPPPPPP/R3K2R w KQ - 1 2"
    );
    test_play_move!(
        play_castle_b_queenside,
        "r3k2r/pppppppp/8/8/8/8/PPPPPPPP/R3K2R b KQkq - 0 1",
        "e8c8",
        "2kr3r/pppppppp/8/8/8/8/PPPPPPPP/R3K2R w KQ - 1 2"
    );
    test_play_move!(
        play_w_pawn_q_promotion,
        "rnbqkbnr/pPpppppp/8/8/8/8/P1PPPPPP/RNBQKBNR w - - 0 1",
        "b7c8Q",
        "rnQqkbnr/p1pppppp/8/8/8/8/P1PPPPPP/RNBQKBNR b - - 0 1"
    );
    test_play_move!(
        play_w_pawn_r_promotion,
        "rnbqkbnr/pPpppppp/8/8/8/8/P1PPPPPP/RNBQKBNR w - - 0 1",
        "b7c8R",
        "rnRqkbnr/p1pppppp/8/8/8/8/P1PPPPPP/RNBQKBNR b - - 0 1"
    );
    test_play_move!(
        play_w_pawn_n_promotion,
        "rnbqkbnr/pPpppppp/8/8/8/8/P1PPPPPP/RNBQKBNR w - - 0 1",
        "b7c8N",
        "rnNqkbnr/p1pppppp/8/8/8/8/P1PPPPPP/RNBQKBNR b - - 0 1"
    );
    test_play_move!(
        play_w_pawn_b_promotion,
        "rnbqkbnr/pPpppppp/8/8/8/8/P1PPPPPP/RNBQKBNR w - - 0 1",
        "b7c8B",
        "rnBqkbnr/p1pppppp/8/8/8/8/P1PPPPPP/RNBQKBNR b - - 0 1"
    );

    test_play_move!(
        play_b_pawn_q_promotion,
        "rnbqkbnr/p1pppppp/8/8/8/8/PpPPPPPP/RNBQKBNR b - - 0 1",
        "b2c1q",
        "rnbqkbnr/p1pppppp/8/8/8/8/P1PPPPPP/RNqQKBNR w - - 0 2"
    );
    test_play_move!(
        play_b_pawn_r_promotion,
        "rnbqkbnr/p1pppppp/8/8/8/8/PpPPPPPP/RNBQKBNR b - - 0 1",
        "b2c1r",
        "rnbqkbnr/p1pppppp/8/8/8/8/P1PPPPPP/RNrQKBNR w - - 0 2"
    );
    test_play_move!(
        play_b_pawn_n_promotion,
        "rnbqkbnr/p1pppppp/8/8/8/8/PpPPPPPP/RNBQKBNR b - - 0 1",
        "b2c1n",
        "rnbqkbnr/p1pppppp/8/8/8/8/P1PPPPPP/RNnQKBNR w - - 0 2"
    );
    test_play_move!(
        play_b_pawn_b_promotion,
        "rnbqkbnr/p1pppppp/8/8/8/8/PpPPPPPP/RNBQKBNR b - - 0 1",
        "b2c1b",
        "rnbqkbnr/p1pppppp/8/8/8/8/P1PPPPPP/RNbQKBNR w - - 0 2"
    );

    // Position::play_move() testing
    macro_rules! test_generate_leapers {
        ($test_name:ident, $starting_position:expr, $expected:expr) => {
            #[test]
            fn $test_name() {
                let starting_position = Position::from($starting_position);
                let mut actual = starting_position.generate_knight_moves();
                actual.sort();
                let mut expected = $expected;
                expected.sort();
                assert_eq!(actual, expected);
            }
        };
    }

    macro_rules! leaper_move {
        ($origin_sq: expr, $dest_sq: expr) => {
            ($origin_sq | ($dest_sq << DEST_BITS_OFFSET))
        };
    }

    test_generate_leapers!(
        startpos_leaper_moves,
        STARTPOS,
        vec![
            leaper_move!(1, 16),
            leaper_move!(1, 18),
            leaper_move!(6, 21),
            leaper_move!(6, 23),
        ]
    );

    test_generate_leapers!(
        startpos_b_leaper_moves,
        STARTPOS_B,
        vec![
            leaper_move!(57, 40),
            leaper_move!(57, 42),
            leaper_move!(62, 45),
            leaper_move!(62, 47),
        ]
    );

    test_generate_leapers!(
        complex_2_leaper_moves,
        COMPLEX_POS_2,
        vec![
            leaper_move!(18, 1),
            leaper_move!(18, 3),
            leaper_move!(18, 24),
            leaper_move!(18, 33),
            leaper_move!(36, 19),
            leaper_move!(36, 26),
            leaper_move!(36, 30),
            leaper_move!(36, 42),
            leaper_move!(36, 46),
            leaper_move!(36, 51),
            leaper_move!(36, 53),
        ]
    );

    test_generate_leapers!(
        complex_2_leaper_moves_b,
        COMPLEX_POS_2_B,
        vec![
            leaper_move!(41, 24),
            leaper_move!(41, 26),
            leaper_move!(41, 35),
            leaper_move!(41, 58),
            leaper_move!(45, 28),
            leaper_move!(45, 30),
            leaper_move!(45, 35),
            leaper_move!(45, 39),
            leaper_move!(45, 55),
            leaper_move!(45, 62),
        ]
    );

    test_generate_leapers!(
        complex_4_leaper_moves,
        COMPLEX_POS_4,
        vec![
            leaper_move!(21, 4),
            leaper_move!(21, 27),
            leaper_move!(21, 31),
            leaper_move!(21, 36),
            leaper_move!(21, 38),
            leaper_move!(47, 30),
            leaper_move!(47, 37),
            leaper_move!(47, 53),
            leaper_move!(47, 62),
        ]
    );
    test_generate_leapers!(
        complex_4_leaper_moves_b,
        COMPLEX_POS_4_B,
        vec![
            leaper_move!(32, 17),
            leaper_move!(32, 26),
            leaper_move!(32, 42),
            leaper_move!(45, 28),
            leaper_move!(45, 30),
            leaper_move!(45, 35),
            leaper_move!(45, 39),
            leaper_move!(45, 62),
        ]
    );

    test_generate_leapers!(
        complex_5_leaper_moves,
        COMPLEX_POS_5,
        vec![
            leaper_move!(1, 11),
            leaper_move!(1, 16),
            leaper_move!(1, 18),
            leaper_move!(12, 6),
            leaper_move!(12, 18),
            leaper_move!(12, 22),
            leaper_move!(12, 27),
            leaper_move!(12, 29),
        ]
    );
    test_generate_leapers!(
        complex_5_leaper_moves_b,
        COMPLEX_POS_5_B,
        vec![
            leaper_move!(13, 3),
            leaper_move!(13, 7),
            leaper_move!(13, 19),
            leaper_move!(13, 23),
            leaper_move!(13, 28),
            leaper_move!(13, 30),
            leaper_move!(57, 40),
            leaper_move!(57, 51),
        ]
    );

    test_generate_leapers!(
        complex_6_leaper_moves,
        COMPLEX_POS_6,
        vec![
            leaper_move!(18, 1),
            leaper_move!(18, 3),
            leaper_move!(18, 8),
            leaper_move!(18, 24),
            leaper_move!(18, 33),
            leaper_move!(18, 35),
            leaper_move!(21, 4),
            leaper_move!(21, 11),
            leaper_move!(21, 27),
            leaper_move!(21, 31),
            leaper_move!(21, 36),
        ]
    );

    test_generate_leapers!(
        complex_6_leaper_moves_b,
        COMPLEX_POS_6_B,
        vec![
            leaper_move!(42, 25),
            leaper_move!(42, 27),
            leaper_move!(42, 32),
            leaper_move!(42, 48),
            leaper_move!(42, 57),
            leaper_move!(42, 59),
            leaper_move!(45, 28),
            leaper_move!(45, 35),
            leaper_move!(45, 39),
            leaper_move!(45, 51),
            leaper_move!(45, 60),
        ]
    );

    test_generate_leapers!(
        leaper_moves_queen_pins_knight,
        "4k3/8/8/4q3/8/4N3/8/4K3 w KQkq - 0 1",
        vec![]
    );

    test_generate_leapers!(
        leaper_moves_rook_pins_knight,
        "4k3/8/8/4r3/8/4N3/8/4K3 w KQkq - 0 1",
        vec![]
    );

    test_generate_leapers!(
        leaper_moves_bishop_pins_knight,
        "4k3/8/8/b7/8/8/3N4/4K3 w KQkq - 0 1",
        vec![]
    );
}
