use std::{
    collections::HashMap,
    error::Error,
    fs::File,
    io::{BufRead, BufReader, Lines},
    path::PathBuf,
};

use dialoguer::{theme::ColorfulTheme, Confirm};
use itertools::{Itertools, ProcessResults};

use crate::error::AocError;

pub type AocSolution = Vec<String>;
pub type AocStringIter<'src> = ProcessResults<'src, Lines<BufReader<File>>, std::io::Error>;
pub type AocResultStringIter = Lines<BufReader<File>>;

#[derive(Debug)]
pub struct AocTestResult {
    pub passed: bool,
    pub output: AocSolution,
    pub expected_output: AocSolution,
}

pub trait AocTask {
    fn directory(&self) -> PathBuf;

    fn title_case(&self, string: String) -> String {
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

    fn name(&self) -> String {
        self.directory()
            .file_name()
            .map(|os_str| os_str.to_string_lossy().to_string())
            .map(|string| string.replace('_', " "))
            .map(|string| self.title_case(string))
            .unwrap_or("Unknown Task".to_owned())
    }

    fn example_paths(&self) -> Result<Vec<(PathBuf, PathBuf)>, AocError> {
        let example_directory = self.directory();
        let task_files = example_directory
            .read_dir()
            .map_err(|err| AocError::MissingExample {
                directory: example_directory.to_string_lossy().to_string(),
                source: err,
            })?
            .collect::<Result<Vec<_>, _>>()
            .map_err(|err| AocError::MissingExample {
                directory: example_directory.to_string_lossy().to_string(),
                source: err,
            })?;

        let example_files = task_files
            .into_iter()
            .filter(|file| file.file_name().to_string_lossy().contains("example"));

        let mut example_inputs = HashMap::new();
        let mut example_outputs = HashMap::new();

        for file in example_files {
            let filename = file.file_name().to_string_lossy().to_string();
            if filename.ends_with("_in") {
                example_inputs.insert(filename, file);
            } else if filename.ends_with("_out") {
                example_outputs.insert(filename, file);
            }
        }

        let mut example_pairs = vec![];
        for (input_filename, input_file) in example_inputs {
            let mut output_filename = input_filename.clone();
            output_filename.replace_range(output_filename.len() - 3.., "_out");
            let output_path = example_directory.join(output_filename);

            if output_path.is_file() && input_file.path().is_file() {
                example_pairs.push((input_file.path().to_owned(), output_path));
            }
        }

        Ok(example_pairs)
    }

    fn input_path(&self) -> PathBuf {
        self.directory().join("in")
    }

    fn solved_phase_path(&self, phase: usize) -> PathBuf {
        self.directory().join(format!(".solved_phase_{phase}"))
    }

    fn phase_is_solved(&self, phase: usize) -> bool {
        self.solved_phase_path(phase).is_file()
    }

    fn mark_phase_as_solved(&self, phase: usize) -> Result<(), AocError> {
        let solved_path = self.solved_phase_path(phase);
        File::create(&solved_path).map_err(|io_err| AocError::MarkSolvedError {
            task_name: self.name(),
            solved_path: solved_path.to_string_lossy().to_string(),
            source: io_err,
        })?;
        Ok(())
    }

    fn solution(
        &self,
        input: AocStringIter,
        phase: usize,
    ) -> Result<AocSolution, Box<dyn Error + Send + Sync>>;

    fn get_file_iterator(&self, path: &PathBuf) -> Result<AocResultStringIter, AocError> {
        let file = File::open(path).map_err(|io_err| AocError::IOReadError {
            path: path.to_string_lossy().to_string(),
            source: io_err,
        })?;
        Ok(BufReader::new(file).lines())
    }

    fn get_file_output(&self, path: &PathBuf) -> Result<AocSolution, AocError> {
        self.get_file_iterator(path)?
            .collect::<Result<Vec<String>, std::io::Error>>()
            .map_err(|err| AocError::IOReadError {
                path: path.to_string_lossy().to_string(),
                source: err,
            })
    }

    fn solve_from_input_path(
        &self,
        input_path: &PathBuf,
        phase: usize,
    ) -> Result<AocSolution, AocError> {
        let input = self.get_file_iterator(input_path)?;
        let output = input
            .process_results(|lines| {
                self.solution(lines, phase)
                    .map_err(|err| AocError::SolutionExecutionError {
                        input_path: input_path.to_string_lossy().to_string(),
                        source: err,
                    })
            })
            .map_err(|line_read_error| AocError::IOReadError {
                path: input_path.to_string_lossy().to_string(),
                source: line_read_error,
            })??;
        Ok(output)
    }

    fn solve(&self, phase: usize) -> Result<AocSolution, AocError> {
        let input_path = self.input_path();
        let output = self.solve_from_input_path(&input_path, phase)?;
        Ok(output)
    }

    fn solutions_match(&self, s1: &AocSolution, s2: &AocSolution) -> bool {
        let matches = s1
            .iter()
            .zip(s2.iter())
            .filter(|&(a, b)| a.trim() == b.trim())
            .count();

        matches == s1.len() && matches == s2.len()
    }

    fn run_example_test(
        &self,
        io_pair: &(PathBuf, PathBuf),
        phase: usize,
    ) -> Result<AocTestResult, AocError> {
        let example_output = self.get_file_output(&io_pair.1)?;
        let output = self.solve_from_input_path(&io_pair.0, phase)?;
        Ok(AocTestResult {
            passed: self.solutions_match(&example_output, &output),
            output,
            expected_output: example_output,
        })
    }

    fn ask_if_solved(&self, phase: usize) -> Result<bool, AocError> {
        let solved = Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt(format!("Is phase {phase} of the task solved?"))
            .interact()
            .map_err(|io_err| AocError::UserInterractionError { source: io_err })?;

        if solved {
            self.mark_phase_as_solved(phase)?;
            Ok(true)
        } else {
            Ok(false)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct SumTask;

    impl AocTask for SumTask {
        fn directory(&self) -> PathBuf {
            PathBuf::from("tests/sum_task")
        }

        fn solution(
            &self,
            input: AocStringIter,
            _phase: usize,
        ) -> Result<AocSolution, Box<dyn Error + Send + Sync>> {
            let mut answers = vec![];
            for line in input {
                answers.push(
                    line.split_whitespace()
                        .map(|num| num.parse::<i32>().unwrap_or(0))
                        .sum::<i32>()
                        .to_string(),
                );
            }
            Ok(answers)
        }
    }

    #[test]
    fn sum_task_name() {
        let task = SumTask;
        assert_eq!(task.name(), String::from("Sum Task"));
    }

    #[test]
    fn sum_task_example_solutions() {
        let task = SumTask;
        let examples = task.example_paths().unwrap();
        assert!(examples.len() > 1);
        for example_path_pair in examples {
            assert!(task.run_example_test(&example_path_pair, 1).unwrap().passed);
        }
    }

    #[test]
    fn sum_task_solution() {
        let task = SumTask;
        let solution = task.solve(1).unwrap();
        let expected_output = vec![7.to_string(), 12.to_string(), 289197.to_string()];
        assert!(task.solutions_match(&solution, &expected_output))
    }

    #[test]
    fn sum_task_solved() {
        let task = SumTask;
        let phase = 1usize;
        let solved_path = task.solved_phase_path(phase);
        if solved_path.exists() {
            std::fs::remove_file(solved_path).unwrap();
        }
        assert!(!task.phase_is_solved(phase));

        task.mark_phase_as_solved(phase).unwrap();
        assert!(task.phase_is_solved(phase));
    }
}
