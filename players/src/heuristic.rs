use std::fmt::Debug;
use std::marker::PhantomData;

use baz_core::*;

pub trait Heuristic<T>
where
    T: Ord,
{
    fn evaluate(&mut self, board: &Board, color: &Color) -> T;
}

pub struct HeuristicPlayer<H, T>
where
    H: Heuristic<T>,
    T: Ord + Debug,
{
    heuristic: H,
    phantom: PhantomData<T>,
}

impl<H, T> HeuristicPlayer<H, T>
where
    H: Heuristic<T>,
    T: Ord + Debug,
{
    pub fn new(heuristic: H) -> HeuristicPlayer<H, T> {
        HeuristicPlayer {
            heuristic,
            phantom: PhantomData,
        }
    }
    fn choose_move(&mut self, board: &Board, color: &Color) -> Move {
        *board
            .legal_moves(color)
            .iter()
            .max_by_key(|m| self.heuristic.evaluate(&board.apply_move(m), color))
            .unwrap()
    }
}

impl<H, T> GamePlayer for HeuristicPlayer<H, T>
where
    H: Heuristic<T>,
    T: Ord + Debug,
{
    fn decide(&mut self, board: &Board, color: &Color) -> Move {
        self.choose_move(board, color)
    }
}
