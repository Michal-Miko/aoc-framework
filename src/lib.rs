pub mod error;
mod task;

use error::AocError;
pub use task::{AocIO, AocInput, AocSolution, AocTask};

pub type BoxedAocTask = Box<dyn AocTask>;

pub fn check_solved_tasks(
    tasks: Vec<BoxedAocTask>,
    phases_per_task: usize,
) -> Result<bool, AocError> {
    for (i, task) in tasks.iter().enumerate() {
        let example_test_result = task.run_example_test()?;
        if !example_test_result.passed {
            println!("✘ {} failed the example test.", task.name());
            println!(
                "Expected output:\n{:#?}",
                example_test_result.expected_output
            );
            println!("Solution output:\n{:#?}", example_test_result.output);
            return Ok(false);
        } else {
            println!("✔ {} passed the example test!", task.name());
        }

        let solution_output = task.solve()?;
        println!("· Solution attempt:\n{}", solution_output.join("\n"));

        for phase in 1..=phases_per_task {
            let solved = task.ask_if_solved(phase)?;
            if !solved {
                println!(
                    "✘ Phase {phase}/{phases_per_task} of {} failed.",
                    task.name()
                );
                return Ok(false);
            } else {
                println!(
                    "✔ Phase {phase}/{phases_per_task} of {} solved!",
                    task.name()
                );
            }
        }

        println!("✔ Task {} - {}/{} done!", task.name(), i + 1, tasks.len());
        println!("=================================================");
    }

    println!("🚀🚀🚀✔️ All tasks have been completed! ✔️🚀🚀🚀");
    Ok(true)
}
