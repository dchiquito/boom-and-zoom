use std::str::FromStr;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum Color {
    White,
    Black,
}
impl Color {
    pub fn invert(&self) -> Color {
        match self {
            Color::White => Color::Black,
            Color::Black => Color::White,
        }
    }
}
impl FromStr for Color {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "white" => Ok(Color::White),
            "w" => Ok(Color::White),
            "black" => Ok(Color::Black),
            "b" => Ok(Color::Black),
            _ => Err("Not a valid color".to_string()),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum Winner {
    White,
    Black,
    Draw,
}

impl Winner {
    pub fn color(&self) -> Option<Color> {
        match self {
            Winner::White => Some(Color::White),
            Winner::Black => Some(Color::Black),
            Winner::Draw => None,
        }
    }
}

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
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
impl From<Position> for i8 {
    fn from(pos: Position) -> Self {
        pos.x + (pos.y * 8)
    }
}
impl From<u8> for Position {
    fn from(index: u8) -> Self {
        Position::from(index as i8)
    }
}
impl From<Position> for u8 {
    fn from(pos: Position) -> Self {
        i8::from(pos) as u8
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

    pub fn x(&self) -> i8 {
        self.x
    }

    pub fn y(&self) -> i8 {
        self.y
    }
}

pub enum PositionOffset {
    Valid(Position),
    ScoreZone(Color),
    Invalid,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
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
impl TryFrom<i8> for Height {
    type Error = ();
    fn try_from(height: i8) -> Result<Self, Self::Error> {
        match height {
            0 => Ok(Height::Dead),
            1 => Ok(Height::One),
            2 => Ok(Height::Two),
            3 => Ok(Height::Three),
            _ => Err(()),
        }
    }
}
impl From<&Height> for i8 {
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

#[derive(Clone, Debug, Hash)]
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
    pub fn can_boom(&self, other: &Piece) -> bool {
        let dx = other.position.x - self.position.x;
        let dy = other.position.y - self.position.y;
        let height = i8::from(&self.height);
        (dx <= height && (dy == 0 || dy == dx)) || (dy <= height && dx == 0)
    }
    // There are not enough bits to serialize the color :(
    pub fn as_u8(&self) -> u8 {
        (u8::from(&self.height) << 6) + u8::from(self.position)
    }
    pub fn from_u8(color: Color, u: u8) -> Piece {
        let position = Position::from(u & 0b111111);
        let height = Height::try_from(u >> 6).unwrap();
        Piece {
            color,
            position,
            height,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Move {
    Boom(usize),
    Zoom(usize, Position),
    Score(usize),
    Concede(Color),
}

#[derive(Clone, Hash)]
pub struct Board {
    pub pieces: [Piece; 8],
    pub black_score: u8,
    pub white_score: u8,
    pub victory_by_concession: Option<Color>,
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
            victory_by_concession: None,
        }
    }
}
impl std::fmt::Debug for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "White: {}  Black {}", self.white_score, self.black_score)?;
        for y in (0..8).rev() {
            write!(f, "{}| ", y + 1)?;
            for x in 0..8 {
                if let Some(index) = self.get_piece_at(&(x, y).into()) {
                    let piece = &self.pieces[index];
                    if piece.height == Height::Dead {
                        write!(f, ". ")?;
                    } else if piece.color == Color::Black {
                        write!(f, "\x1b[37;40m")?;
                    } else {
                        write!(f, "\x1b[47;30m")?;
                    }
                    write!(f, "{}\x1b[0m ", Into::<u8>::into(&piece.height))?;
                } else {
                    write!(f, ". ")?;
                }
            }
            writeln!(f)?;
        }
        writeln!(f, " +-----------------")?;
        writeln!(f, "   a b c d e f g h")?;
        Ok(())
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
                new_board.pieces[*idx].position = *position;
            }
            Move::Score(idx) => {
                let points_scored = Into::<u8>::into(&new_board.pieces[*idx].height);
                match new_board.pieces[*idx].color {
                    Color::White => new_board.white_score += points_scored,
                    Color::Black => new_board.black_score += points_scored,
                }
                new_board.pieces[*idx].height = Height::Dead;
            }
            Move::Concede(color) => {
                new_board.victory_by_concession = Some(color.invert());
            }
        }
        new_board
    }
    pub fn get_piece(&self, index: usize) -> &Piece {
        &self.pieces[index]
    }
    pub fn get_piece_at(&self, position: &Position) -> Option<usize> {
        (0..8).find(|&i| {
            self.pieces[i].height != Height::Dead && self.pieces[i].position.index == position.index
        })
    }
    pub fn legal_moves(&self, color: &Color) -> LegalMoveIterator {
        LegalMoveIterator::for_color(self, color)
    }
    pub fn legal_moves_for(&self, piece: &Piece) -> LegalMoveIterator {
        let piece_index = self
            .get_piece_at(&piece.position)
            .expect("piece needs to be on the board");
        LegalMoveIterator::for_piece(self, piece_index)
    }
    pub fn boomable(&self, piece_index: usize) -> bool {
        assert!(piece_index < 8);
        let piece = &self.pieces[piece_index];
        // White
        if piece_index < 4 {
            self.pieces[4..].iter().any(|other| other.can_boom(piece))
        } else {
            self.pieces[..4].iter().any(|other| other.can_boom(piece))
        }
    }
    pub fn winner(&self) -> Option<Winner> {
        if let Some(color) = self.victory_by_concession {
            return Some(match color {
                Color::White => Winner::White,
                Color::Black => Winner::Black,
            });
        }
        let white_potential: u8 = self.pieces[0..4].iter().map(|p| u8::from(&p.height)).sum();
        let black_potential: u8 = self.pieces[4..8].iter().map(|p| u8::from(&p.height)).sum();
        if white_potential == 0 || black_potential == 0 {
            Some(match self.white_score.cmp(&self.black_score) {
                std::cmp::Ordering::Less => Winner::Black,
                std::cmp::Ordering::Equal => Winner::Draw,
                std::cmp::Ordering::Greater => Winner::White,
            })
        } else if self.white_score + white_potential < self.black_score {
            Some(Winner::Black)
        } else if self.black_score + black_potential < self.white_score {
            Some(Winner::White)
        } else {
            None
        }
    }
    pub fn compress(&self) -> (u64, u8, u8) {
        let pieces = ((self.pieces[7].as_u8() as u64) << 56)
            + ((self.pieces[6].as_u8() as u64) << 48)
            + ((self.pieces[5].as_u8() as u64) << 40)
            + ((self.pieces[4].as_u8() as u64) << 32)
            + ((self.pieces[3].as_u8() as u64) << 24)
            + ((self.pieces[2].as_u8() as u64) << 16)
            + ((self.pieces[1].as_u8() as u64) << 8)
            + (self.pieces[0].as_u8() as u64);
        (pieces, self.white_score, self.black_score)
    }
    pub fn decompress(pieces: u64, white_score: u8, black_score: u8) -> Board {
        let pieces = [
            Piece::from_u8(Color::White, (pieces & 0xff) as u8),
            Piece::from_u8(Color::White, ((pieces >> 8) & 0xff) as u8),
            Piece::from_u8(Color::White, ((pieces >> 16) & 0xff) as u8),
            Piece::from_u8(Color::White, ((pieces >> 24) & 0xff) as u8),
            Piece::from_u8(Color::Black, ((pieces >> 32) & 0xff) as u8),
            Piece::from_u8(Color::Black, ((pieces >> 40) & 0xff) as u8),
            Piece::from_u8(Color::Black, ((pieces >> 48) & 0xff) as u8),
            Piece::from_u8(Color::Black, ((pieces >> 56) & 0xff) as u8),
        ];
        Board {
            pieces,
            white_score,
            black_score,
            victory_by_concession: None,
        }
    }
}

