use baz_core::{Board, Color, Height, Winner};
use num::rational::Rational32;

use crate::heuristic::{HResult, Heuristic};

pub struct NaiveHeuristic();

const LOGIT: bool = false;
impl Heuristic<HResult<Rational32>> for NaiveHeuristic {
    fn evaluate(
        &mut self,
        board: &baz_core::Board,
        color: &baz_core::Color,
    ) -> HResult<Rational32> {
        if let Some(final_score) = match board.winner() {
            Some(Winner::White) => {
                if color == &Color::White {
                    Some(Self::max())
                } else {
                    Some(Self::min())
                }
            }
            Some(Winner::Black) => {
                if color == &Color::White {
                    Some(Self::min())
                } else {
                    Some(Self::max())
                }
            }
            Some(Winner::Draw) => Some(HResult::Draw),
            None => None,
        } {
            return final_score;
        }
        let (our_score, our_turns) = Self::score_and_turns(board, color);
        let (their_score, their_turns) = Self::score_and_turns(board, &color.invert());
        let turns = our_turns.min(their_turns);
        HResult::Unknown(
            Rational32::new(our_score * turns, our_turns)
                - Rational32::new(their_score * turns, their_turns),
        )
    }
    fn min() -> HResult<Rational32> {
        HResult::Loss
    }
    fn max() -> HResult<Rational32> {
        HResult::Win
    }
    fn draw() -> HResult<Rational32> {
        HResult::Draw
    }
}

impl NaiveHeuristic {
    fn score_and_turns(board: &Board, color: &Color) -> (i32, i32) {
        let range = match color {
            Color::White => 0..4,
            Color::Black => 4..8,
        };
        board.pieces[range]
            .iter()
            .filter(|p| p.height != Height::Dead)
            .map(|p| {
                (
                    Into::<i32>::into(Into::<i8>::into(&p.height)),
                    if color == &Color::White {
                        8 - p.position.y() as i32
                    } else {
                        1 + p.position.y() as i32
                    },
                )
            })
            .filter(|(height, _distance)| *height > 0)
            .map(|(height, distance)| (height, (distance + height - 1) / height))
            .fold((0, 0), |(score, sum_turns), (height, turns)| {
                (score + height, sum_turns + turns)
            })
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn score_and_turns_no_pieces() {
//         assert_eq!(NaiveHeuristic::score_and_turns(0, &[]), (0, 0));
//     }
//     #[test]
//     fn score_and_turns_one_pieces() {
//         assert_eq!(
//             GeniusHeuristic::score_and_turns(0, &[(1, 1, false)]),
//             (1, 1)
//         );
//         assert_eq!(GeniusHeuristic::score_and_turns(0, &[(1, 1, true)]), (1, 1));
//     }
//     #[test]
//     fn score_and_turns_turn_calculation() {
//         let matrix = [
//             // Height 1
//             ((1, 1, false), (1, 1)),
//             ((1, 2, false), (1, 2)),
//             ((1, 3, false), (1, 3)),
//             ((1, 4, false), (1, 4)),
//             ((1, 5, false), (1, 5)),
//             ((1, 6, false), (1, 6)),
//             ((1, 7, false), (1, 7)),
//             ((1, 8, false), (1, 8)),
//             // Height 2
//             ((2, 1, false), (2, 1)),
//             ((2, 2, false), (2, 1)),
//             ((2, 3, false), (2, 2)),
//             ((2, 4, false), (2, 2)),
//             ((2, 5, false), (2, 3)),
//             ((2, 6, false), (2, 3)),
//             ((2, 7, false), (2, 4)),
//             ((2, 8, false), (2, 4)),
//             // Height 3
//             ((3, 1, false), (3, 1)),
//             ((3, 2, false), (3, 1)),
//             ((3, 3, false), (3, 1)),
//             ((3, 4, false), (3, 2)),
//             ((3, 5, false), (3, 2)),
//             ((3, 6, false), (3, 2)),
//             ((3, 7, false), (3, 3)),
//             ((3, 8, false), (3, 3)),
//         ];
//         for (datum, expected) in matrix {
//             assert_eq!(GeniusHeuristic::score_and_turns(0, &[datum]), expected);
//         }
//     }
//     #[test]
//     fn score_and_turns_no_boom() {
//         assert_eq!(
//             GeniusHeuristic::score_and_turns(0, &[(3, 1, false), (2, 1, false), (1, 1, true)]),
//             (6, 3)
//         );
//     }
//     #[test]
//     fn score_and_turns_one_boom() {
//         assert_eq!(
//             GeniusHeuristic::score_and_turns(0b1, &[(1, 1, true)]),
//             (0, 0)
//         );
//         assert_eq!(
//             GeniusHeuristic::score_and_turns(0b1, &[(2, 1, true)]),
//             (1, 1)
//         );
//         assert_eq!(
//             GeniusHeuristic::score_and_turns(0b1, &[(3, 1, true)]),
//             (2, 1)
//         );
//         assert_eq!(
//             GeniusHeuristic::score_and_turns(0b1, &[(3, 8, true)]),
//             (2, 4)
//         );
//     }
// }
