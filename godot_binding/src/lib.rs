use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::{Arc, Barrier, Mutex};

use baz_core::{Board, Game, GamePlayer, Move};
use baz_players::{GoFasterHeuristic, HeuristicPlayer};
use godot::engine::{Node, NodeVirtual};
use godot::prelude::*;

struct MyExtension;

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {}

#[derive(GodotClass)]
#[class(base=Node)]
struct GodotGameBoard {
    #[base]
    _base: Base<Node>,

    // tx: Sender<Move>,
    // sync_barrier: Arc<Barrier>,
    // game: Arc<Mutex<Game<GodotGamePlayer, HeuristicPlayer<GoFasterHeuristic, i8>>>>,
    game: Game<GodotGamePlayer, HeuristicPlayer<GoFasterHeuristic, i8>>,
}

#[godot_api]
impl NodeVirtual for GodotGameBoard {
    fn init(base: Base<Node>) -> Self {
        // let (tx, rx) = channel();
        // let sync_barrier = Arc::new(Barrier::new(2));
        // let game = Arc::new(Mutex::new(Game::new(
        //     GodotGamePlayer::new(rx),
        //     GoFasterHeuristic::player(1),
        // )));
        let game = Game::new(GodotGamePlayer {}, GoFasterHeuristic::player(1));
        // let game_clone = game.clone();
        // let sync_barrier_clone = sync_barrier.clone();
        // std::thread::spawn(move || {
        //     // Wait for the initial update to finish
        //     sync_barrier_clone.wait();
        //     loop {
        //         // lock the game until human player plays
        //         game_clone.lock().unwrap().play_turn();
        //         // wait until Godot has updated everything and calls resume
        //         sync_barrier_clone.wait();
        //         // Play the AI move
        //         game_clone.lock().unwrap().play_turn();
        //     }
        // });
        Self {
            _base: base,
            // tx,
            game,
            // sync_barrier,
        }
    }
}

#[godot_api]
impl GodotGameBoard {
    // #[func]
    // fn resume(&self) {
    //     self.sync_barrier.wait();
    // }

    #[func]
    fn get_piece_at(&self, board_coords: Vector2) -> i64 {
        let position = (board_coords.x as i8, board_coords.y as i8).into();
        self.game
            .board()
            .get_piece_at(&position)
            .map(|p| p as i64)
            .unwrap_or(-1)
    }

    #[func]
    fn get_piece_position(&self, index: i64) -> Vector2 {
        // let game = self.game.lock().unwrap();
        let piece = self.game.board().get_piece(index.try_into().unwrap());
        Vector2 {
            x: piece.position.x().into(),
            y: piece.position.y().into(),
        }
    }

    #[func]
    fn get_piece_height(&self, index: i64) -> i64 {
        Into::<i8>::into(
            &self
                .game
                // .lock()
                // .unwrap()
                .board()
                .get_piece(index.try_into().unwrap())
                .height,
        )
        .into()
    }

    #[func]
    fn is_piece_white(&self, index: i64) -> bool {
        self.game
            // .lock()
            // .unwrap()
            .board()
            .get_piece(index.try_into().unwrap())
            .color
            == baz_core::Color::White
    }

    #[func]
    fn legal_moves(&self, index: i64) -> Array<Vector2> {
        self.game
            .board()
            .legal_moves_for(self.game.board().get_piece(index as usize))
            .iter()
            .map(|m| match m {
                Move::Boom(i) => {
                    let pos = self.game.board().get_piece(*i).position;
                    Vector2 {
                        x: pos.x() as f32,
                        y: pos.y() as f32,
                    }
                }
                Move::Zoom(_i, pos) => Vector2 {
                    x: pos.x() as f32,
                    y: pos.y() as f32,
                },
                Move::Score(_i) => Vector2 { x: -1.0, y: -1.0 },
            })
            .collect()
        // Array::new()
    }

    #[func]
    fn move_or_boom(&mut self, index: i64, board_coords: Vector2) {
        // let game = self.game.lock().unwrap();
        let position = (board_coords.x as i8, board_coords.y as i8).into();
        // if let Some(boomed_piece) = game.board().get_piece_at(&position) {
        //     self.tx.send(Move::Boom(boomed_piece)).unwrap()
        // } else {
        //     self.tx.send(Move::Zoom(index as usize, position)).unwrap()
        // }
        let piece = self.game.board().get_piece(index as usize);
        if &piece.color != self.game.turn() {
            return;
        }
        let mov = if let Some(boomed_piece) = self.game.board().get_piece_at(&position) {
            Move::Boom(boomed_piece)
        } else {
            Move::Zoom(index as usize, position)
        };
        if self
            .game
            .board()
            .legal_moves_for(piece)
            .iter()
            .any(|m| *m == mov)
        {
            self.game.apply_move(&mov);
            self.game.play_turn();
        }
    }

    #[func]
    fn score(&mut self, index: i64) {
        let mov = Move::Score(index as usize);
        if self
            .game
            .board()
            .legal_moves_for(self.game.board().get_piece(index as usize))
            .iter()
            .any(|m| *m == mov)
        {
            self.game.apply_move(&mov);
            self.game.play_turn();
        }
    }
}

struct GodotGamePlayer {
    // rx: Receiver<Move>,
}

// impl GodotGamePlayer {
//     fn new(rx: Receiver<Move>) -> GodotGamePlayer {
//         GodotGamePlayer { rx }
//     }
// }

impl GamePlayer for GodotGamePlayer {
    fn decide(&mut self, board: &Board, color: &baz_core::Color) -> Move {
        panic!("Not allowed")
        // self.rx.recv().unwrap()
    }
}
