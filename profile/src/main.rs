use baz_core::{Board, Color, Game, Height};
use baz_players::{GeniusHeuristic, Heuristic, MinMaxPlayer};
use clap::{Parser, ValueEnum};

#[derive(Clone, Debug, ValueEnum)]
enum Player {
    Minimax,
}

// impl FromStr for Player {
//     type Err = ();
//
//     fn from_str(s: &str) -> Result<Self, Self::Err> {
//         match s.to_ascii_lowercase() {
//
//             _ => Err(())
//         }
//     }
// }

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    #[arg(long)]
    player1: Player,
    #[arg(long)]
    player2: Player,
    #[arg(long, default_value_t = 2)]
    depth1: usize,
    #[arg(long, default_value_t = 2)]
    depth2: usize,
}

fn main() {
    let args = Args::parse();
    println!("{args:?}");
    let player1 = match args.player1 {
        Player::Minimax => MinMaxPlayer::new(GeniusHeuristic(), args.depth1),
    };
    let player2 = match args.player2 {
        Player::Minimax => MinMaxPlayer::new(GeniusHeuristic(), args.depth2),
    };
    let mut game = Game::new(player1, player2);
    while game.board().winner().is_none() {
        game.play_turn();
        println!();
        print_board(game.board());
        println!();
        GeniusHeuristic().evaluate(game.board(), &Color::Black);
        let stdin = std::io::stdin();
        let mut buffer = String::new();
        let _ = stdin.read_line(&mut buffer).unwrap();
    }
}

fn print_board(board: &Board) {
    println!("White: {}  Black {}", board.white_score, board.black_score);
    for y in (0..8).rev() {
        print!("{}| ", y + 1);
        for x in 0..8 {
            if let Some(index) = board.get_piece_at(&(x, y).into()) {
                let piece = &board.pieces[index];
                if piece.height == Height::Dead {
                    print!(". ");
                } else if piece.color == Color::Black {
                    print!("\x1b[37;40m");
                } else {
                    print!("\x1b[47;30m");
                }
                print!("{}\x1b[0m ", Into::<u8>::into(&piece.height));
            } else {
                print!(". ");
            }
        }
        println!();
    }
    println!(" +-----------------");
    println!("   a b c d e f g h");
}
