use baz_core::{Color, Height};

use crate::heuristic::{Heuristic, HeuristicPlayer};

// Try to end the game as soon as possible
pub struct GoFastHeuristic();

impl Heuristic<i8> for GoFastHeuristic {
    fn evaluate(&mut self, board: &baz_core::Board, color: &baz_core::Color) -> i8 {
        -board
            .pieces
            .iter()
            .filter(|p| &p.color == color && p.height != Height::Dead)
            .map(|p| {
                (
                    Into::<i8>::into(&p.height),
                    if color == &Color::White {
                        7 - p.position.y()
                    } else {
                        p.position.y()
                    },
                )
            })
            .map(|(height, distance)| (distance + height - 1) / height)
            .sum::<i8>()
    }
}

impl GoFastHeuristic {
    pub fn player(depth: usize) -> HeuristicPlayer<GoFastHeuristic, i8> {
        HeuristicPlayer::new(GoFastHeuristic(), depth)
    }
}

// Try to always end the game faster than the opponent. This enables strategic booming instead of
// only zooming as fast as possible.
pub struct GoFasterHeuristic();

impl Heuristic<i8> for GoFasterHeuristic {
    fn evaluate(&mut self, board: &baz_core::Board, color: &Color) -> i8 {
        let mut heuristic = GoFastHeuristic();
        // let a = heuristic.evaluate(board, color);
        // let b = heuristic.evaluate(board, &color.invert());
        // println!("{} - {} = {}", a, b, a - b);
        heuristic.evaluate(board, color) - heuristic.evaluate(board, &color.invert())
    }
}
impl GoFasterHeuristic {
    pub fn player(depth: usize) -> HeuristicPlayer<GoFasterHeuristic, i8> {
        HeuristicPlayer::new(GoFasterHeuristic(), depth)
    }
}
