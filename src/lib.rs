pub mod error;
mod task;

pub use task::{AocIO, AocInput, AocSolution, AocTask};

pub fn add(left: usize, right: usize) -> usize {
    left + right
}
