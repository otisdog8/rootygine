#[cfg(test)]
use crate::search;
use chess::{Board, Color};
use std::str::FromStr;
use crate::search::transposition_table::{Flag, TransTable, TransTableEntry};

#[test]
fn vienna_gambit() {
    // Tests the response of engine after e5 from vienna gambit accepted.
    let color_to_move = Color::Black;
    let board = Board::from_str("rnbqkb1r/pppp1ppp/5n2/4P3/5p2/2N5/PPPP2PP/R1BQKBNR b KQkq - 0 4")
        .expect("Invalid FEN");
    let best_move = search::iterative_deepening_search(board, color_to_move, 7, None);
    // Two possible variations, both are correct
    assert!(best_move.to_string() == "f6g8" || best_move.to_string() == "d8e7");
}

#[test]
fn mate1() {
    // Makes sure it doesn't miss mate in ones
    let color_to_move = Color::Black;
    let board =
        Board::from_str("4k3/2np1p2/4p1Pn/2q5/2P4P/5b2/2r2R2/6K1 b - - 0 34").expect("Invalid FEN");
    let best_move = search::iterative_deepening_search(board, color_to_move, 7, None);
    assert!(best_move.to_string() == "c5f2");
}

#[test]
fn backrank2() {
    // Tests the response of engine when faced with backrank checkmate opportunity
    let color_to_move = Color::White;
    let board = Board::from_str("2R2rk1/4pppp/8/8/8/8/6K1/2R5 w - - 0 1").expect("Invalid FEN");
    let best_move = search::iterative_deepening_search(board, color_to_move, 7, None);
    assert!(best_move.to_string() == "c8f8");
}

#[test]
fn morphy2() {
    // Tests the response of engine when faced with backrank checkmate opportunity
    let color_to_move = Color::White;
    let board = Board::from_str("kbK5/pp6/1P6/8/8/8/8/R7 w - -").expect("Invalid FEN");
    let best_move = search::iterative_deepening_search(board, color_to_move, 7, None);
    assert!(best_move.to_string() == "a1a6");
}

#[test]
fn kasperov4() {
    // Tests the response of engine when faced with backrank checkmate opportunity
    let color_to_move = Color::White;
    let board =
        Board::from_str("4k2r/1R3R2/p3p1pp/4b3/1BnNr3/8/P1P5/5K2 w - - 1 0").expect("Invalid FEN");
    let best_move = search::iterative_deepening_search(board, color_to_move, 7, None);
    assert!(best_move.to_string() == "f7e7");
}
