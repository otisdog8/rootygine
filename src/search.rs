use std::cmp::Ordering;

use chess::{Board, ChessMove, Color, MoveGen};
use log::debug;
use std::collections::VecDeque;

use transposition_table::{Flag, TransTable, TransTableEntry};

use self::evaluate::evaluate;

mod evaluate;
mod threading;
pub mod transposition_table;
mod utils;

#[derive(Debug)]
struct MoveEval {
    chess_move: ChessMove,
    eval: f32,
}

impl PartialOrd for MoveEval {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for MoveEval {}

impl PartialEq for MoveEval {
    fn eq(&self, other: &Self) -> bool {
        self.chess_move == other.chess_move && self.eval == other.eval
    }
}

impl Ord for MoveEval {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.eval > other.eval {
            Ordering::Greater
        } else if self.eval < other.eval {
            Ordering::Less
        } else {
            Ordering::Equal
        }
    }
}



// Uses iterative deepening technique and transposition tables to optimize faster search
pub fn iterative_deepening_search(
    board: Board,
    color_to_move: Color,
    target_depth: i32,
    tt: &mut TransTable,
) -> ChessMove {
    // start with depth 4
    let mut depth = 4;
    let mut tt = transposition_table::TransTable::new();
    let mut possible_moves: VecDeque<ChessMove> = MoveGen::new_legal(&board).collect();

    while depth < target_depth + 1 {
        // the best moves from the last iteration are searched first to improve alpha-beta pruning performance
        debug!("Evaluating with depth {}", depth);
        // Need to rethink... this may result in two copies of the transposition table at once
        let mut scores = negamax_root(board, color_to_move, depth, possible_moves, &mut tt);

        // Stop if you found checkmate
        if scores[0].eval == 10000.0 {
            return scores[0].chess_move;
        }

        let mut best_moves: VecDeque<ChessMove> = VecDeque::new();
        scores.reverse();
        for score in scores {
            debug!("{}, {}", score.chess_move, score.eval);
            best_moves.push_back(score.chess_move);
        }

        possible_moves = best_moves;


        depth += 1;
    }

    *possible_moves
        .get(possible_moves.len()-1)
        .expect("This is imepossible. There should be at least one possible move.")
}

fn negamax_root(
    board: Board,
    color_to_move: Color,
    max_depth: i32,
    mut moves: VecDeque<ChessMove>,
    tt: &mut TransTable,
) -> Vec<MoveEval> {
    // Returns moves in best to worst order
    let mut combined_evals: Vec<MoveEval> = vec![];
    let num_possible_moves = moves.len();
    // Should probably switch out for a custom implementation
    let alpha = -f32::INFINITY;
    let beta = f32::INFINITY;

    // let work = threading::divide_work(&mut moves);

    /*for thread_work in work {
        let mut thread_local_tt = tt.clone();
        let mut thread_local_alpha = alpha.clone();

        let thread = thread::spawn(move || {
    */
    let mut scores: Vec<MoveEval> = vec![];

    for (i, possible_move) in moves.iter().enumerate() {
        debug!("Evaluating {}/{} moves", i, num_possible_moves);

        let new_board = board.make_move_new(*possible_move);

        // Check if it's a terminal node
        if new_board.status() == chess::BoardStatus::Checkmate {
            // Return 10000 and +/- for how close to checkmate it is
            let score = MoveEval {
                chess_move: *possible_move,
                eval: 10000.0,
            };
            combined_evals.push(score);
            break;
        } else if new_board.status() == chess::BoardStatus::Stalemate {
            let score = MoveEval {
                chess_move: *possible_move,
                eval: -1000.0,
            };
            combined_evals.push(score);
        } else {
            let evaluation = -negamax(
                new_board,
                max_depth,
                max_depth - 1,
                -beta,
                -alpha,
                utils::flip_color(color_to_move),
                tt,
            );

            let score = MoveEval {
                chess_move: *possible_move,
                eval: evaluation,
            };

            combined_evals.push(score);

            let alpha = f32::max(alpha, evaluation);

            if alpha >= beta {
                break;
            }
        }
    }
    /*
    for handle in handles {
        let mut output = handle.join().unwrap();
        combined_evals.append(&mut output);
    }
    */

    // Sort from best to worst
    combined_evals.sort_by(|a, b| b.cmp(a));

    combined_evals
}

fn negamax(
    current_board: chess::Board,
    max_depth: i32,
    current_depth: i32,
    mut alpha: f32,
    mut beta: f32,
    color: chess::Color,
    tt: &mut TransTable,
) -> f32 {
    let alpha_original = alpha;
    let current_board_status = current_board.status();

    // Check if it's a terminal node
    if current_board_status == chess::BoardStatus::Checkmate {
        // Return 10000 and +/- for how close to checkmate it is
        if current_board.side_to_move() == chess::Color::White {
            return (10000 - current_depth) as f32;
        } else {
            return (-10000 + current_depth) as f32;
        }
    } else if current_board_status == chess::BoardStatus::Stalemate {
        // Avoid stalemate at all costs but at less cost than checkmate
        if current_board.side_to_move() == chess::Color::White {
            return (-1000 + current_depth) as f32;
        } else {
            return (1000 - current_depth) as f32;
        }
    }

    let tt_entry = tt.tt.get(&current_board.get_hash());

    match tt_entry {
        Some(entry) => {
            if entry.depth >= max_depth {
                if entry.flag == Flag::Exact {
                    return entry.eval;
                } else if entry.flag == Flag::Lowerbound {
                    alpha = f32::max(alpha, entry.eval);
                } else if entry.flag == Flag::Upperbound {
                    beta = f32::min(beta, entry.eval);
                }

                if alpha >= beta {
                    return entry.eval;
                }
            }
        }
        None => {}
    }

    // Negamax algorithm requires that evaluations be returned relative to the side being evaluated
    if current_depth == 0 {
        if color == chess::Color::White {
            return evaluate::evaluate(current_board);
        } else {
            return -evaluate::evaluate(current_board);
        }
    }

    // Eventually use algorithm to sort them by potential to save time
    let mut possible_moves = MoveGen::new_legal(&current_board);
    let mut move_eval_arr = vec![];
    for possible_move in possible_moves {
        let mut new_board = Board::default();
        current_board.make_move(possible_move, &mut new_board);
        let tt_entry = tt.tt.get(&new_board.get_hash());

        let eval = match tt_entry {
            Some(entry) => entry.eval,
            None => evaluate(new_board),
        };
        move_eval_arr.push(MoveEval {chess_move: possible_move, eval});
    }
    move_eval_arr.sort_by(|a, b| b.cmp(a));
    move_eval_arr.reverse();


    let mut value = -f32::INFINITY;

    for move_eval in move_eval_arr {
        value = f32::max(
            value,
            -negamax(
                current_board.make_move_new(move_eval.chess_move),
                max_depth,
                current_depth - 1,
                -beta,
                -alpha,
                utils::flip_color(color),
                tt,
            ),
        );

        alpha = f32::max(alpha, value);

        if alpha >= beta {
            break;
        }
    }

    // Don't write checkmates into the transposition table
    if !(-9000.0..=9000.0).contains(&value) {
        let flag: Flag;
        if value <= alpha_original {
            flag = Flag::Upperbound;
        } else if value >= beta {
            flag = Flag::Lowerbound;
        } else {
            flag = Flag::Exact;
        }

        let tt_entry = TransTableEntry {
            depth: max_depth - current_depth + 1,
            flag,
            eval: value,
        };

        tt.add_entry(current_board, tt_entry);
    }

    value
}
