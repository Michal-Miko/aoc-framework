pub mod error;
mod task;
pub mod traits;

use std::path::PathBuf;

use crossterm::style::Stylize;
use itertools::Itertools;
use prettydiff::diff_chars;

use error::AocError;
pub use task::{AocSolution, AocStringIter, AocTask};

pub type BoxedAocTask = Box<dyn AocTask>;

const CROSS: &str = "✘";
const CHECKMARK: &str = "✔";
const DOT: &str = "·";

fn solve_task_phase(
    task: &BoxedAocTask,
    phase: usize,
    phases_per_task: usize,
) -> Result<bool, AocError> {
    let solution_output = task.solve(phase)?;
    println!(
        "{} {} {}:\n{}",
        DOT.blue(),
        "Solution for phase".blue(),
        phase.to_string().dark_yellow(),
        solution_output.join("\n").blue()
    );

    let mut solved = task.phase_is_solved(phase);

    if !solved {
        solved = task.ask_if_solved(phase)?;
    }

    if !solved {
        println!(
            "{} Phase {}/{} of {} {}.",
            CROSS.dark_red(),
            phase.to_string().dark_yellow(),
            phases_per_task.to_string().dark_yellow(),
            task.name().bold(),
            "failed".dark_red()
        );
        Ok(false)
    } else {
        println!(
            "{} Phase {}/{} of {} {}!",
            CHECKMARK.dark_green(),
            phase.to_string().dark_yellow(),
            phases_per_task.to_string().dark_yellow(),
            task.name().bold(),
            "passed".dark_green()
        );
        Ok(true)
    }
}

fn solve_example_phase(
    task: &BoxedAocTask,
    example: &(PathBuf, PathBuf),
    phase: usize,
) -> Result<bool, AocError> {
    let example_result = task.run_example_test(example, phase)?;
    let example_name = example
        .0
        .file_name()
        .map(|name| {
            let name_str = name.to_string_lossy();
            name_str[..name_str.len() - 3].to_owned()
        })
        .unwrap_or("<failed to parse example name>".into());

    if phase == 1 && !example_result.passed {
        println!(
            "{} {} {} the {} test in phase {}.",
            CROSS.dark_red(),
            task.name().bold(),
            "failed".dark_red(),
            example_name.bold(),
            phase.to_string().dark_yellow(),
        );
        let result = example_result.output.clone().into_iter();
        let expected = example_result.expected_output.into_iter();

        println!("Diff:");
        for lines in result.zip_longest(expected) {
            let (res_line, exp_line) = match lines {
                itertools::EitherOrBoth::Both(r, e) => (r, e),
                itertools::EitherOrBoth::Left(r) => (r, Default::default()),
                itertools::EitherOrBoth::Right(e) => (Default::default(), e),
            };
            println!("{}", diff_chars(&res_line, &exp_line));
        }
        // Exit early since we printed the diff already and there is no need to print the output
        return Ok(false);
    } else if phase == 1 {
        println!(
            "{} {} {} the {} test in phase {}!",
            CHECKMARK.dark_green(),
            task.name().bold(),
            "passed".dark_green(),
            example_name.clone().bold(),
            phase.to_string().dark_yellow(),
        );
    }

    println!(
        "{} {} {} {} {}:\n{}",
        DOT.cyan(),
        "Output of the".cyan(),
        example_name.bold(),
        "test in phase".cyan(),
        phase.to_string().dark_yellow(),
        example_result.output.join("\n").cyan()
    );

    Ok(true)
}

pub fn check_solved_tasks(
    tasks: Vec<BoxedAocTask>,
    phases_per_task: usize,
) -> Result<bool, AocError> {
    for (i, task) in tasks.iter().enumerate() {
        for phase in 1..=phases_per_task {
            for example in task.example_paths()? {
                if !solve_example_phase(task, &example, phase)? {
                    return Ok(false);
                }
            }

            if !solve_task_phase(task, phase, phases_per_task)? {
                return Ok(false);
            }
        }

        println!(
            "{}",
            format!(
                "{} Task {} - {}/{} done!",
                CHECKMARK,
                task.name(),
                i + 1,
                tasks.len()
            )
            .dark_green()
        );
        println!("=================================================");
    }

    println!(
        "{}",
        "🚀🚀🚀✔️ All tasks have been completed! ✔️🚀🚀🚀".dark_green()
    );
    Ok(true)
}
