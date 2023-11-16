use baz_core::{Board, Color, Height, Move, Piece};
use num::{rational::Rational32, ToPrimitive};

use crate::heuristic::{Heuristic, HeuristicPlayer};
/**
 * Let's put down some thoughts about how this genius heuristic will work.
 *
 * The fundamental concept here is point density. A piece is worth 1-3 points and will take 1-8
 * turns to cash out. If you commit all of your turns to running a piece to the end of the board
 * with no interruptions, the point density is a measurement of how efficient a use of time that
 * is. The density is the points divided by the turns required to make it to the score zone, in
 * units of points/turn.
 *
 * For example, a 3 piece on the back rank is worth 3 points / 3 turns = 1 p/t. A 2 piece on the
 * back rank is work 2 points / 4 turns = 1/2 p/t. A 1 piece on the back rank is worth 1 point / 8
 * turns = 1/8 p/t. In general, bigger pieces and pieces closer to the score zone are worth more.
 *
 * Now, consider the case where White has four 1 pieces ready to score, while Black has a single 3
 * piece ready to score. Clearly White has greater scoring potential (4 points to Black's 3), but
 * as soon as Black scores the game is over. We need a way to adjust our expectation of White's
 * result.
 *
 * Consider this algorithm: Determine how many turns it would take each player to naively score all
 * of their pieces (in the example above, White 4 and Black 1). Choose the minimum total turn
 * count (1). Because White doesn't have sufficient time, order White's pieces by point density
 * descending and disregard the pieces that there is not enough time to score (1 turn means White
 * scores 1 point). Black won the race, and so gets to score all their potential points (Black
 * scores 3 points). Subtract Black's point estimate from White's to get the differential (-2,
 * meaning Black wins by 2).
 *
 * (Obviously, add any points already scored to the potential points. This text assumes no points
 * have been scored yet.)
 *
 * That's silly. Consider this instead: Average White's point density (1 p/t + 1 p/t + 1 p/t + 1
 * p/t / 4 = 1 p/t), and multiply it by the number of turns it will take Black to score (1 turn) to
 * get the expected White score (1 point). This will sometimes result in weird fractional score
 * estimates, but is purely arithmetic and doesn't involve any sorting or looping.
 *
 * What would a fractional score look like? Consider White has a 2 piece on the back rank (4 turns
 * to score, 1/2 p/t) and a 1 piece ready to score (1 p/t), and Black has a 3 piece on the back
 * rank (1 p/t). White's clock is 5 turns while Black's is 3. White's average is 3/4 p/t,
 * multiplied by 3 turns is 9/4 p/t = 2.25 points.
 *
 * I think the fractional representation is better. This score estimation is based on a idylic and
 * idealized race to the finish, so the fraction represents how close a board state is to a
 * breakpoint.
 *
 * Speaking of idealizations, let's think about booming. Suppose White has a 3 piece on the back
 * rank, and Black has a piece that could boom it if Black so chose. Consider booming. This cannot
 * affect your own score, only lower the potential score (and also limit the options) of the
 * opponent. Consider zooming. This increases your own score, and I suppose it also might change
 * the time constraints impacting the final score of your opponent. Now consider our standoff. If
 * Black decides to address the situation first, Black can either boom the piece or disengage.
 * Booming a 3 piece is basically always good, so let's just assume if Black chooses to disengage
 * it's a game winning kind of move. If White decides to address the situation first, then White
 * can either zoom out of range (let's assume there's a way to do that) or possibly boom the black
 * piece shorter so it cannot reach the White piece anymore. If White booms Black, that doesn't
 * affect White's score, which is what we're trying to calculate, so we'll just ignore that
 * possibility until we are calculating Black's score.
 *
 * Ok, so for our standoff we are making some assumptions and limiting ourselves to two options:
 * Black booms White, or White zooms away. If Black plays first, the boom. If White plays first,
 * the zoom. That's a 50/50 choice either way. The actual distribution is dependent on the rest of
 * the board state, and we're not doing any speculative execution right now.
 *
 * So, 50% of the time, White gets to zoom the piece as normal, for a point density of 3 points / 3
 * turns = 1 p/t. The other 50% of the time, Black booms the piece first, changing the density to 2
 * points / 4 turns = 1/2 p/t. We can average those densities to get 3/4 p/t. That's pretty good;
 * because Black has the option on the board to boom a White piece, that lower's the potential
 * point value of that piece.
 *
 * What about booming a 1 piece? 50% of the time, it's point density is 1/turns p/t. The other 50%
 * of the time, it's point density is 0 points / 0 turns, which is not a number. If a 1 piece would
 * get boomed, it also needs to be removed from the average and it's turns removed from the clock.
 *
 * Ok. There's two possiblities: If Black booms White's 1 piece, it will either decrease White's
 * potential score, or increase it. If it would increase White's potential score, then Black would
 * not do that booming. So if we can determine that the boom would be detrimental to Black, then we
 * don't need to do anything.
 *
 * Hmm. If a piece gets boomed, it increases the turn clock, which changes the final calculation in
 * addition to the point density of the piece in question. Obviously, we simply average the turn
 * clock as well! Everything is resolved.
 *
 * This is silly. There are only four pieces, so only 16 permutations of boomability to check. Just
 * check every possible combination of pieces boomed. If those pieces can't actually be boomed,
 * then don't actually boom them. Optionally, check that booming a 1 piece actually lowers the
 * score; if it doesn't, just use the unboomed value again. Average the 16 point densities and turn
 * counts to get a final point density and turn count for the color, then do the same for the other
 * player and scale by the faster one to get the final score estimates.
 *
 * Of course, this is just a heuristic. We must rely on a search tree prune away poor assumptions.
 * For example, say that Black has a defensive position such that White cannot advance any pieces
 * without getting boomed. Our heuristic does not consider the possibility of moving into range of
 * a boom while rushing to score, and so might give White the advantage if White has more higher
 * value pieces because none can be boomed at the present moment. A search tree would determine
 * that the heuristic was overly optimistic if it cannot find a sequence to justify that optimism.
 *
 * Still, I think it's probably a good enough heuristic for now.
 */

