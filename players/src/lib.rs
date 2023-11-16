mod forward_random;
mod genius;
mod go_fast;
mod heuristic;
mod random;

pub use forward_random::ForwardRandomPlayer;
pub use genius::GeniusHeuristic;
pub use go_fast::{GoFastHeuristic, GoFasterHeuristic};
pub use heuristic::HeuristicPlayer;
pub use random::RandomPlayer;
