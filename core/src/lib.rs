#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Color {
    White,
    Black,
}

#[derive(Clone)]
pub struct Position {
    index: i8,
    x: i8,
    y: i8,
}
impl From<(i8, i8)> for Position {
    fn from((x, y): (i8, i8)) -> Self {
        Position {
            index: x + (y * 8),
            x,
            y,
        }
    }
}
impl From<i8> for Position {
    fn from(index: i8) -> Self {
        Position {
            index,
            x: index % 8,
            y: index / 8,
        }
    }
}
impl std::fmt::Debug for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let columns = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];
        f.write_str(&format!("{}{}", columns[self.x as usize], self.y + 1))
    }
}
impl TryFrom<&str> for Position {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let columns = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];
        let mut chars = value.chars();
        let col = chars.next().ok_or(())?;
        let row = chars.next().ok_or(())?;
        if chars.next().is_some() {
            Err(())
        } else {
            let x = columns.iter().position(|cname| cname == &col).ok_or(())?;
            let y = row.to_digit(10).ok_or(())? - 1;
            Ok((x as i8, y as i8).into())
        }
    }
}
impl Position {
    pub fn offset(&self, dx: i8, dy: i8) -> PositionOffset {
        let x = self.x + dx;
        let y = self.y + dy;
        let index = x + (y * 8);
        if y == -1 && (-1..9).contains(&x) {
            // Meaning black gets points for going here
            PositionOffset::ScoreZone(Color::Black)
        } else if y == 8 && (-1..9).contains(&x) {
            // Meaning white gets points for going here
            PositionOffset::ScoreZone(Color::White)
        } else if !(0..8).contains(&x) || !(0..8).contains(&y) {
            PositionOffset::Invalid
        } else {
            PositionOffset::Valid(Position { x, y, index })
        }
    }
}

pub enum PositionOffset {
    Valid(Position),
    ScoreZone(Color),
    Invalid,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Height {
    Dead,
    One,
    Two,
    Three,
}
impl TryFrom<u8> for Height {
    type Error = ();
    fn try_from(height: u8) -> Result<Self, Self::Error> {
        match height {
            0 => Ok(Height::Dead),
            1 => Ok(Height::One),
            2 => Ok(Height::Two),
            3 => Ok(Height::Three),
            _ => Err(()),
        }
    }
}
impl From<&Height> for u8 {
    fn from(height: &Height) -> Self {
        match height {
            Height::Dead => 0,
            Height::One => 1,
            Height::Two => 2,
            Height::Three => 3,
        }
    }
}
impl Height {
    pub fn boom(&self) -> Height {
        match self {
            Height::Three => Height::Two,
            Height::Two => Height::One,
            Height::One => Height::Dead,
            Height::Dead => Height::Dead,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Piece {
    pub color: Color,
    pub position: Position,
    pub height: Height,
}
impl Piece {
    pub fn new(color: Color, position: (i8, i8)) -> Piece {
        Piece {
            color,
            position: position.into(),
            height: Height::Three,
        }
    }
    pub fn boom(&mut self) {
        self.height = self.height.boom()
    }
}

#[derive(Debug)]
pub enum Move {
    Boom(usize),
    Zoom(usize, Position),
    Score(usize),
}

#[derive(Clone)]
pub struct Board {
    pub pieces: [Piece; 8],
    pub black_score: u8,
    pub white_score: u8,
}
impl Default for Board {
    fn default() -> Board {
        Board {
            pieces: [
                Piece::new(Color::White, (2, 0)),
                Piece::new(Color::White, (3, 0)),
                Piece::new(Color::White, (4, 0)),
                Piece::new(Color::White, (5, 0)),
                Piece::new(Color::Black, (2, 7)),
                Piece::new(Color::Black, (3, 7)),
                Piece::new(Color::Black, (4, 7)),
                Piece::new(Color::Black, (5, 7)),
            ],
            black_score: 0,
            white_score: 0,
        }
    }
}
impl Board {
    pub fn apply_move(&self, mov: &Move) -> Board {
        let mut new_board = self.clone();
        match mov {
            Move::Boom(idx) => {
                new_board.pieces[*idx].boom();
            }
            Move::Zoom(idx, position) => {
                new_board.pieces[*idx].position = position.clone();
            }
            Move::Score(idx) => {
                if new_board.pieces[*idx].color == Color::White {
                    new_board.white_score += Into::<u8>::into(&new_board.pieces[*idx].height);
                }
                new_board.pieces[*idx].height = Height::Dead;
            }
        }
        new_board
    }
    pub fn get_piece_at(&self, position: &Position) -> Option<usize> {
        (0..8).find(|&i| self.pieces[i].position.index == position.index)
    }
    pub fn valid_moves_for(&self, piece: &Piece) -> Vec<Move> {
        let piece_index = self
            .get_piece_at(&piece.position)
            .expect("piece needs to be one the board");
        if piece.height == Height::Dead {
            return vec![];
        }
        let directions = [
            (-1, 0),
            (-1, -1),
            (0, -1),
            (1, -1),
            (1, 0),
            (1, 1),
            (0, 1),
            (-1, 1),
        ];
        let mut moves = vec![];
        let mut has_scored = false;
        // Attempt movement in every direction
        for (dx, dy) in directions {
            // Set our initial range and position
            let mut position = piece.position.clone().offset(dx, dy);
            let mut range = piece.height.boom();
            while range != Height::Dead {
                // This optional ensures we don't move off the edge
                // if let Some(pos) = &position {
                match &position {
                    PositionOffset::Valid(pos) => {
                        // Check if we have encountered another piece
                        if let Some(piece_at_pos_index) = self.get_piece_at(pos) {
                            // If the piece is an enemy, we can boom it
                            if self.pieces[piece_at_pos_index].color != piece.color {
                                moves.push(Move::Boom(piece_at_pos_index));
                            }
                            // No jumping over pieces, so we are done with this direction
                            break;
                        } else {
                            // Empty square, we can move there
                            moves.push(Move::Zoom(piece_index, pos.clone()));
                            // Increment our position and decrement our range
                            position = pos.offset(dx, dy);
                            range = range.boom();
                        }
                    }
                    PositionOffset::ScoreZone(color) => {
                        if !has_scored && color == &piece.color {
                            moves.push(Move::Score(piece_index));
                            // We only want to have scoring as an option once, even if
                            // it's possible to score in multiple different ways
                            has_scored = true;
                        }
                        // We've reached the score zone, can't move any further than that
                        break;
                    }
                    PositionOffset::Invalid => {
                        // We've walked off the side of the board, so stop
                        break;
                    }
                }
            }
        }
        moves
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let board = Board::default();
        println!("{:?}", board.pieces[0].position);
        println!("{:?}", board.valid_moves_for(&board.pieces[0]));
        assert_eq!(4, 5);
    }
}
