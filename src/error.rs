use std::error::Error;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AocError {
    #[error(
        "Failed to mark the task {task_name} as solved by creating the solved file: {solved_path}"
    )]
    MarkSolvedError {
        task_name: String,
        solved_path: String,
        source: std::io::Error,
    },
    #[error("Failed to read the IO file: {input_path}")]
    IOReadError {
        input_path: String,
        source: std::io::Error,
    },
    #[error("Your solution failed to pass the example test")]
    InvalidExampleOutput {
        input: String,
        output: String,
        expected_output: String,
    },
    #[error("Your solution returned an error: {source}")]
    SolutionExecutionError {
        input: String,
        source: Box<dyn Error + Send + Sync>,
    },
    #[error("Failed to get user input")]
    UserInterractionError { source: std::io::Error },
}
