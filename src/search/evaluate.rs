//! The super weird evaluation function. It counts up pieces for each side, then compare number of possible moves.
//! It weights the bishop slightly more heavily than
//! the knight, which is generally true for almost all cases. It used to be 3.5, but I figured
//! that the possible moves decrease from the loss of a bishop may compensate for that. Each additional move would add 0.1
//! The randomness is added so that moves with the same eval can be chosen randomly.

use chess::{BitBoard, ChessMove, Color, MoveGen, Piece, Square, ALL_SQUARES, EMPTY};
use rand::{prelude::SmallRng, Rng, SeedableRng};

// This  implements Piece Square Tables (PSQT) for each piece type. The
// PSQT's are written from White's point of view, as if looking at a chess
// diagram, with A1 on the lower left corner.
// Taken from https://github.com/mvanthoor/rustic/blob/master/src/evaluation/psqt.rs

type Psqt = [i8; 64];

#[rustfmt::skip]
const KING_MG: Psqt = [
    0,    0,     0,     0,    0,    0,    0,    0,
    0,    0,     0,     0,    0,    0,    0,    0,
    0,    0,     0,     0,    0,    0,    0,    0,
    0,    0,     0,    20,   20,    0,    0,    0,
    0,    0,     0,    20,   20,    0,    0,    0,
    0,    0,     0,     0,    0,    0,    0,    0,
    0,    0,     0,   -10,  -10,    0,    0,    0,
    0,    0,    30,   -10,  -10,    0,   30,    0,
];

#[rustfmt::skip]
const QUEEN_MG: Psqt = [
    -30,  -20,  -10,  -10,  -10,  -10,  -20,  -30,
    -20,  -10,   -5,   -5,   -5,   -5,  -10,  -20,
    -10,   -5,   10,   10,   10,   10,   -5,  -10,
    -10,   -5,   10,   20,   20,   10,   -5,  -10,
    -10,   -5,   10,   20,   20,   10,   -5,  -10,
    -10,   -5,   -5,   -5,   -5,   -5,   -5,  -10,
    -20,  -10,   -5,   -5,   -5,   -5,  -10,  -20,
    -30,  -20,  -10,  -10,  -10,  -10,  -20,  -30 
];

#[rustfmt::skip]
const ROOK_MG: Psqt = [
    0,   0,   0,   0,   0,   0,   0,   0,
   15,  15,  15,  20,  20,  15,  15,  15,
    0,   0,   0,   0,   0,   0,   0,   0,
    0,   0,   0,   0,   0,   0,   0,   0,
    0,   0,   0,   0,   0,   0,   0,   0,
    0,   0,   0,   0,   0,   0,   0,   0,
    0,   0,   0,   0,   0,   0,   0,   0,
    0,   0,   0,  10,  10,  10,   0,   0
];

#[rustfmt::skip]
const BISHOP_MG: Psqt = [
    -20,    0,    0,    0,    0,    0,    0,  -20,
    -15,    0,    0,    0,    0,    0,    0,  -15,
    -10,    0,    0,    5,    5,    0,    0,  -10,
    -10,   10,   10,   30,   30,   10,   10,  -10,
      5,    5,   10,   25,   25,   10,    5,    5,
      5,    5,    5,   10,   10,    5,    5,    5,
    -10,    5,    5,   10,   10,    5,    5,  -10,
    -20,  -10,  -10,  -10,  -10,  -10,  -10,  -20
];

#[rustfmt::skip]
const KNIGHT_MG: Psqt = [
    -20, -10,  -10,  -10,  -10,  -10,  -10,  -20,
    -10,  -5,   -5,   -5,   -5,   -5,   -5,  -10,
    -10,  -5,   15,   15,   15,   15,   -5,  -10,
    -10,  -5,   15,   15,   15,   15,   -5,  -10,
    -10,  -5,   15,   15,   15,   15,   -5,  -10,
    -10,  -5,   10,   15,   15,   15,   -5,  -10,
    -10,  -5,   -5,   -5,   -5,   -5,   -5,  -10,
    -20,   0,  -10,  -10,  -10,  -10,    0,  -20
];

