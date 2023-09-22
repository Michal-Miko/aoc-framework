pub mod error;
mod task;
pub mod traits;

use crossterm::style::Stylize;
use itertools::Itertools;
use prettydiff::diff_chars;

use error::AocError;
pub use task::{AocIO, AocInput, AocSolution, AocTask};

pub type BoxedAocTask = Box<dyn AocTask>;

const CROSS: &str = "✘";
const CHECKMARK: &str = "✔";
const DOT: &str = "·";

pub fn check_solved_tasks(
    tasks: Vec<BoxedAocTask>,
    phases_per_task: usize,
) -> Result<bool, AocError> {
    for (i, task) in tasks.iter().enumerate() {
        let example_test_result = task.run_example_test(1)?;
        if !example_test_result.passed {
            println!(
                "{} {} {} the example test.",
                CROSS.dark_red(),
                task.name().bold(),
                "failed".dark_red()
            );
            let result = example_test_result.output.into_iter();
            let expected = example_test_result.expected_output.into_iter();

            println!("Diff:");
            for lines in result.zip_longest(expected) {
                let (res_line, exp_line) = match lines {
                    itertools::EitherOrBoth::Both(r, e) => (r, e),
                    itertools::EitherOrBoth::Left(r) => (r, Default::default()),
                    itertools::EitherOrBoth::Right(e) => (Default::default(), e),
                };
                println!("{}", diff_chars(&res_line, &exp_line));
            }

            return Ok(false);
        } else {
            println!(
                "{} {} {} the example test!",
                CHECKMARK.dark_green(),
                task.name().bold(),
                "passed".dark_green()
            );
        }

        for phase in 1..=phases_per_task {
            let mut solved = task.phase_is_solved(phase);
            if !solved {
                let solution_output = task.solve(phase)?;
                println!(
                    "{} {} {}\n{}",
                    DOT.blue(),
                    "Output for phase".blue(),
                    phase.to_string().dark_yellow(),
                    solution_output.join("\n").blue()
                );
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
                return Ok(false);
            } else {
                println!(
                    "{} Phase {}/{} of {} {}!",
                    CHECKMARK.dark_green(),
                    phase.to_string().dark_yellow(),
                    phases_per_task.to_string().dark_yellow(),
                    task.name().bold(),
                    "passed".dark_green()
                );
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
