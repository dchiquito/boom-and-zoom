mod forward_random;
mod go_fast;
mod heuristic;
mod random;

pub use forward_random::ForwardRandomPlayer;
pub use go_fast::{GoFastHeuristic, GoFasterHeuristic};
pub use heuristic::HeuristicPlayer;
pub use random::RandomPlayer;