#[rustfmt::skip]
const PAWN_MG: Psqt = [
     0,   0,   0,   0,   0,   0,   0,   0,
    60,  60,  60,  60,  70,  60,  60,  60,
    40,  40,  40,  50,  60,  40,  40,  40,
    20,  20,  20,  40,  50,  20,  20,  20,
     5,   5,  15,  30,  40,  10,   5,   5,
     5,   5,  10,  20,  30,   5,   5,   5,
     5,   5,   5, -30, -30,   5,   5,   5,
     0,   0,   0,   0,   0,   0,   0,   0
];

#[rustfmt::skip]
pub const FLIP: [usize; 64] = [
    56, 57, 58, 59, 60, 61, 62, 63,
    48, 49, 50, 51, 52, 53, 54, 55,
    40, 41, 42, 43, 44, 45, 46, 47,
    32, 33, 34, 35, 36, 37, 38, 39,
    24, 25, 26, 27, 28, 29, 30, 31,
    16, 17, 18, 19, 20, 21, 22, 23,
     8,  9, 10, 11, 12, 13, 14, 15,
     0,  1,  2,  3,  4,  5,  6,  7,
];

pub fn evaluate(board: chess::Board) -> f32 {
    // In the order white, black
    let mut color_eval: Vec<f32> = vec![];
    // In the order of pawn, knight, bishop, root, queen, king
    let piece_values: Vec<f32> = vec![1.0, 3.0, 3.1, 5.0, 12.0, 100.0];

    for color in chess::ALL_COLORS {
        let color_bitboard = board.color_combined(color);
        let mut color_specific_eval: f32 = 0.0;

        for (i, piece) in chess::ALL_PIECES.iter().enumerate() {
            let piece_bitboard = board.pieces(*piece);
            // Looks for pieces of that type of that color
            let num_of_pieces_of_type = piece_bitboard & color_bitboard;
            color_specific_eval += (num_of_pieces_of_type.popcnt() as f32) * piece_values[i];
        }

        color_eval.push(color_specific_eval);
    }
    let mut placement_black = 0;
    let mut placement_white = 0;
    let white = board.color_combined(Color::White);
    let combined = board.combined();
    let pawns = board.pieces(Piece::Pawn);
    let knights = board.pieces(Piece::Knight);
    let bishops = board.pieces(Piece::Bishop);
    let rooks = board.pieces(Piece::Rook);
    let queens = board.pieces(Piece::Queen);
    // Kings is omitted because it is never used: caught by the if statement
    for i in 0..64 {
        let square_board = BitBoard(1u64 << i);

        if combined & square_board == EMPTY {
            // Do nothing
        } else if white & square_board != EMPTY {
            let ind = FLIP[i];
            // If the square is white
            if (pawns ^ rooks ^ bishops & square_board != EMPTY) {
                if pawns & square_board != EMPTY {
                    placement_white += PAWN_MG[ind];
                } else if rooks & square_board != EMPTY {
                    placement_white += ROOK_MG[ind];
                } else {
                    placement_white += BISHOP_MG[ind];
                }
            } else {
                if knights & square_board != EMPTY {
                    placement_white += KNIGHT_MG[ind];
                } else if queens & square_board != EMPTY {
                    placement_white += QUEEN_MG[ind];
                } else {
                    placement_white += KING_MG[ind];
                }
            }
        } else {
            // If the square is black (if its not empty or white)
            if (pawns ^ rooks ^ bishops & square_board != EMPTY) {
                if pawns & square_board != EMPTY {
                    placement_white += PAWN_MG[i];
                } else if rooks & square_board != EMPTY {
                    placement_white += ROOK_MG[i];
                } else {
                    placement_white += BISHOP_MG[i];
                }
            } else {
                if knights & square_board != EMPTY {
                    placement_white += KNIGHT_MG[i];
                } else if queens & square_board != EMPTY {
                    placement_white += QUEEN_MG[i];
                } else {
                    placement_white += KING_MG[i];
                }
            }
        }
    }

    color_eval[0] - color_eval[1] + placement_white as f32 / 100.0 - placement_black as f32 / 100.0
}
