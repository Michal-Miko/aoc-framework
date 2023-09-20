use std::fmt::Display;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum AocError<E: Display> {
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
    SolutionExecutionError { input: String, source: E },
    #[error("Failed to get user input")]
    UserInterractionError { source: std::io::Error },
}

// pub enum GitlabError {
//     #[error("Your GitLab Access Token is invalid: [{status}] {message}\nRun jit reset-gitlab to change it.")]
//     InvalidAccessToken { status: StatusCode, message: String },
//     #[error("Deserialization error")]
//     DeserializationError(#[from] serde_json::Error),
//     #[error("Request error")]
//     RequestError(#[from] reqwest::Error),
//     #[error("GitLab Api error: {status} - {message}")]
//     GitlabApiError { status: StatusCode, message: String },
//     #[error("Failed to resolve the remote repository path from current directory")]
//     InvalidRemoteError,
// }
