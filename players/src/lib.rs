mod forward_random;
mod genius;
mod go_fast;
mod heuristic;
mod minmax;
mod naive;
mod random;

pub use forward_random::ForwardRandomPlayer;
pub use genius::GeniusHeuristic;
pub use go_fast::{GoFastHeuristic, GoFasterHeuristic};
pub use heuristic::{HResult, Heuristic, HeuristicPlayer};
pub use minmax::MinMaxPlayer;
pub use naive::NaiveHeuristic;
pub use random::RandomPlayer;
