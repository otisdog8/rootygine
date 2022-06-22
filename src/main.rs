mod search;
mod tests;
use chess::{self, BoardStatus, ChessMove};
use chess::{Board, Color};
use log::debug;
use search::transposition_table::{Flag, TransTable, TransTableEntry, self};
use std::io;
use std::str::FromStr;
use std::sync::{Mutex, Arc};

fn main() {
    env_logger::init();
    player_play();
}

fn player_play() {
    let mut board = Board::default();
    let mut tt = Arc::new(Mutex::new(transposition_table::TransTable::new()));
    loop {
        let mut buffer = String::new();
        let stdin = io::stdin(); // We get `Stdin` here.
        stdin.read_line(&mut buffer).unwrap();
        let player_move = ChessMove::from_str(buffer.trim()).unwrap();
        if board.legal(player_move) {
            board = board.make_move_new(player_move);
        } else {
            debug!("Enter an FEN of the position AFTER you moved: ");
            let mut buffer = String::new();
            stdin.read_line(&mut buffer).unwrap();
            match Board::from_str(buffer.trim()) {
                Ok(board_new) => {
                    board = board_new;
                    debug!("Successfully made new board");
                }
                Err(_) => {
                    debug!("{}", buffer);

                    debug!("FAILED at parsing FEN");
                    board = Board::default();
                }
            }
        }

        if board.status() == BoardStatus::Checkmate || board.status() == BoardStatus::Stalemate {
            break;
        }

        let color_to_move = Color::Black;

        let engine_move =
            search::iterative_deepening_search(board, color_to_move, 7,  Some(tt.clone()));
        board = board.make_move_new(engine_move);
        println!("Engine move: {}", engine_move);

        if board.status() == BoardStatus::Checkmate || board.status() == BoardStatus::Stalemate {
            break;
        }
    }
}
/*
fn testing() {
    let color_to_move = Color::Black;
    let board =
        Board::from_str("r1b1k2r/1pQp1ppp/p1n1p3/2b5/2p1PN1q/2N2B2/PPP2P1P/2KR3R b kq - 0 1")
            .expect("Invalid FEN");
    let best_move =
        search::iterative_deepening_search(board, color_to_move, 7, &mut TransTable::new());
    debug!("Test");

    println!("Top Engine Move: {}", best_move);
}
*/