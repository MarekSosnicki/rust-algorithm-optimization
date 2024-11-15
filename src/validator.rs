use ahash::HashSet;

use crate::problem::{PersonId, ProblemDescription, Solution, MAX_PEOPLE_FOR_TABLE};

pub fn validate_solution(input: &ProblemDescription, solution: &Solution) {
    assert_eq!(
        input.tables.len(),
        solution.solution_per_table.len(),
        "No of tables in solution must match"
    );

    assert_eq!(
        input.people.len(),
        solution.solution_per_table.values().flatten().count(),
        "No of people in the solution must be the same"
    );

    let all_people_from_solution: HashSet<PersonId> = solution
        .solution_per_table
        .values()
        .cloned()
        .flatten()
        .collect();
    let all_people_from_input: HashSet<PersonId> = input.people.iter().map(|p| p.id).collect();
    assert_eq!(
        all_people_from_input, all_people_from_solution,
        "all peaople from input shoud be in solution"
    );

    for (table_day_id, people_for_table) in solution.solution_per_table.iter() {
        assert!(
            people_for_table.len() <= MAX_PEOPLE_FOR_TABLE,
            "Table {} had more people then allowed",
            table_day_id
        )
    }
}
