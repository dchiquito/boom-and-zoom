use std::fmt::Debug;
use std::marker::PhantomData;
use std::time::{Duration, Instant};

use crate::heuristic::Heuristic;
use baz_core::*;

pub struct MinMaxPlayer<H, T>
where
    H: Heuristic<T>,
    T: Clone + Debug + Ord,
{
    heuristic: H,
    iterations: usize,
    max_depth: usize,
    max_width: usize,
    top_alpha: T,
    top_beta: T,
    time_per_turn: Duration,
    timeout: Option<Instant>,
    phantom: PhantomData<T>,
}
impl<H, T> MinMaxPlayer<H, T>
where
    H: Heuristic<T>,
    T: Clone + Debug + Ord,
{
    pub fn new(heuristic: H, time_per_turn: Duration) -> MinMaxPlayer<H, T> {
        MinMaxPlayer {
            heuristic,
            iterations: 0,
            max_depth: 0,
            max_width: 0,
            top_alpha: H::min(),
            top_beta: H::max(),
            time_per_turn,
            timeout: None,
            phantom: PhantomData,
        }
    }
    fn choose_move(&mut self, board: &Board, color: &Color) -> Move {
        board
            .legal_moves(color)
            .max_by_key(|m| self.heuristic.evaluate(&board.apply_move(m), color))
            // TODO why this throw error
            .unwrap()
    }
    fn minimax(
        &mut self,
        board: &Board,
        color: &Color,
        maximizing: bool,
        mut alpha: T,
        mut beta: T,
        depth: usize,
    ) -> T {
        self.iterations += 1;
        if depth >= self.max_depth {
            return self.heuristic.evaluate(board, color);
        }
        if let Some(timeout) = self.timeout {
            if Instant::now() > timeout {
                return self.heuristic.evaluate(board, color);
            }
        }
        // println!("{color:?} max:{maximizing} depth:{depth} alpha:{alpha:?} beta:{beta:?}",);
        let mut scores_and_boards = board
            .legal_moves(color)
            .map(|m| board.apply_move(&m))
            .map(|b| (self.heuristic.evaluate(&b, color), b))
            .collect::<Vec<(T, Board)>>();
        scores_and_boards
            .sort_by(|(h1, _), (h2, _)| if maximizing { h1.cmp(h2) } else { h2.cmp(h1) });
        let mut v = if maximizing { H::min() } else { H::max() };
        for (_, new_board) in scores_and_boards.iter().take(self.max_width) {
            // println!("Checkin out {new_board:#?} {alpha_limit:?} {beta:?}");
            let new_v = self.minimax(
                new_board,
                color,
                !maximizing,
                alpha.clone(),
                beta.clone(),
                depth + 1,
            );
            // println!("Considering {new_v:?}");
            if maximizing {
                v = v.max(new_v);
                if v > beta {
                    break;
                }
                alpha = alpha.max(v.clone());
            } else {
                v = v.min(new_v);
                if v < alpha {
                    break;
                }
                beta = beta.min(v.clone());
            };
        }
        // println!("Returning {:?}", v);
        v
    }
}

impl<H, T> GamePlayer for MinMaxPlayer<H, T>
where
    H: Heuristic<T>,
    T: Clone + Debug + Ord,
{
    fn decide(&mut self, board: &Board, color: &Color) -> Move {
        let now = Instant::now();
        self.timeout = Some(now + self.time_per_turn);
        let mut best_move = (board.legal_moves(color).next().unwrap(), H::min());
        let mut last_best_move = (board.legal_moves(color).next().unwrap(), H::min());
        self.top_alpha = H::min();
        self.top_beta = H::max();
        self.max_depth = 1;
        self.max_width = 6;
        while Instant::now() - now < self.time_per_turn {
            last_best_move = best_move;
            best_move = board
                .legal_moves(color)
                .map(|m| {
                    (
                        m,
                        self.minimax(&board.apply_move(&m), color, true, H::min(), H::max(), 0),
                    )
                })
                .max_by_key(|(_m, score)| score.clone())
                // TODO why this throw error
                .unwrap();
            self.max_depth += 1;
        }
        let time_taken = Instant::now() - now;
        println!(
            "Decided after {time_taken:?}, {} iterations, and a max depth of {}: {:?}",
            self.iterations, self.max_depth, last_best_move
        );
        // Intentionally discard the abortive partially calculated result
        last_best_move.0
    }
}
