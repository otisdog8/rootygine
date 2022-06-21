mod search;
mod tests;
use chess::{self, BoardStatus, ChessMove};
use chess::{Board, Color};
use log::debug;
use std::io;
use std::str::FromStr;
use search::transposition_table::{Flag, TransTable, TransTableEntry};

fn main() {
    env_logger::init();
    player_play();
}

fn player_play() {
    let mut board = Board::default();
    let mut transposition_table: TransTable = TransTable::new();
    loop {
        let mut buffer = String::new();
        let stdin = io::stdin(); // We get `Stdin` here.
        stdin.read_line(&mut buffer).unwrap();
        let player_move = ChessMove::from_str(buffer.trim()).unwrap();
        board = board.make_move_new(player_move);

        if board.status() == BoardStatus::Checkmate || board.status() == BoardStatus::Stalemate {
            break;
        }

        let color_to_move = Color::Black;

        let engine_move = search::iterative_deepening_search(board, color_to_move, 7, &mut transposition_table);
        board = board.make_move_new(engine_move);
        println!("Engine move: {}", engine_move);

        if board.status() == BoardStatus::Checkmate || board.status() == BoardStatus::Stalemate {
            break;
        }
    }
}

fn testing() {
    let color_to_move = Color::Black;
    let board =
        Board::from_str("r1b1k2r/1pQp1ppp/p1n1p3/2b5/2p1PN1q/2N2B2/PPP2P1P/2KR3R b kq - 0 1")
            .expect("Invalid FEN");
    let best_move = search::iterative_deepening_search(board, color_to_move, 7, &mut TransTable::new());
    debug!("Test");

    println!("Top Engine Move: {}", best_move);
}
