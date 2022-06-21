use std::collections::VecDeque;

use chess::ChessMove;

const THREADS: usize = 4;

pub fn divide_work(possible_moves: &mut VecDeque<ChessMove>) -> Vec<Vec<ChessMove>> {
    // Try to split them up as evenly as possible
    let num_per_thread = possible_moves.len() / THREADS;
    let mut work_out: Vec<Vec<ChessMove>> = vec![];

    // Could use improvements in the future, since the first thread would evaluate much faster
    // than the second and third threads due to move ordering improvements.
    for _ in 0..THREADS - 2 {
        let mut thread_work = vec![];
        for _ in 0..num_per_thread - 1 {
            thread_work.push(
                possible_moves
                    .pop_front()
                    .expect("Error in divide work function."),
            );
        }
        work_out.push(thread_work);
    }

    // This accounts for uneven divisions
    let mut thread_work = vec![];
    while !possible_moves.is_empty() {
        thread_work.push(
            possible_moves
                .pop_front()
                .expect("Error in divide work function."),
        );
    }
    work_out.push(thread_work);

    work_out
}
