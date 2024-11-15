use std::hash::Hasher;

use ahash::AHashMap;
use chrono::Utc;
use itertools::Itertools;
use rand::prelude::IteratorRandom;
use rand::{thread_rng, Rng};
use smallvec::{smallvec, SmallVec};

use crate::objective_value_calculator::v6::ObjectiveValueCalculator;
use crate::problem::{
    AlgorithmResults, PersonId, ProblemDescription, Solution, TableDayId, MAX_PEOPLE_FOR_TABLE,
};

type TableDaySolutionHash = u64;

#[derive(Clone, PartialEq, Eq)]
struct TableDaySolution {
    table_day_id: TableDayId,
    people: SmallVec<[PersonId; MAX_PEOPLE_FOR_TABLE]>,
    hash: TableDaySolutionHash,
}

impl TableDaySolution {
    fn new(table_day_id: TableDayId, people: SmallVec<[PersonId; MAX_PEOPLE_FOR_TABLE]>) -> Self {
        let hash = calculate_table_day_hash(table_day_id, &people);
        Self {
            table_day_id,
            people,
            hash,
        }
    }

    fn remove_person_on_index(&mut self, index: usize) -> PersonId {
        let result = self.people.remove(index);
        self.hash = calculate_table_day_hash(self.table_day_id, &self.people);
        result
    }
}

fn calculate_table_day_hash(table_day_id: TableDayId, people: &[PersonId]) -> TableDaySolutionHash {
    let mut hasher = fxhash::FxHasher64::default();
    hasher.write_usize(table_day_id);
    if let Some(min_person_position) = people.iter().position_min() {
        for index in 0..people.len() {
            hasher.write_usize(people[(index + min_person_position) % people.len()])
        }
    }
    hasher.finish()
}

type InsertionCache = AHashMap<TableDaySolutionHash, AHashMap<PersonId, Option<f64>>>;

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
            .map(|t| TableDaySolution::new(t.id, smallvec![]))
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
    let mut current_cost = solution.cost(&calculator);

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
            people_to_move.push(table_day.remove_person_on_index(chosen_person_index))
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
                .get(&table_day_solution.hash)
                .and_then(|cache_for_table_day| cache_for_table_day.get(&person_to_insert).cloned())
                .unwrap_or_else(|| {
                    let value_optional = insert_into_best_table_position(
                        calculator,
                        table_day_solution,
                        person_to_insert,
                    )
                    .map(|(_, v)| v);
                    insertion_cache
                        .entry(table_day_solution.hash)
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
        solution.solution_per_table[insertion_table] = TableDaySolution::new(
            solution.solution_per_table[insertion_table].table_day_id,
            insert_into_best_table_position(
                calculator,
                &solution.solution_per_table[insertion_table],
                person_to_insert,
            )
            .unwrap()
            .0,
        );
    }
}

fn insert_into_best_table_position(
    calculator: &ObjectiveValueCalculator,
    table_day_solution: &TableDaySolution,
    person_to_insert: PersonId,
) -> Option<(SmallVec<[PersonId; MAX_PEOPLE_FOR_TABLE]>, f64)> {
    if table_day_solution.people.is_empty() {
        let people = smallvec![person_to_insert];
        let insertion_value = calculator.table_value(table_day_solution.table_day_id, &people);
        return Some((people, insertion_value));
    } else if table_day_solution.people.len() < MAX_PEOPLE_FOR_TABLE {
        let current_cost =
            calculator.table_value(table_day_solution.table_day_id, &table_day_solution.people);
        let mut best_solution = None;
        for insertion_index in 0..table_day_solution.people.len() {
            let mut updated_people = table_day_solution.people.clone();
            updated_people.insert(insertion_index, person_to_insert);
            let insertion_value = calculator
                .table_value(table_day_solution.table_day_id, &updated_people)
                - current_cost;
            if insertion_value > best_solution.as_ref().map(|(_, v)| *v).unwrap_or(f64::MIN) {
                best_solution = Some((updated_people, insertion_value))
            }
        }
        best_solution
    } else {
        // Table at full capacity
        None
    }
}
