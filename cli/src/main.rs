use baz_core::{Board, Color, Position};

fn print_board(board: &Board) {
    for y in (0..8).rev() {
        print!("{}| ", y + 1);
        for x in 0..8 {
            if let Some(index) = board.get_piece_at(&(x, y).into()) {
                let piece = &board.pieces[index];
                if piece.color == Color::Black {
                    print!("\x1b[31m");
                } else {
                    print!("\x1b[32m");
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

fn read_position_from_input() -> Result<Position, ()> {
    let stdin = std::io::stdin();
    let mut buffer = String::new();
    stdin.read_line(&mut buffer).map_err(|_| ())?;
    buffer.trim().try_into()
}

fn main() -> std::io::Result<()> {
    println!("Hello, world!");
    let board = Board::default();
    print_board(&board);
    let pos = read_position_from_input().unwrap();
    let x = board.get_piece_at(&pos);

    Ok(())
}
