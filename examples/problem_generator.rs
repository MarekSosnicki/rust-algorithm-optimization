use chrono::{Duration, TimeZone, Utc};
use clap::Parser;
use itertools::{iproduct, Itertools};
use rand::prelude::SliceRandom;
use rand::{thread_rng, Rng};
use std::path::PathBuf;

use table_problem::problem::{
    Person, PersonVisit, ProblemDescription, TableDay, MAX_PEOPLE_FOR_TABLE,
};

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
        let start_date = Utc.with_ymd_and_hms(2024, 3, 3, 0, 0, 0).unwrap();
        let no_of_people = thread_rng().gen_range(args.min_people..args.max_people);
        let min_no_of_table_days = no_of_people / MAX_PEOPLE_FOR_TABLE;
        // no required to allocate all people + 5% min 2 day tables added
        let no_of_table_days = min_no_of_table_days + (min_no_of_table_days * 105 / 100).max(2);
        // no of tables
        let no_of_tables = (no_of_people / MAX_PEOPLE_FOR_TABLE / 2).max(3);

        let days = (0..7).map(|day| start_date + Duration::days((day + 1) as i64));
        let mut all_possible_day_tables = iproduct!(days,  0..no_of_tables)
            .collect_vec();
        all_possible_day_tables.shuffle(&mut thread_rng());

        println!(
            "Generating problem {} no_of_people {} no_of_table_days {} no_of_tables {}",
            problem_id, no_of_people, no_of_table_days, no_of_tables
        );

        let problem_data = ProblemDescription {
            tables: all_possible_day_tables
                .into_iter()
                .enumerate()
                .map(|(id, (date, table_id))| TableDay {
                    id,
                    table_id,
                    date,
                })
                .take(no_of_table_days)
                .collect(),
            people: (0..no_of_people)
                .map(|id| Person {
                    id,
                    visits: (0..thread_rng().gen_range(0..3))
                        .map(|_| PersonVisit {
                            table_id: thread_rng().gen_range(0..no_of_tables),
                            at: start_date - Duration::days(thread_rng().gen_range(1..30)),
                        })
                        .collect(),
                })
                .collect(),
            people_relations: (0..no_of_people)
                .map(|p1| {
                    (
                        p1,
                        ((p1 + 1)..no_of_people)
                            .filter_map(move |p2| {
                                if thread_rng().gen_range(0.0..1.0) > 0.3 {
                                    Some((p2, thread_rng().gen_range(-1.0..1.0)))
                                } else {
                                    None
                                }
                            })
                            .collect(),
                    )
                })
                .collect(),
        };

        let json_string = serde_json::to_string(&problem_data).unwrap();

        std::fs::write(
            args.output_dir.join(format!("p_{}.json", problem_id)),
            json_string,
        )
        .expect("Failed to write output");
    }

    println!("Successfully generated {} problems", args.no_of_problems);
}
