use std::fs;
use std::path::PathBuf;
use clap::Parser;
use table_problem::problem::{ObjectiveValueCalculator, ProblemDescription};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    input_dir: PathBuf,
}


fn main() {
    let args = Args::parse();

    let dir = fs::read_dir(args.input_dir).unwrap();

    for file in dir {
        let filename_path = file.unwrap().path();
        let file_content = std::fs::read_to_string(&filename_path).unwrap();
        let problem : ProblemDescription = serde_json::from_str(&file_content).unwrap();
        let calculator = ObjectiveValueCalculator::new(&problem);

        println!("Running algorithm for {}", filename_path.display());
        let result = table_problem::algorithm_base::solve(&problem, chrono::Duration::seconds(1));
        println!("Result value {}  details {:?}", calculator.solution_value(&result) ,result);
    }

    println!("Successfully calculated");
}
