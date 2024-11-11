use chrono::{Duration, TimeZone, Utc};
use itertools::{iproduct, Itertools};
use rand::{Rng, thread_rng};
use rand::prelude::SliceRandom;
use crate::problem::{MAX_PEOPLE_FOR_TABLE, Person, PersonVisit, ProblemDescription, TableDay};

pub fn generate_problem(no_of_people: usize, no_of_table_days: usize) -> ProblemDescription {
    let start_date = Utc.with_ymd_and_hms(2024, 3, 3, 0, 0, 0).unwrap();

    // no of tables
    let no_of_tables = (no_of_people / MAX_PEOPLE_FOR_TABLE / 2).max(3);

    let days = (0..7).map(|day| start_date + Duration::days((day + 1) as i64));
    let mut all_possible_day_tables = iproduct!(days,  0..no_of_tables)
        .collect_vec();
    all_possible_day_tables.shuffle(&mut thread_rng());



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
                                Some((p2, thread_rng().gen_range(0.0..2.0)))
                            } else {
                                None
                            }
                        })
                        .collect(),
                )
            })
            .collect(),
    };
    problem_data
}
