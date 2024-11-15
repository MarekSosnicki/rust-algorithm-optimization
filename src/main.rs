use std::fs;
use std::path::PathBuf;

use clap::Parser;

use table_problem::algorithm::v5::solve;
use table_problem::objective_value_calculator::v1::ObjectiveValueCalculator;
use table_problem::problem::ProblemDescription;
use table_problem::validator::validate_solution;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    input_dir: PathBuf,
}

fn main() {
    let args = Args::parse();

    let dir = fs::read_dir(args.input_dir).unwrap();

    let mut results = vec![];
    for file in dir {
        let filename_path = file.unwrap().path();
        let file_content = std::fs::read_to_string(&filename_path).unwrap();
        let problem: ProblemDescription = serde_json::from_str(&file_content).unwrap();
        let calculator = ObjectiveValueCalculator::new(&problem);

        println!("--- Running algorithm for {}", filename_path.display());
        let result = solve(&problem, chrono::Duration::seconds(1));
        println!(
            "Result value {}",
            calculator.solution_value(&result.solution)
        );
        validate_solution(&problem, &result.solution);
        results.push(result);
    }

    println!("Successfully calculated");

    let avg_iterations = results.iter().map(|r| r.no_of_iterations).sum::<usize>() / results.len();
    let avg_time = results
        .iter()
        .map(|r| r.elapsed.num_milliseconds())
        .sum::<i64>()
        / results.len() as i64;

    println!(
        "Avg iterations {}, avg elapsed {}ms",
        avg_iterations, avg_time
    );
}
