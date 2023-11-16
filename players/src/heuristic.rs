use std::fmt::Debug;
use std::marker::PhantomData;

use baz_core::*;

pub trait Heuristic<T>
where
    T: Clone + Ord,
{
    fn evaluate(&mut self, board: &Board, color: &Color) -> T;
}

pub struct HeuristicPlayer<H, T>
where
    H: Heuristic<T>,
    T: Clone + Debug + Ord,
{
    heuristic: H,
    depth: usize,
    phantom: PhantomData<T>,
}

impl<H, T> HeuristicPlayer<H, T>
where
    H: Heuristic<T>,
    T: Ord + Clone + Debug,
{
    pub fn new(heuristic: H, depth: usize) -> HeuristicPlayer<H, T> {
        HeuristicPlayer {
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
    fn minimax(&mut self, board: &Board, color: &Color, depth: usize) -> (Move, T) {
        if depth == 0 {
            let m = self.choose_move(board, color);
            return (m, self.heuristic.evaluate(&board.apply_move(&m), color));
        }
        board
            .legal_moves(color)
            .iter()
            .map(|m1| {
                let new_board = board.apply_move(m1);
                let min = new_board
                    .legal_moves(&color.invert())
                    .iter()
                    .map(|m2| self.minimax(&new_board.apply_move(m2), color, depth - 1))
                    .min_by_key(|(_, h)| h.clone())
                    .unwrap()
                    .1;
                (*m1, min)
            })
            .max_by_key(|(_, h)| h.clone())
            .unwrap()
    }
}

impl<H, T> GamePlayer for HeuristicPlayer<H, T>
where
    H: Heuristic<T>,
    T: Clone + Debug + Ord,
{
    fn decide(&mut self, board: &Board, color: &Color) -> Move {
        self.choose_move(board, color)
        // self.minimax(board, color, self.depth).0
    }
}
