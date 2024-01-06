use std::time::Duration;

use baz_core::{Board, Color, GamePlayer, Move, Position};
use baz_dueler::StdioGamePlayer;
use baz_players::{
    ForwardRandomPlayer, GeniusHeuristic, GoFastHeuristic, GoFasterHeuristic, HResult,
    HeuristicPlayer, MinMaxPlayer, NaiveHeuristic, RandomPlayer,
};
use clap::{Parser, Subcommand};
use num::Rational32;

struct StdinHumanPlayer();

impl GamePlayer for StdinHumanPlayer {
    fn decide(&mut self, board: &Board, color: &Color) -> Move {
        println!("{board:?}");
        println!("{:?}'s turn", color);
        loop {
            let first_piece_index = StdinHumanPlayer::pick_piece(board);
            if &board.pieces[first_piece_index].color != color {
                // Selected one of their pieces, can't move it
                println!("Choose one of your pieces");
                continue;
            }
            let second_position_target = StdinHumanPlayer::pick_position();
            let mov = if let HumanMoveTarget::Position(second_position) = second_position_target {
                if let Some(second_piece_index) = board.get_piece_at(&second_position) {
                    Move::Boom(second_piece_index)
                } else {
                    // Selected one of our pieces, zoom it
                    Move::Zoom(first_piece_index, second_position)
                }
            } else {
                Move::Score(first_piece_index)
            };
            if board
                .legal_moves_for(&board.pieces[first_piece_index])
                .any(|m| m == mov)
            {
                return mov;
            } else {
                println!("Invalid move");
            }
        }
    }
}

enum HumanMoveTarget {
    Position(Position),
    ScoreZone,
}

impl StdinHumanPlayer {
    fn pick_position() -> HumanMoveTarget {
        let stdin = std::io::stdin();
        let mut buffer = String::new();
        let mut position: Result<Position, ()> = Err(());
        while position.is_err() {
            stdin
                .read_line(&mut buffer)
                .map_err(|_| ())
                .expect("no IO error please");
            let trim_buffer = buffer.trim();
            if trim_buffer == "s" {
                return HumanMoveTarget::ScoreZone;
            }
            position = trim_buffer.try_into();
            if position.is_err() {
                println!("Not a valid square");
            }
        }
        HumanMoveTarget::Position(position.unwrap())
    }
    fn pick_piece(board: &Board) -> usize {
        let mut piece_index = None;
        while piece_index.is_none() {
            if let HumanMoveTarget::Position(position) = Self::pick_position() {
                piece_index = board.get_piece_at(&position);
            }
            if piece_index.is_none() {
                println!("No piece there")
            }
        }
        piece_index.unwrap()
    }
}

#[derive(Parser, Debug)]
#[command(author, version)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    Play {
        #[command(subcommand)]
        player: PlayerOptions,
    },
    // Not very interesting
    // Duel,
    // I'm lazy, godot already exists
    // HumanReadable,
}

#[derive(Clone, Debug, Subcommand)]
enum PlayerOptions {
    Random,
    RandomForward,
    GoFast,
    GoFaster,
    Genius,
    Naive,
}

enum AIPlayer {
    Random(RandomPlayer),
    RandomForward(ForwardRandomPlayer),
    GoFast(HeuristicPlayer<GoFastHeuristic, i8>),
    GoFaster(HeuristicPlayer<GoFasterHeuristic, i8>),
    Genius(MinMaxPlayer<GeniusHeuristic, HResult<Rational32>>),
    Naive(MinMaxPlayer<NaiveHeuristic, HResult<Rational32>>),
}
impl From<PlayerOptions> for AIPlayer {
    fn from(value: PlayerOptions) -> Self {
        match value {
            PlayerOptions::Random => AIPlayer::Random(RandomPlayer()),
            PlayerOptions::RandomForward => AIPlayer::RandomForward(ForwardRandomPlayer()),
            PlayerOptions::GoFast => AIPlayer::GoFast(HeuristicPlayer::new(GoFastHeuristic())),
            PlayerOptions::GoFaster => {
                AIPlayer::GoFaster(HeuristicPlayer::new(GoFasterHeuristic()))
            }
            PlayerOptions::Genius => AIPlayer::Genius(MinMaxPlayer::new(
                GeniusHeuristic(),
                Duration::from_millis(10),
            )),
            PlayerOptions::Naive => AIPlayer::Naive(MinMaxPlayer::new(
                NaiveHeuristic(),
                Duration::from_millis(10),
            )),
        }
    }
}
impl GamePlayer for AIPlayer {
    fn decide(&mut self, board: &Board, color: &Color) -> Move {
        match self {
            AIPlayer::Random(player) => player.decide(board, color),
            AIPlayer::RandomForward(player) => player.decide(board, color),
            AIPlayer::GoFast(player) => player.decide(board, color),
            AIPlayer::GoFaster(player) => player.decide(board, color),
            AIPlayer::Genius(player) => player.decide(board, color),
            AIPlayer::Naive(player) => player.decide(board, color),
        }
    }
}

fn main() -> std::io::Result<()> {
    // let mut game = Game::new(RandomPlayer(), StdinHumanPlayer());
    let args = Args::parse();
    match args.command {
        Commands::Play { player } => {
            let ai = AIPlayer::from(player);
            let mut stdio_player = StdioGamePlayer::new(ai);
            stdio_player.main()?;
        }
    }
    // let mut game = Game::new(RandomPlayer(), RandomPlayer());
    // game.finish_game();
    // // while game.winner().is_none() {
    // // game.play_turn();
    // // print_board(game.board());
    // // let stdin = std::io::stdin();
    // // let mut buffer = String::new();
    // // let _ = stdin.read_line(&mut buffer);
    // // }
    // println!("Winner: {:?}", game.winner());
    //
    // let mut whites = 0;
    // let mut blacks = 0;
    // let mut draws = 0;
    // for _ in 0..1000 {
    //     // let mut game = Game::new(GoFastHeuristic::player(0), GoFasterHeuristic::player());
    //     let mut game = Game::new(
    //         StdinHumanPlayer(),
    //         MinMaxPlayer::new(GeniusHeuristic(), Duration::from_secs(5)),
    //     );
    //     match game.finish_game() {
    //         baz_core::Winner::White => whites += 1,
    //         baz_core::Winner::Black => blacks += 1,
    //         baz_core::Winner::Draw => draws += 1,
    //     }
    // }
    // println!("White wins: {}", whites);
    // println!("Black wins: {}", blacks);
    // println!("Draws: {}", draws);
    //
    Ok(())
}
