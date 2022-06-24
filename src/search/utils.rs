use std::sync::atomic::{self, AtomicU32};

use atomic::Ordering;
use chess::{ChessMove, Color, Board, BoardStatus};

pub fn flip_color(input_color: Color) -> Color {
    if input_color == Color::White {
        Color::Black
    } else {
        Color::White
    }
}

pub fn dump_top_moves(moves: &Vec<ChessMove>) -> Vec<String> {
    let mut output: Vec<String> = vec![];

    for chess_move in moves {
        output.push(chess_move.to_string());
    }

    output
}

pub fn fast_board_status(board: Board) -> BoardStatus {
    // If king is checked
    // Can the king move
    // Can pieces block
    // Can checkers be taken
    // 2+ checkers + no king move = checkmate
    // 1 checker, no block, can't be taken, no king move = checkmate


    // Stalemate
    // If king and pinned pieces cannot move
    BoardStatus::Ongoing
}

// Stolen shamelessly from https://github.com/rust-lang/rust/issues/72353 because
// there is no native atomic f64 support
#[derive(Debug)]
pub struct AtomicF32 {
    storage: AtomicU32,
}
impl AtomicF32 {
    pub fn new(value: f32) -> Self {
        let as_u32 = value.to_bits();
        Self {
            storage: AtomicU32::new(as_u32),
        }
    }
    pub fn store(&self, value: f32, ordering: Ordering) {
        let as_u32 = value.to_bits();
        self.storage.store(as_u32, ordering)
    }
    pub fn load(&self, ordering: Ordering) -> f32 {
        let as_u32 = self.storage.load(ordering);
        f32::from_bits(as_u32)
    }
}
impl Clone for AtomicF32 {
    fn clone(&self) -> AtomicF32 {
        Self::new(self.load(Ordering::Relaxed))
    }
}
