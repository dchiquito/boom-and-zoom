use std::fmt::Debug;
use std::marker::PhantomData;

use crate::heuristic::Heuristic;
use baz_core::*;
use num::traits::Inv;

pub struct MinMaxPlayer<H, T>
where
    H: Heuristic<T>,
    T: Clone + Debug + Inv<Output = T> + Ord,
{
    heuristic: H,
    depth: usize,
    phantom: PhantomData<T>,
}
impl<H, T> MinMaxPlayer<H, T>
where
    H: Heuristic<T>,
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
    fn minimax(
        &mut self,
        board: &Board,
        color: &Color,
        maximizing: bool,
        depth: usize,
        mut alpha: T,
        mut beta: T,
    ) -> T {
        if depth == 0 {
            return self.heuristic.evaluate(board, color);
        }
        // println!("{color:?} max:{maximizing} depth:{depth} alpha:{alpha:?} beta:{beta:?}",);
        let mut scores_and_boards = board
            .legal_moves(color)
            .iter()
            .map(|m| board.apply_move(m))
            .map(|b| (self.heuristic.evaluate(&b, color), b))
            .collect::<Vec<(T, Board)>>();
        scores_and_boards
            .sort_by(|(h1, _), (h2, _)| if maximizing { h1.cmp(h2) } else { h2.cmp(h1) });
        let mut v = if maximizing { H::min() } else { H::max() };
        for (_, new_board) in scores_and_boards {
            // println!("Checkin out {new_board:#?} {alpha_limit:?} {beta:?}");
            let new_v = self.minimax(
                &new_board,
                color,
                !maximizing,
                depth - 1,
                alpha.clone(),
                beta.clone(),
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
    T: Clone + Debug + Inv<Output = T> + Ord,
{
    fn decide(&mut self, board: &Board, color: &Color) -> Move {
        // let start = Instant::now();
        // let min_wait = Duration::from_secs(1);
        // let mut mov = board.legal_moves(color)[0];
        // let mut depth = 1;
        // while start.elapsed() < min_wait {
        *board
            .legal_moves(color)
            .iter()
            .max_by_key(|m| {
                self.minimax(
                    &board.apply_move(m),
                    color,
                    true,
                    self.depth,
                    H::min(),
                    H::max(),
                )
            })
            // TODO why this throw error
            .unwrap()
        // depth += 1;
        // }
        // println!(
        //     "Decided after {} seconds and searching to depth {}\n",
        //     start.elapsed().as_secs(),
        //     depth
        // );
        // mov
    }
}
