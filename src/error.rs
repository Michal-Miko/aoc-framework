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
    #[error("Failed to read the IO file: {path}")]
    IOReadError {
        path: String,
        source: std::io::Error,
    },
    #[error("Could not find any example inputs/outputs in the folder {directory}. Expected at least one pair of files that start with `example_` and end with `_in`/`_out`. Error: {source}")]
    MissingExample {
        directory: String,
        source: std::io::Error,
    },
    #[error("Your solution returned an error: {source}")]
    SolutionExecutionError {
        input_path: String,
        source: Box<dyn Error + Send + Sync>,
    },
    #[error("Failed to get user input")]
    UserInterractionError { source: dialoguer::Error },
}
