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
            time_per_turn,
            timeout: None,
            phantom: PhantomData,
        }
    }
    fn minimax(
        &mut self,
        board: &Board,
        color: &Color,
        maximizing: bool,
        mut alpha: T,
        mut beta: T,
        depth: usize,
    ) -> (T, Option<Move>) {
        if let Some(winner) = board.winner() {
            if let Some(winner_color) = winner.color() {
                if &winner_color == color {
                    return (H::max(), None);
                } else {
                    return (H::min(), None);
                }
            } else {
                return (H::draw(), None);
            }
        }
        self.iterations += 1;
        if depth >= self.max_depth {
            return (self.heuristic.evaluate(board, color), None);
        }
        if let Some(timeout) = self.timeout {
            if Instant::now() > timeout {
                return (self.heuristic.evaluate(board, color), None);
            }
        }
        let piece_color = if maximizing {
            color.clone()
        } else {
            color.invert()
        };
        let mut scores_and_boards = board
            .legal_moves(&piece_color)
            .map(|m| (m, board.apply_move(&m)))
            .map(|(m, b)| (self.heuristic.evaluate(&b, color), m, b))
            .collect::<Vec<(T, Move, Board)>>();
        scores_and_boards
            .sort_by(|(h1, _, _), (h2, _, _)| if maximizing { h2.cmp(h1) } else { h1.cmp(h2) });
        let mut best_score = if maximizing { H::min() } else { H::max() };
        let mut best_move = None;
        for (_estimate, new_move, new_board) in scores_and_boards.iter().take(self.max_width) {
            let (new_score, _) = self.minimax(
                new_board,
                color,
                !maximizing,
                alpha.clone(),
                beta.clone(),
                depth + 1,
            );
            if maximizing {
                if new_score > best_score {
                    best_score = new_score;
                    best_move = Some(new_move);
                    if best_score > beta {
                        break;
                    }
                    alpha = alpha.max(best_score.clone());
                }
            } else if new_score < best_score {
                best_score = new_score;
                best_move = Some(new_move);
                if best_score < alpha {
                    break;
                }
                beta = beta.min(best_score.clone());
            };
        }
        (best_score, best_move.copied())
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
        let mut best_move = (H::min(), None);
        let mut last_best_move = (H::min(), None);
        self.max_depth = 3;
        self.max_width = 6; // TODO tune this
        while Instant::now() - now < self.time_per_turn {
            last_best_move = best_move;
            best_move = self.minimax(board, color, true, H::min(), H::max(), 0);
            self.max_depth += 1;
        }
        let time_taken = Instant::now() - now;
        println!(
            "Decided after {time_taken:?}, {} iterations, and a max depth of {}: {:?}",
            self.iterations,
            self.max_depth - 2,
            last_best_move
        );
        // Intentionally discard the abortive partially calculated result
        last_best_move.1.unwrap()
    }
}

#[cfg(test)]
mod test {
    use crate::GeniusHeuristic;

    use super::*;

    #[test]
    fn test_final_moves() {
        let mut board = Board::default();
        /*
         * 8 w.......
         * 7 ........
         * 6 ........
         * 5 ........
         * 4 ........
         * 3 ........
         * 2 ........
         * 1 b.......
         *   abcdefgh
         */
        board.pieces[0].position = "a4".try_into().unwrap();
        board.pieces[1].height = Height::Dead;
        board.pieces[2].height = Height::Dead;
        board.pieces[3].height = Height::Dead;
        board.pieces[4].position = "h1".try_into().unwrap();
        board.pieces[5].height = Height::Dead;
        board.pieces[6].height = Height::Dead;
        board.pieces[7].height = Height::Dead;

        let mut player = MinMaxPlayer::new(GeniusHeuristic(), Duration::from_millis(10));
        assert_eq!(player.decide(&board, &Color::White), Move::Score(0));
        assert_eq!(1, 2);
    }
}
