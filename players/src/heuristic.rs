use std::cmp::Ordering;
use std::fmt::Debug;
use std::marker::PhantomData;

use baz_core::*;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum HResult<T>
where
    T: Ord + Default,
{
    Win,
    Draw,
    Loss,
    Unknown(T),
}
impl<T> Ord for HResult<T>
where
    T: Ord + Default,
{
    fn cmp(&self, other: &Self) -> Ordering {
        match self {
            HResult::Win => match other {
                HResult::Win => Ordering::Equal,
                _ => Ordering::Greater,
            },
            HResult::Draw => match other {
                HResult::Win => Ordering::Less,
                HResult::Draw => Ordering::Equal,
                HResult::Loss => Ordering::Greater,
                HResult::Unknown(ev) => T::default().cmp(ev),
            },
            HResult::Loss => match other {
                HResult::Loss => Ordering::Equal,
                _ => Ordering::Less,
            },
            HResult::Unknown(ev) => match other {
                HResult::Win => Ordering::Less,
                HResult::Draw => ev.cmp(&T::default()),
                HResult::Loss => Ordering::Greater,
                HResult::Unknown(other_ev) => ev.cmp(other_ev),
            },
        }
    }
}
impl<T> PartialOrd for HResult<T>
where
    T: Ord + Default,
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

pub trait Heuristic<T>
where
    T: Clone + Ord,
{
    fn evaluate(&self, board: &Board, color: &Color) -> T;
    fn min() -> T;
    fn max() -> T;
    fn draw() -> T;
}

pub struct HeuristicPlayer<H, T>
where
    H: Heuristic<T>,
    T: Clone + Debug + Ord,
{
    heuristic: H,
    phantom: PhantomData<T>,
}

impl<H, T> HeuristicPlayer<H, T>
where
    H: Heuristic<T>,
    T: Clone + Debug + Ord,
{
    pub fn new(heuristic: H) -> HeuristicPlayer<H, T> {
        HeuristicPlayer {
            heuristic,
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
}

impl<H, T> GamePlayer for HeuristicPlayer<H, T>
where
    H: Heuristic<T>,
    T: Clone + Debug + Ord,
{
    fn decide(&mut self, board: &Board, color: &Color) -> Move {
        self.choose_move(board, color)
    }
}
