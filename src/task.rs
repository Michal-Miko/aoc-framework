use std::{
    fmt::Display,
    fs::File,
    io::{BufRead, BufReader, Lines},
    path::PathBuf,
};

use dialoguer::Confirm;

use crate::error::AocError;

pub type AocSolution = Vec<String>;
pub type AocIO = Lines<BufReader<File>>;
pub type AocInput = AocIO;

pub trait AocTask<E: Display> {
    fn directory() -> PathBuf;

    fn title_case(string: String) -> String {
        string
            .split_whitespace()
            .map(|token| {
                let mut chars = token.chars();
                match chars.next() {
                    None => String::new(),
                    Some(c) => c.to_uppercase().chain(chars).collect(),
                }
            })
            .collect::<Vec<_>>()
            .join(" ")
    }

    fn name() -> String {
        Self::directory()
            .file_name()
            .map(|os_str| os_str.to_string_lossy().to_string())
            .map(|string| string.replace('_', " "))
            .map(Self::title_case)
            .unwrap_or("Unknown Task".to_owned())
    }

    fn example_input_path() -> PathBuf {
        Self::directory().join("example_in")
    }

    fn example_output_path() -> PathBuf {
        Self::directory().join("example_out")
    }

    fn input_path() -> PathBuf {
        Self::directory().join("in")
    }

    fn solved_path() -> PathBuf {
        Self::directory().join(".solved")
    }

    fn is_solved() -> bool {
        Self::solved_path().is_file()
    }

    fn mark_as_solved() -> Result<(), AocError<E>> {
        let solved_path = Self::solved_path();
        File::create(&solved_path).map_err(|io_err| AocError::<E>::MarkSolvedError {
            task_name: Self::name(),
            solved_path: solved_path.to_string_lossy().to_string(),
            source: io_err,
        })?;
        Ok(())
    }

    fn solution(input: AocInput) -> Result<AocSolution, E>;

    fn get_file_iterator(path: PathBuf) -> Result<AocIO, AocError<E>> {
        let file = File::open(&path).map_err(|io_err| AocError::<E>::IOReadError {
            input_path: path.to_string_lossy().to_string(),
            source: io_err,
        })?;
        Ok(BufReader::new(file).lines())
    }

    fn get_example_output() -> Result<AocSolution, AocError<E>> {
        Self::get_file_iterator(Self::example_output_path())?
            .collect::<Result<Vec<String>, std::io::Error>>()
            .map_err(|err| AocError::<E>::IOReadError {
                input_path: Self::example_output_path().to_string_lossy().to_string(),
                source: err,
            })
    }

    fn solve_from_input_path(input_path: PathBuf) -> Result<AocSolution, AocError<E>> {
        let input = Self::get_file_iterator(input_path.clone())?;
        let output =
            Self::solution(input).map_err(|err| AocError::<E>::SolutionExecutionError {
                input: input_path.to_string_lossy().to_string(),
                source: err,
            })?;
        Ok(output)
    }

    fn solve() -> Result<AocSolution, AocError<E>> {
        let input_path = Self::input_path();
        let output = Self::solve_from_input_path(input_path)?;
        Ok(output)
    }

    fn compare_solutions(s1: &AocSolution, s2: &AocSolution) -> bool {
        let matches = s1
            .iter()
            .zip(s2.iter())
            .filter(|&(a, b)| a.trim() == b.trim())
            .count();

        matches == s1.len() && matches == s2.len()
    }

    fn passes_example_test() -> Result<bool, AocError<E>> {
        let example_output = Self::get_example_output()?;
        let output = Self::solve_from_input_path(Self::example_input_path())?;
        Ok(Self::compare_solutions(&example_output, &output))
    }

    fn check_if_solved() -> Result<(), AocError<E>> {
        let solved = Confirm::new()
            .with_prompt("Is the task solved?")
            .interact()
            .map_err(|io_err| AocError::<E>::UserInterractionError { source: io_err })?;

        if solved {
            Self::mark_as_solved()?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct SumTask;

    impl AocTask<String> for SumTask {
        fn directory() -> PathBuf {
            PathBuf::from("tests/sum_task")
        }

        fn solution(input: AocInput) -> Result<AocSolution, String> {
            let mut answers = vec![];
            for line in input {
                if let Ok(string) = line {
                    answers.push(
                        string
                            .split_whitespace()
                            .map(|num| num.parse::<i32>().unwrap_or(0))
                            .sum::<i32>()
                            .to_string(),
                    );
                }
            }
            Ok(answers)
        }
    }

    #[test]
    fn sum_task_name() {
        assert_eq!(SumTask::name(), String::from("Sum Task"));
    }

    #[test]
    fn sum_task_example_solution() {
        assert!(SumTask::passes_example_test().unwrap())
    }

    #[test]
    fn sum_task_solution() {
        let solution = SumTask::solve().unwrap();
        let expected_output = vec![7.to_string(), 12.to_string(), 289197.to_string()];
        assert!(SumTask::compare_solutions(&solution, &expected_output))
    }

    #[test]
    fn sum_task_solved() {
        let solved_path = SumTask::solved_path();
        if solved_path.exists() {
            std::fs::remove_file(solved_path).unwrap();
        }
        assert!(!SumTask::is_solved());

        SumTask::mark_as_solved().unwrap();
        assert!(SumTask::is_solved());
    }
}
