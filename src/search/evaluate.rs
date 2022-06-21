//! The super weird evaluation function. It counts up pieces for each side, then compare number of possible moves.
//! It weights the bishop slightly more heavily than
//! the knight, which is generally true for almost all cases. It used to be 3.5, but I figured
//! that the possible moves decrease from the loss of a bishop may compensate for that. Each additional move would add 0.1
//! The randomness is added so that moves with the same eval can be chosen randomly.

use chess::{ChessMove, Color, MoveGen, BitBoard};
use rand::{prelude::SmallRng, Rng, SeedableRng};

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

    // Piece out of position

    // Center (vertical)
    // Center (horizontal)
    //

    color_eval[0] - color_eval[1]

}
