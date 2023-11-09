use baz_core::{play_game, Board, Color, GamePlayer, Height, Move, Position};

struct StdinHumanPlayer();

fn print_board(board: &Board) {
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
            let second_position = StdinHumanPlayer::pick_position();
            let mov = if let Some(second_piece_index) = board.get_piece_at(&second_position) {
                Move::Boom(second_piece_index)
            } else {
                // Selected one of our pieces, zoom it
                Move::Zoom(first_piece_index, second_position)
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

impl StdinHumanPlayer {
    fn pick_position() -> Position {
        let stdin = std::io::stdin();
        let mut buffer = String::new();
        let mut position: Result<Position, ()> = Err(());
        while position.is_err() {
            stdin
                .read_line(&mut buffer)
                .map_err(|_| ())
                .expect("no IO error please");
            position = buffer.trim().try_into();
            if position.is_err() {
                println!("Not a valid square");
            }
        }
        position.unwrap()
    }
    fn pick_piece(board: &Board) -> usize {
        let mut piece_index = None;
        while piece_index.is_none() {
            piece_index = board.get_piece_at(&Self::pick_position());
            if piece_index.is_none() {
                println!("No piece there")
            }
        }
        piece_index.unwrap()
    }
}

fn main() -> std::io::Result<()> {
    println!("Hello, world!");
    // let board = Board::default();
    // print_board(&board);
    // let pos = read_position_from_input().unwrap();
    // let x = board.get_piece_at(&pos);
    play_game(StdinHumanPlayer(), StdinHumanPlayer());

    Ok(())
}
