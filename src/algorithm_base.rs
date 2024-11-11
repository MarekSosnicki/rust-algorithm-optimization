use chrono::Utc;
use rand::{Rng, thread_rng};
use rand::prelude::IteratorRandom;
use crate::objective_value_calculator::v1::ObjectiveValueCalculator;
use crate::problem::{MAX_PEOPLE_FOR_TABLE, PersonId, ProblemDescription, Solution, TableDayId};

pub fn solve(input: &ProblemDescription, time_limit: chrono::Duration) -> Solution{
    let start = Utc::now();

    let calculator =  ObjectiveValueCalculator::new(&input);


    let mut solution = Solution{
        solution_per_table: input.tables.iter().map(|t| (t.id, vec![])).collect(),
    };

    // initial solution - find minimum cost
    insert_into_best_positions(&calculator, &mut solution, input.people.iter().map(|p|p.id));

    println!("Base Solution cost {}", calculator.solution_value(&solution));

    let mut iteration = 0;
    let mut last_improved_iteration = 0;
    let max_no_to_remove_in_iteration = (input.people.len() / 10).min(input.people.len() / 2);
    loop {
        iteration +=1;
        if iteration - last_improved_iteration > 200 {
            println!("Terminated with no improvement");
            break;
        }
        if Utc::now() - start > time_limit {
            println!("Terminated with time limit");
            break
        }
        let current_cost = calculator.solution_value(&solution);
        let mut new_solution = solution.clone();

        let no_people_to_move = thread_rng().gen_range(2..max_no_to_remove_in_iteration);

        let mut people_to_move = vec![];

        for _ in 0..no_people_to_move {
            let table_id = *new_solution.solution_per_table.iter().filter(|(_, p)| !p.is_empty()).choose(&mut thread_rng()).unwrap().0;
            let table_to_mutate = new_solution.solution_per_table.get_mut(&table_id).unwrap();

            let chosen_person_index = (0..table_to_mutate.len()).choose(&mut thread_rng()).unwrap();
            people_to_move.push(table_to_mutate.remove(chosen_person_index))
        }
        insert_into_best_positions(&calculator, &mut new_solution, people_to_move.into_iter());


        let new_cost = calculator.solution_value(&new_solution);

        if new_cost > current_cost {
            solution = new_solution;
            last_improved_iteration = iteration;
        }
    }

    println!("Final Solution cost {}", calculator.solution_value(&solution));

    println!("Finished after {} iterations", iteration);


    solution
}


fn insert_into_best_positions(calculator: &ObjectiveValueCalculator, solution: &mut Solution, people_ids: impl Iterator<Item = PersonId>) {
    for person_id in people_ids {
        let mut best_insertion_description : Option<(TableDayId, usize)>= None;
        let mut best_insertion_value = f64::MIN;

        for (table_day_id, people) in solution.solution_per_table.iter() {
            if people.is_empty() {
                let insertion_value = calculator.table_value(*table_day_id, &[person_id]);
                if insertion_value > best_insertion_value {
                    best_insertion_description = Some((*table_day_id,0));
                    best_insertion_value = insertion_value
                }
            } else if people.len() < MAX_PEOPLE_FOR_TABLE {
                let current_cost = calculator.table_value(*table_day_id, people);
                for insertion_index in 0..people.len() {
                    let mut updated_people = people.clone();
                    updated_people.insert(insertion_index, person_id);
                    let insertion_value = calculator.table_value(*table_day_id, &[person_id]) - current_cost;
                    if insertion_value > best_insertion_value {
                        best_insertion_description = Some((*table_day_id,0));
                        best_insertion_value = insertion_value
                    }
                }
            }
        }

        let (insertion_table, insertion_index) = best_insertion_description.expect("No possible insertions?");
        solution.solution_per_table.get_mut(&insertion_table).unwrap().insert(insertion_index, person_id);
    }
}