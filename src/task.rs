use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader, Lines},
    path::PathBuf,
};

use dialoguer::{theme::ColorfulTheme, Confirm};

use crate::error::AocError;

pub type AocSolution = Vec<String>;
pub type AocIO = Lines<BufReader<File>>;
pub type AocInput = Lines<BufReader<File>>;

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

    fn example_input_path(&self) -> PathBuf {
        self.directory().join("example_in")
    }

    fn example_output_path(&self) -> PathBuf {
        self.directory().join("example_out")
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
        input: AocInput,
        phase: usize,
    ) -> Result<AocSolution, Box<dyn Error + Send + Sync>>;

    fn get_file_iterator(&self, path: PathBuf) -> Result<AocIO, AocError> {
        let file = File::open(&path).map_err(|io_err| AocError::IOReadError {
            input_path: path.to_string_lossy().to_string(),
            source: io_err,
        })?;
        Ok(BufReader::new(file).lines())
    }

    fn get_example_output(&self) -> Result<AocSolution, AocError> {
        self.get_file_iterator(self.example_output_path())?
            .collect::<Result<Vec<String>, std::io::Error>>()
            .map_err(|err| AocError::IOReadError {
                input_path: self.example_output_path().to_string_lossy().to_string(),
                source: err,
            })
    }

    fn solve_from_input_path(
        &self,
        input_path: PathBuf,
        phase: usize,
    ) -> Result<AocSolution, AocError> {
        let input = self.get_file_iterator(input_path.clone())?;
        let output =
            self.solution(input, phase)
                .map_err(|err| AocError::SolutionExecutionError {
                    input: input_path.to_string_lossy().to_string(),
                    source: err,
                })?;
        Ok(output)
    }

    fn solve(&self, phase: usize) -> Result<AocSolution, AocError> {
        let input_path = self.input_path();
        let output = self.solve_from_input_path(input_path, phase)?;
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

    fn run_example_test(&self, phase: usize) -> Result<AocTestResult, AocError> {
        let example_output = self.get_example_output()?;
        let output = self.solve_from_input_path(self.example_input_path(), phase)?;
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
            input: AocInput,
            _phase: usize,
        ) -> Result<AocSolution, Box<dyn Error + Send + Sync>> {
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
        let task = SumTask;
        assert_eq!(task.name(), String::from("Sum Task"));
    }

    #[test]
    fn sum_task_example_solution() {
        let task = SumTask;
        assert!(task.run_example_test(1).unwrap().passed)
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