pub struct LegalMoveIterator<'a> {
    board: &'a Board,
    piece_index: usize,
    max_piece_index: usize,
    dir_index: usize,
    distance: i8,
    has_scored: bool,
}
impl<'a> LegalMoveIterator<'a> {
    pub fn for_piece(board: &'a Board, piece_index: usize) -> LegalMoveIterator<'a> {
        let mut iter = LegalMoveIterator {
            board,
            piece_index,
            max_piece_index: piece_index,
            dir_index: 0,
            distance: 0,
            has_scored: false,
        };
        iter.advance();
        iter
    }
    pub fn for_color(board: &'a Board, color: &Color) -> LegalMoveIterator<'a> {
        let mut iter = match color {
            Color::White => LegalMoveIterator {
                board,
                piece_index: 0,
                max_piece_index: 3,
                dir_index: 0,
                distance: 0,
                has_scored: false,
            },
            Color::Black => LegalMoveIterator {
                board,
                piece_index: 4,
                max_piece_index: 7,
                dir_index: 0,
                distance: 0,
                has_scored: false,
            },
        };
        iter.advance();
        iter
    }
    fn advance(&mut self) {
        if self.piece_index < 8 {
            let piece = &self.board.pieces[self.piece_index];
            if self.distance == i8::from(&piece.height) {
                self.end_of_the_line();
            } else {
                self.distance += 1;
            }
        }
    }
    fn end_of_the_line(&mut self) {
        self.distance = 0;
        self.dir_index += 1;
        if self.dir_index >= 8 {
            self.dir_index = 0;
            self.piece_index += 1;
        }
        self.advance();
    }
}
impl<'a> Iterator for LegalMoveIterator<'a> {
    type Item = Move;

    fn next(&mut self) -> Option<Self::Item> {
        let piece_index = self.piece_index;
        if piece_index > self.max_piece_index {
            return None;
        }
        let piece = &self.board.pieces[piece_index];
        let (dx, dy) = [
            (-1, 0),
            (-1, -1),
            (0, -1),
            (1, -1),
            (1, 0),
            (1, 1),
            (0, 1),
            (-1, 1),
        ][self.dir_index];
        let (dx, dy) = (dx * self.distance, dy * self.distance);
        // Attempt movement in every direction
        // Set our initial range and position
        let position = piece.position.offset(dx, dy);
        match &position {
            PositionOffset::Valid(pos) => {
                if let Some(piece_at_pos_index) = self.board.get_piece_at(pos) {
                    self.end_of_the_line();
                    if self.board.pieces[piece_at_pos_index].color != piece.color {
                        Some(Move::Boom(piece_at_pos_index))
                    } else {
                        self.next()
                    }
                } else {
                    self.advance();
                    Some(Move::Zoom(piece_index, *pos))
                }
            }
            PositionOffset::ScoreZone(color) => {
                self.end_of_the_line();
                if !self.has_scored && color == &piece.color {
                    // We only want to have scoring as an option once, even if
                    // it's possible to score in multiple different ways
                    self.has_scored = true;
                    return Some(Move::Score(piece_index));
                }
                self.next()
            }
            PositionOffset::Invalid => {
                self.end_of_the_line();
                self.next()
            }
        }
    }
}

pub trait GamePlayer {
    fn decide(&mut self, board: &Board, color: &Color) -> Move;
}

pub struct Game<W: GamePlayer, B: GamePlayer> {
    board: Board,
    white_player: W,
    black_player: B,
    turn: Color,
}
impl<W, B> Game<W, B>
where
    W: GamePlayer,
    B: GamePlayer,
{
    pub fn new(white_player: W, black_player: B) -> Self {
        Self {
            board: Board::default(),
            white_player,
            black_player,
            turn: Color::White,
        }
    }
    pub fn play_turn(&mut self) -> Option<Winner> {
        let mov = match self.turn {
            Color::White => self.white_player.decide(&self.board, &self.turn),
            Color::Black => self.black_player.decide(&self.board, &self.turn),
        };
        self.apply_move(&mov);
        self.winner()
    }
    pub fn apply_move(&mut self, mov: &Move) {
        self.board = self.board.apply_move(mov);
        self.turn = self.turn.invert();
    }
    pub fn finish_game(&mut self) -> Winner {
        while self.board.winner().is_none() {
            self.play_turn();
        }
        self.winner().expect("there must be a winner")
    }
    pub fn winner(&self) -> Option<Winner> {
        self.board.winner()
    }

    pub fn board(&self) -> &Board {
        &self.board
    }

    pub fn turn(&self) -> &Color {
        &self.turn
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::{thread_rng, RngCore};

    macro_rules! zoom {
        ($index:expr, $square:tt) => {
            Move::Zoom($index, stringify!($square).try_into().unwrap())
        };
    }
    macro_rules! boom {
        ($index:expr) => {
            Move::Boom($index)
        };
    }
    macro_rules! score {
        ($index:expr) => {
            Move::Score($index)
        };
    }

    #[test]
    fn it_works() {
        let mut board = Board::default();
        /*
         * 8 ..bbbb..
         * 7 ........
         * 6 ........
         * 5 ........
         * 4 ........
         * 3 ........
         * 2 ........
         * 1 ..Xwww..
         *   abcdefgh
         */
        assert_eq!(
            board
                .legal_moves_for(&board.pieces[0])
                .collect::<Vec<Move>>(),
            vec![
                zoom!(0, b1),
                zoom!(0, a1),
                zoom!(0, d2),
                zoom!(0, e3),
                zoom!(0, f4),
                zoom!(0, c2),
                zoom!(0, c3),
                zoom!(0, c4),
                zoom!(0, b2),
                zoom!(0, a3),
            ]
        );
        /*
         * 8 ...bbb..
         * 7 ..b.....
         * 6 ..X.....
         * 5 ...w....
         * 4 ........
         * 3 ........
         * 2 ........
         * 1 ...www..
         *   abcdefgh
         */
        board.pieces[0].position = "c6".try_into().unwrap();
        board.pieces[1].position = "d5".try_into().unwrap();
        board.pieces[4].position = "c7".try_into().unwrap();
        assert_eq!(
            board
                .legal_moves_for(&board.pieces[0])
                .collect::<Vec<Move>>(),
            vec![
                // Left
                zoom!(0, b6),
                zoom!(0, a6),
                // Down Left
                zoom!(0, b5),
                zoom!(0, a4),
                // Down
                zoom!(0, c5),
                zoom!(0, c4),
                zoom!(0, c3),
                // Down Right is blocked
                // Right
                zoom!(0, d6),
                zoom!(0, e6),
                zoom!(0, f6),
                // Up Right
                zoom!(0, d7),
                boom!(6),
                // Up
                boom!(4),
                // Up Left
                zoom!(0, b7),
                zoom!(0, a8),
                score!(0)
            ]
        );
        board.pieces[0].boom();
        assert_eq!(
            board
                .legal_moves_for(&board.pieces[0])
                .collect::<Vec<Move>>(),
            vec![
                // Left
                zoom!(0, b6),
                zoom!(0, a6),
                // Down Left
                zoom!(0, b5),
                zoom!(0, a4),
                // Down
                zoom!(0, c5),
                zoom!(0, c4),
                // Down Right is blocked
                // Right
                zoom!(0, d6),
                zoom!(0, e6),
                // Up Right
                zoom!(0, d7),
                boom!(6),
                // Up
                boom!(4),
                // Up Left
                zoom!(0, b7),
                zoom!(0, a8),
            ]
        );
        board.pieces[0].boom();
        assert_eq!(
            board
                .legal_moves_for(&board.pieces[0])
                .collect::<Vec<Move>>(),
            vec![
                // Left
                zoom!(0, b6),
                // Down Left
                zoom!(0, b5),
                // Down
                zoom!(0, c5),
                // Down Right is blocked
                // Right
                zoom!(0, d6),
                // Up Right
                zoom!(0, d7),
                // Up
                boom!(4),
                // Up Left
                zoom!(0, b7),
            ]
        );
        board.pieces[0].height = Height::Dead;
        board.pieces[1].height = Height::Dead;
        board.pieces[2].height = Height::Dead;
        board.pieces[3].height = Height::Dead;
        board.pieces[4].height = Height::Dead;
        board.pieces[5].height = Height::Dead;
        board.pieces[6].height = Height::Dead;
        board.pieces[7].height = Height::Dead;
        assert_eq!(
            board.legal_moves(&Color::White).collect::<Vec<Move>>(),
            vec![]
        );
        assert_eq!(
            board.legal_moves(&Color::Black).collect::<Vec<Move>>(),
            vec![]
        );
    }
    #[test]
    fn fuzz_board_compression() {
        let mut rand = thread_rng();
        for _ in 0..100000 {
            let (pieces1, white_score1, black_score1) = (
                rand.next_u64(),
                (rand.next_u32() % 256) as u8,
                (rand.next_u32() % 256) as u8,
            );
            let board = Board::decompress(pieces1, white_score1, black_score1);
            let (pieces2, white_score2, black_score2) = board.compress();
            assert_eq!(pieces1, pieces2);
            assert_eq!(white_score1, white_score2);
            assert_eq!(black_score1, black_score2);
        }
    }
}
