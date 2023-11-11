use baz_core::{Board, Color, Game, GamePlayer, Height, Move, Position};
use baz_players::RandomPlayer;

struct StdinHumanPlayer();

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

impl GamePlayer for StdinHumanPlayer {
    fn decide(&mut self, board: &Board, color: &Color) -> Board {
        print_board(board);
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
                .valid_moves_for(&board.pieces[first_piece_index])
                .iter()
                .any(|m| m == &mov)
            {
                return board.apply_move(&mov);
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

fn main() -> std::io::Result<()> {
    // let mut game = Game::new(RandomPlayer(), StdinHumanPlayer());
    let mut game = Game::new(RandomPlayer(), RandomPlayer());
    game.finish_game();
    // while game.winner().is_none() {
    // game.play_turn();
    // print_board(game.board());
    // let stdin = std::io::stdin();
    // let mut buffer = String::new();
    // let _ = stdin.read_line(&mut buffer);
    // }
    println!("Winner: {:?}", game.winner());

    let mut whites = 0;
    let mut blacks = 0;
    let mut draws = 0;
    for _ in 0..1000 {
        let mut game = Game::new(RandomPlayer(), RandomPlayer());
        match game.finish_game() {
            baz_core::Winner::White => whites += 1,
            baz_core::Winner::Black => blacks += 1,
            baz_core::Winner::Draw => draws += 1,
        }
    }
    println!("White wins: {}", whites);
    println!("Black wins: {}", blacks);
    println!("Draws: {}", draws);

    Ok(())
}
