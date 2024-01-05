use baz_core::{Board, Color, GamePlayer, Move};

pub fn serialize_move(mov: &Move) -> String {
    match mov {
        Move::Boom(index) => format!("Boom {index}"),
        Move::Zoom(index, pos) => format!("Zoom {} {}", index, i8::from(*pos)),
        Move::Score(index) => format!("Score {index}"),
        Move::Concede => "Concede".to_string(),
    }
}
pub fn deserialize_move(line: &str) -> Move {
    if let Some(remainder) = line.strip_prefix("Boom ") {
        Move::Boom(remainder.trim_end().parse().expect("Invalid index"))
    } else if let Some(remainder) = line.strip_prefix("Zoom ") {
        let (index, pos) = remainder
            .trim_end()
            .split_once(' ')
            .expect("Malformed zoom serialization");
        Move::Zoom(
            index.parse().expect("Invalid index"),
            pos.parse::<i8>().expect("Invalid position").into(),
        )
    } else if let Some(remainder) = line.strip_prefix("Score ") {
        Move::Score(remainder.trim_end().parse().expect("Invalid index"))
    } else if line == "Concede\n" {
        Move::Concede
    } else {
        panic!("Unable to deserialize \"{line}\"")
    }
}

pub struct StdioGamePlayer<T>
where
    T: GamePlayer,
{
    player: T,
}

impl<T> StdioGamePlayer<T>
where
    T: GamePlayer,
{
    pub fn new(player: T) -> StdioGamePlayer<T> {
        StdioGamePlayer { player }
    }
    pub fn main(&mut self) -> std::io::Result<()> {
        let stdin = std::io::stdin();
        let mut buffer = String::new();
        let mut board = Board::default();
        // Get the color from stdin
        stdin.read_line(&mut buffer)?;
        let color: Color;
        // We must make the first move if we are playing white
        if buffer == "white\n" {
            color = Color::White;
            let our_move = self.player.decide(&board, &color);
            board = board.apply_move(&our_move);
            println!("{}", serialize_move(&our_move));
        } else if buffer == "black\n" {
            color = Color::Black;
        } else {
            panic!("Unrecognized color statement: {buffer}")
        }
        while board.winner().is_none() {
            // Get the opponents move from stdin and apply it to the board
            buffer = String::new();
            stdin.read_line(&mut buffer)?;
            let their_move = deserialize_move(&buffer);
            board = board.apply_move(&their_move);
            // Decide on a move and print it
            let our_move = self.player.decide(&board, &color);
            board = board.apply_move(&our_move);
            println!("{}", serialize_move(&our_move));
        }
        Ok(())
    }
}