// Try to end the game as soon as possible
pub struct GeniusHeuristic();

impl Heuristic<Rational32> for GeniusHeuristic {
    fn evaluate(&mut self, board: &baz_core::Board, color: &baz_core::Color) -> Rational32 {
        let (mut our_score, our_turns) = GeniusHeuristic::estimate_score_and_turns(board, color);
        let (mut their_score, their_turns) =
            GeniusHeuristic::estimate_score_and_turns(board, &color.invert());
        let min_turns = our_turns.min(their_turns);
        if our_turns > 0.into() {
            our_score = our_score * min_turns / our_turns;
        }
        if their_turns > 0.into() {
            their_score = their_score * min_turns / their_turns;
        }
        let estimate = our_score - their_score;
        println!(
            "Our score: {} Their score: {}, turns 'till the end: {} estimate: {}",
            our_score.to_f64().unwrap(),
            their_score.to_f64().unwrap(),
            min_turns.to_f64().unwrap(),
            estimate.to_f64().unwrap(),
        );
        estimate
    }
}

impl GeniusHeuristic {
    pub fn player(depth: usize) -> HeuristicPlayer<GeniusHeuristic, Rational32> {
        HeuristicPlayer::new(GeniusHeuristic(), depth)
    }
    fn estimate_score_and_turns(board: &Board, color: &Color) -> (Rational32, Rational32) {
        // Vec of all pieces of the color that can be boomed by the enemy
        let boomables: Vec<Piece> = board
            // Check all the legal moves for the opponent
            .legal_moves(&color.invert())
            .iter()
            // Filter out only the booms, map to the target of the boom
            .filter_map(|m| match m {
                &Move::Boom(index) => Some(index),
                _ => None,
            })
            // Map to the piece being boomed
            .map(|index| board.get_piece(index))
            .cloned()
            .collect();
        // height of the piece, distance to the score zone, if the piece can be boomed
        let piece_data: Vec<(i8, i8, bool)> = board
            .pieces
            .iter()
            .filter(|p| &p.color == color && p.height != Height::Dead)
            .map(|p| (p, boomables.iter().any(|p2| p.position == p2.position)))
            .map(|(p, boomable)| {
                (
                    Into::<i8>::into(&p.height),
                    if color == &Color::White {
                        7 - p.position.y()
                    } else {
                        p.position.y()
                    },
                    boomable,
                )
            })
            .collect();
        let scenarios = (1 << piece_data.len()) as usize;
        // TODO find an effective way to trim out scenarios where a 1 piece is boomed to
        // our benefit
        let (sum_score, sum_turns) = (0..scenarios)
            .map(|mask| GeniusHeuristic::score_and_turns(mask, &piece_data))
            .fold((0, 0), |(sum_score, sum_turns), (score, turns)| {
                (sum_score + score as i32, sum_turns + turns as i32)
            });
        let average_score = Rational32::new_raw(sum_score, scenarios as i32);
        let average_turns = Rational32::new_raw(sum_turns, scenarios as i32);
        if color == &Color::White {
            (average_score + board.white_score as i32, average_turns)
        } else {
            (average_score + board.black_score as i32, average_turns)
        }
    }
    fn score_and_turns(mask: usize, data: &[(i8, i8, bool)]) -> (i8, i8) {
        data.iter()
            .enumerate()
            .map(|(index, (height, distance, boomable))| {
                if *boomable && (mask << index) & 1 == 1 {
                    (height - 1, *distance)
                } else {
                    (*height, *distance)
                }
            })
            .filter(|(height, _distance)| *height > 0)
            .map(|(height, distance)| (height, (distance + height - 1) / height))
            .fold((0, 0), |(score, sum_turns), (height, turns)| {
                (score + height, sum_turns + turns)
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn score_and_turns_no_pieces() {
        assert_eq!(GeniusHeuristic::score_and_turns(0, &[]), (0, 0));
    }
    #[test]
    fn score_and_turns_one_pieces() {
        assert_eq!(
            GeniusHeuristic::score_and_turns(0, &[(1, 1, false)]),
            (1, 1)
        );
        assert_eq!(GeniusHeuristic::score_and_turns(0, &[(1, 1, true)]), (1, 1));
    }
    #[test]
    fn score_and_turns_turn_calculation() {
        let matrix = [
            // Height 1
            ((1, 1, false), (1, 1)),
            ((1, 2, false), (1, 2)),
            ((1, 3, false), (1, 3)),
            ((1, 4, false), (1, 4)),
            ((1, 5, false), (1, 5)),
            ((1, 6, false), (1, 6)),
            ((1, 7, false), (1, 7)),
            ((1, 8, false), (1, 8)),
            // Height 2
            ((2, 1, false), (2, 1)),
            ((2, 2, false), (2, 1)),
            ((2, 3, false), (2, 2)),
            ((2, 4, false), (2, 2)),
            ((2, 5, false), (2, 3)),
            ((2, 6, false), (2, 3)),
            ((2, 7, false), (2, 4)),
            ((2, 8, false), (2, 4)),
            // Height 3
            ((3, 1, false), (3, 1)),
            ((3, 2, false), (3, 1)),
            ((3, 3, false), (3, 1)),
            ((3, 4, false), (3, 2)),
            ((3, 5, false), (3, 2)),
            ((3, 6, false), (3, 2)),
            ((3, 7, false), (3, 3)),
            ((3, 8, false), (3, 3)),
        ];
        for (datum, expected) in matrix {
            assert_eq!(GeniusHeuristic::score_and_turns(0, &[datum]), expected);
        }
    }
    #[test]
    fn score_and_turns_no_boom() {
        assert_eq!(
            GeniusHeuristic::score_and_turns(0, &[(3, 1, false), (2, 1, false), (1, 1, true)]),
            (6, 3)
        );
    }
    #[test]
    fn score_and_turns_one_boom() {
        assert_eq!(
            GeniusHeuristic::score_and_turns(0b1, &[(1, 1, true)]),
            (0, 0)
        );
        assert_eq!(
            GeniusHeuristic::score_and_turns(0b1, &[(2, 1, true)]),
            (1, 1)
        );
        assert_eq!(
            GeniusHeuristic::score_and_turns(0b1, &[(3, 1, true)]),
            (2, 1)
        );
        assert_eq!(
            GeniusHeuristic::score_and_turns(0b1, &[(3, 8, true)]),
            (2, 4)
        );
    }
}
