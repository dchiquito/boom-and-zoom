use baz_core::*;
use rand::prelude::*;

use crate::RandomPlayer;

// Picks a random piece and moves it forward
pub struct ForwardRandomPlayer();

impl GamePlayer for ForwardRandomPlayer {
    fn decide(&mut self, board: &Board, color: &Color) -> Move {
        let mut rng = rand::thread_rng();
        let piece = board
            .pieces
            .iter()
            .filter(|p| p.height != Height::Dead)
            .filter(|p| &p.color == color)
            .choose(&mut rng)
            .unwrap();
        board
            .legal_moves_for(piece)
            .filter(|m| match m {
                Move::Zoom(_, position) => match color {
                    Color::White => position.y() > piece.position.y(),
                    Color::Black => position.y() < piece.position.y(),
                },
                Move::Score(_) => true,
                Move::Boom(_) => false,
                // While technically a legal move, conceding will never be suggested
                Move::Concede(_) => unreachable!(),
            })
            .choose(&mut rng)
            .unwrap_or_else(|| RandomPlayer().decide(board, color))
    }
}
