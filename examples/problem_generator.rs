use clap::Parser;
use itertools::Itertools;
use rand::{thread_rng, Rng};
use std::path::PathBuf;
use table_problem::generator::generate_problem;
use table_problem::problem::MAX_PEOPLE_FOR_TABLE;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    output_dir: PathBuf,

    #[arg(short, long)]
    no_of_problems: usize,

    #[arg(long, default_value_t = 100)]
    min_people: usize,

    #[arg(long, default_value_t = 200)]
    max_people: usize,
}

fn main() {
    let args = Args::parse();

    std::fs::create_dir_all(&args.output_dir).expect("Failed to create output dir");

    for problem_id in 0..args.no_of_problems {
        let no_of_people = thread_rng().gen_range(args.min_people..args.max_people);
        let min_no_of_table_days = no_of_people / MAX_PEOPLE_FOR_TABLE;
        // no required to allocate all people + 5% min 2 day tables added
        let no_of_table_days = min_no_of_table_days + (min_no_of_table_days * 105 / 100).max(2);

        println!(
            "Generating problem {} no_of_people {} no_of_table_days {}",
            problem_id, no_of_people, no_of_table_days
        );
        let problem_data = generate_problem(no_of_people, no_of_table_days);

        let json_string = serde_json::to_string(&problem_data).unwrap();

        std::fs::write(
            args.output_dir.join(format!("p_{}.json", problem_id)),
            json_string,
        )
        .expect("Failed to write output");
    }

    println!("Successfully generated {} problems", args.no_of_problems);
}
