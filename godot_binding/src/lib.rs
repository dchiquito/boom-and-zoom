use baz_core::Game;
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

    game: Game<HeuristicPlayer<GoFasterHeuristic, i8>, HeuristicPlayer<GoFasterHeuristic, i8>>,
}

#[godot_api]
impl NodeVirtual for GodotGameBoard {
    fn init(base: Base<Node>) -> Self {
        let game = Game::new(GoFasterHeuristic::player(1), GoFasterHeuristic::player(1));
        Self { _base: base, game }
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

