mod error;
mod task;

pub use task::AocTask;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}
