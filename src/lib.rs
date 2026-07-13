pub mod algo;
pub mod io;
pub mod math;
pub mod search;

pub use algo::{find_two_sum_indices, max_matches_with_tolerance, min_pairs_with_limit};
pub use io::{Output, Scanner, Writable};
pub use math::{factorial, mod_pow, next_permutation, ModMatrix2x2};
pub use search::{dfs_recursive, in_bounds, CARDINAL_DIRS, KNIGHT_DIRS};
