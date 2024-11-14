use chrono::Utc;
use rand::{Rng, thread_rng};
use rand::prelude::IteratorRandom;
use smallvec::{smallvec, SmallVec};

use crate::objective_value_calculator::v6::ObjectiveValueCalculator;
use crate::problem::{
    AlgorithmResults, MAX_PEOPLE_FOR_TABLE, PersonId, ProblemDescription, Solution, TableDayId,
};

#[derive(Clone)]
struct TableDaySolution {
    table_day_id: TableDayId,
    people: SmallVec<[PersonId; MAX_PEOPLE_FOR_TABLE]>,
}
#[derive(Clone)]
struct SolutionInner {
    solution_per_table: Vec<TableDaySolution>,
}

impl SolutionInner {
    fn cost(&self, objective_value_calculator: &ObjectiveValueCalculator) -> f64 {
        self.solution_per_table
            .iter()
            .map(|tds| objective_value_calculator.table_value(tds.table_day_id, &tds.people))
            .sum()
    }
}

impl From<SolutionInner> for Solution {
    fn from(value: SolutionInner) -> Self {
        Solution {
            solution_per_table: value
                .solution_per_table
                .iter()
                .map(|tds| (tds.table_day_id, tds.people.to_vec()))
                .collect(),
        }
    }
}

/// Smallvec for tables
pub fn solve(input: &ProblemDescription, time_limit: chrono::Duration) -> AlgorithmResults {
    let start = Utc::now();

    let calculator = ObjectiveValueCalculator::new(&input);

    let mut solution = SolutionInner {
        solution_per_table: input
            .tables
            .iter()
            .map(|t| TableDaySolution {
                table_day_id: t.id,
                people: smallvec![],
            })
            .collect(),
    };

    // initial solution - find minimum cost
    insert_into_best_positions(
        &calculator,
        &mut solution,
        input.people.iter().map(|p| p.id),
    );

    println!("Base Solution cost {}", solution.cost(&calculator));

    let mut iteration = 0;
    let mut last_improved_iteration = 0;
    let mut current_cost = solution.cost(&calculator);

    let max_no_to_remove_in_iteration = (input.people.len() / 20).max(input.people.len().min(5));
    loop {
        iteration += 1;
        if iteration - last_improved_iteration > 200 {
            println!("Terminated with no improvement");
            break;
        }
        if Utc::now() - start > time_limit {
            println!("Terminated with time limit");
            break;
        }
        let mut new_solution = solution.clone();

        let no_people_to_move = thread_rng().gen_range(2..max_no_to_remove_in_iteration);

        let mut people_to_move = vec![];

        for _ in 0..no_people_to_move {
            let table_day = new_solution
                .solution_per_table
                .iter_mut()
                .filter(|tds| !tds.people.is_empty())
                .choose(&mut thread_rng())
                .unwrap();

            let chosen_person_index = (0..table_day.people.len())
                .choose(&mut thread_rng())
                .unwrap();
            people_to_move.push(table_day.people.remove(chosen_person_index))
        }
        insert_into_best_positions(&calculator, &mut new_solution, people_to_move.into_iter());

        let new_cost = new_solution.cost(&calculator);

        if new_cost > current_cost {
            solution = new_solution;
            last_improved_iteration = iteration;
            current_cost = new_cost;
        }
    }

    println!("Final Solution cost {}", solution.cost(&calculator));

    println!("Finished after {} iterations", iteration);

    AlgorithmResults {
        solution: Solution::from(solution),
        no_of_iterations: iteration,
        elapsed: Utc::now() - start,
    }
}

fn insert_into_best_positions(
    calculator: &ObjectiveValueCalculator,
    solution: &mut SolutionInner,
    people_ids: impl Iterator<Item = PersonId>,
) {
    for person_id in people_ids {
        let mut best_insertion_description: Option<(usize, usize)> = None;
        let mut best_insertion_value = f64::MIN;

        for (
            table_day_index,
            TableDaySolution {
                table_day_id,
                people,
            },
        ) in solution.solution_per_table.iter().enumerate()
        {
            if people.is_empty() {
                let insertion_value = calculator.table_value(*table_day_id, &[person_id]);
                if insertion_value > best_insertion_value {
                    best_insertion_description = Some((table_day_index, 0));
                    best_insertion_value = insertion_value
                }
            } else if people.len() < MAX_PEOPLE_FOR_TABLE {
                let current_cost = calculator.table_value(*table_day_id, people);
                for insertion_index in 0..people.len() {
                    let mut updated_people = people.clone();
                    updated_people.insert(insertion_index, person_id);
                    let insertion_value =
                        calculator.table_value(*table_day_id, &updated_people) - current_cost;
                    if insertion_value > best_insertion_value {
                        best_insertion_description = Some((table_day_index, 0));
                        best_insertion_value = insertion_value
                    }
                }
            }
        }

        let (insertion_table, insertion_index) =
            best_insertion_description.expect("No possible insertions?");
        solution.solution_per_table[insertion_table]
            .people
            .insert(insertion_index, person_id);
    }
}
