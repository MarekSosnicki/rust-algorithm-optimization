use std::hash::{Hash, Hasher};

use ahash::AHashMap;
use chrono::Utc;
use itertools::Itertools;
use rand::{Rng, thread_rng};
use rand::prelude::IteratorRandom;
use smallvec::{smallvec, SmallVec};

use crate::objective_value_calculator::v6::ObjectiveValueCalculator;
use crate::problem::{
    AlgorithmResults, MAX_PEOPLE_FOR_TABLE, PersonId, ProblemDescription, Solution, TableDayId,
};

#[derive(Clone, PartialEq, Eq)]
struct TableDaySolution {
    table_day_id: TableDayId,
    people: SmallVec<[PersonId; MAX_PEOPLE_FOR_TABLE]>,
}

impl Hash for TableDaySolution {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_usize(self.table_day_id);
        if let Some(min_person_position) = self.people.iter().position_min() {
            for index in 0..self.people.len() {
                state.write_usize(self.people[(index + min_person_position) % self.people.len()])
            }
        }
    }
}

type InsertionCache = AHashMap<TableDaySolution, AHashMap<PersonId, Option<f64>>>;

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

/// Insertion cache
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

    let mut insertion_cache: InsertionCache = Default::default();

    // initial solution - find minimum cost
    insert_into_best_positions(
        &calculator,
        &mut insertion_cache,
        &mut solution,
        input.people.iter().map(|p| p.id),
    );

    println!("Base Solution cost {}", solution.cost(&calculator));

    let mut iteration = 0;
    let mut last_improved_iteration = 0;
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
        let current_cost = solution.cost(&calculator);
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
        insert_into_best_positions(
            &calculator,
            &mut insertion_cache,
            &mut new_solution,
            people_to_move.into_iter(),
        );

        let new_cost = new_solution.cost(&calculator);

        if new_cost > current_cost {
            solution = new_solution;
            last_improved_iteration = iteration;
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
    insertion_cache: &mut InsertionCache,
    solution: &mut SolutionInner,
    people_to_insert: impl Iterator<Item = PersonId>,
) {
    for person_to_insert in people_to_insert {
        let mut best_insertion_table: Option<usize> = None;
        let best_insertion_value = f64::MIN;

        for (table_day_index, table_day_solution) in solution.solution_per_table.iter().enumerate()
        {
            let value_optional = insertion_cache
                .get(&table_day_solution)
                .and_then(|cache_for_table_day| cache_for_table_day.get(&person_to_insert).cloned())
                .unwrap_or_else(|| {
                    let value_optional = insert_into_best_table_position(
                        calculator,
                        table_day_solution,
                        person_to_insert,
                    )
                    .map(|(_, v)| v);
                    insertion_cache
                        .entry(table_day_solution.clone())
                        .or_default()
                        .insert(person_to_insert, value_optional);
                    value_optional
                });

            if let Some(value) = value_optional {
                if value > best_insertion_value {
                    best_insertion_table = Some(table_day_index);
                }
            }
        }

        let insertion_table = best_insertion_table.expect("No possible insertions?");
        solution.solution_per_table[insertion_table] = insert_into_best_table_position(
            calculator,
            &solution.solution_per_table[insertion_table],
            person_to_insert,
        )
        .unwrap()
        .0;
    }
}

fn insert_into_best_table_position(
    calculator: &ObjectiveValueCalculator,
    table_day_solution: &TableDaySolution,
    person_to_insert: PersonId,
) -> Option<(TableDaySolution, f64)> {
    if table_day_solution.people.is_empty() {
        let people = smallvec![person_to_insert];
        let insertion_value = calculator.table_value(table_day_solution.table_day_id, &people);
        return Some((
            TableDaySolution {
                table_day_id: table_day_solution.table_day_id,
                people,
            },
            insertion_value,
        ));
    } else if table_day_solution.people.len() < MAX_PEOPLE_FOR_TABLE {
        let current_cost =
            calculator.table_value(table_day_solution.table_day_id, &table_day_solution.people);
        let mut best_solution: Option<(TableDaySolution, f64)> = None;
        for insertion_index in 0..table_day_solution.people.len() {
            let mut updated_people = table_day_solution.people.clone();
            updated_people.insert(insertion_index, person_to_insert);
            let insertion_value = calculator
                .table_value(table_day_solution.table_day_id, &updated_people)
                - current_cost;
            if insertion_value > best_solution.as_ref().map(|(_, v)| *v).unwrap_or(f64::MIN) {
                best_solution = Some((
                    TableDaySolution {
                        table_day_id: table_day_solution.table_day_id,
                        people: updated_people,
                    },
                    insertion_value,
                ))
            }
        }
        best_solution
    } else {
        // Table at full capacity
        None
    }
}
