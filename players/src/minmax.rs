use std::fmt::Debug;
use std::marker::PhantomData;

use crate::heuristic::{Heuristic, SymmetricalHeuristic};
use baz_core::*;
use num::traits::Inv;

pub struct MinMaxPlayer<H, T>
where
    H: Heuristic<T> + SymmetricalHeuristic<T>,
    T: Clone + Debug + Inv<Output = T> + Ord,
{
    heuristic: H,
    depth: usize,
    phantom: PhantomData<T>,
}
impl<H, T> MinMaxPlayer<H, T>
where
    H: Heuristic<T> + SymmetricalHeuristic<T>,
    T: Clone + Debug + Inv<Output = T> + Ord,
{
    pub fn new(heuristic: H, depth: usize) -> MinMaxPlayer<H, T> {
        MinMaxPlayer {
            heuristic,
            depth,
            phantom: PhantomData,
        }
    }
    fn choose_move(&mut self, board: &Board, color: &Color) -> Move {
        *board
            .legal_moves(color)
            .iter()
            .max_by_key(|m| self.heuristic.evaluate(&board.apply_move(m), color))
            // TODO why this throw error
            .unwrap()
    }
    fn minimax(&mut self, board: &Board, color: &Color, depth: usize, alpha: T, mut beta: T) -> T {
        if depth == 0 {
            return self.heuristic.evaluate(board, color);
        }
        // println!(
        //     "Calling minimax for {:?} depth {} alpha {:?} beta {:?}",
        //     color, depth, alpha, beta
        // );
        let mut v = H::min();
        let alpha_limit = H::inv(alpha.clone());
        for mov in board.legal_moves(color).iter() {
            let new_board = board.apply_move(mov);
            v = v.max(self.minimax(
                &new_board,
                &color.invert(),
                depth - 1,
                beta.clone(),
                alpha.clone(),
            ));
            if v < alpha_limit {
                break;
            }
            if v > beta {
                beta = v.clone();
            }
        }
        // println!("Returning {:?}", v);
        v
    }
}

impl<H, T> GamePlayer for MinMaxPlayer<H, T>
where
    H: Heuristic<T> + SymmetricalHeuristic<T>,
    T: Clone + Debug + Inv<Output = T> + Ord,
{
    fn decide(&mut self, board: &Board, color: &Color) -> Move {
        self.heuristic.log_estimate(board, color);
        *board
            .legal_moves(color)
            .iter()
            .max_by_key(|m| {
                self.minimax(&board.apply_move(m), color, self.depth, H::min(), H::min())
            })
            // TODO why this throw error
            .unwrap()
    }
}
