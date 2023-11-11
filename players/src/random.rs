use baz_core::*;
use rand::prelude::*;

pub struct RandomPlayer();

impl GamePlayer for RandomPlayer {
    fn decide(&mut self, board: &Board, color: &Color) -> Move {
        let mut rng = rand::thread_rng();
        let piece = board
            .pieces
            .iter()
            .filter(|p| p.height != Height::Dead)
            .filter(|p| &p.color == color)
            .choose(&mut rng)
            .unwrap();
        let moves = board.legal_moves_for(piece);
        *moves.choose(&mut rng).unwrap()
    }
}
