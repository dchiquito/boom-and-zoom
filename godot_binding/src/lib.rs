use baz_core::{Game, Piece};
use baz_players::{GoFasterHeuristic, HeuristicPlayer};
use core::ops::Deref;
use godot::engine::{Node, NodeVirtual};
use godot::prelude::*;

struct MyExtension;

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {}

#[derive(GodotClass)]
#[class(base=Node)]
struct GodotGameBoard {
    #[base]
    base: Base<Node>,

    game: Game<HeuristicPlayer<GoFasterHeuristic, i8>, HeuristicPlayer<GoFasterHeuristic, i8>>,
    // pieces: Vec<Gd<GamePiece>>,
}

#[godot_api]
impl NodeVirtual for GodotGameBoard {
    fn init(mut base: Base<Node>) -> Self {
        godot_print!("Hello, world!"); // Prints to the Godot console

        let game = Game::new(GoFasterHeuristic::player(1), GoFasterHeuristic::player(1));

        // let game_piece_scene: Gd<PackedScene> = load("res://GamePiece.tscn");
        // let pieces: Vec<Gd<GamePiece>> = game
        //     .board()
        //     .pieces
        //     .iter()
        //     .map(|p| {
        //         let mut piece = game_piece_scene.instantiate_as::<GamePiece>();
        //         piece
        //     })
        //     .collect();
        // for p in pieces {
        //     base.add_child(p);
        // }

        Self { base, game }
    }
}

#[godot_api]
impl GodotGameBoard {
    #[func]
    fn logit(&self) {
        godot_print!("LOGGIN IT");
    }
    #[func]
    fn get_piece_position(&self, index: i64) -> Vector2 {
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
                .board()
                .get_piece(index.try_into().unwrap())
                .height,
        )
        .into()
    }

    #[func]
    fn is_piece_white(&self, index: i64) -> bool {
        self.game.board().get_piece(index.try_into().unwrap()).color == baz_core::Color::White
    }

    #[func]
    fn advance(&mut self) {
        self.game.play_turn();
    }
}

// #[derive(GodotClass)]
// #[class(base=Node2D)]
// struct GamePiece {
//     #[base]
//     base: Base<Node2D>,
//
//     #[var]
//     is_white: bool,
//
//     #[var]
//     height: i64,
//
//     #[var]
//     board_coords: Vector2,
// }
//
// #[godot_api]
// impl Node2DVirtual for GamePiece {
//     fn init(base: Base<Node2D>) -> Self {
//         Self { base, is_white:true, height:3, board_coords:Vector2{x:0.0,y:0.0} }
//     }
//     // #[func]
//     fn on_notification(&mut self, piece: &Piece) {
//         self.is_white = piece.color == baz_core::Color::White;
//         self.height = Into::<i8>::into(&piece.height).into();
//         self.board_coords = Vector2 {
//             x: piece.position.x().into(),
//             y: piece.position.y().into(),
//         }
//     }
//
// }
//
// #[godot_api]
// impl GamePiece {
// }
