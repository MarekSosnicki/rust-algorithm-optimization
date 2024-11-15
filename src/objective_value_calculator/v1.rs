use std::collections::{BTreeMap, HashMap};

use chrono::Datelike;
use itertools::Itertools;

use crate::problem::{Person, PersonId, ProblemDescription, Solution, TableDay, TableDayId};

pub struct ObjectiveValueCalculator<'a> {
    people_map: HashMap<PersonId, &'a Person>,
    table_map: HashMap<TableDayId, &'a TableDay>,
    relations: &'a BTreeMap<PersonId, BTreeMap<PersonId, f64>>,
}

impl<'a> ObjectiveValueCalculator<'a> {
    pub fn new(input: &'a ProblemDescription) -> Self {
        let people_map: HashMap<PersonId, &Person> =
            input.people.iter().map(|p| (p.id, p)).collect();
        let table_map: HashMap<TableDayId, &TableDay> =
            input.tables.iter().map(|t| (t.id, t)).collect();
        Self {
            people_map,
            table_map,
            relations: &input.people_relations,
        }
    }

    pub fn solution_value(&self, solution: &Solution) -> f64 {
        solution
            .solution_per_table
            .iter()
            .map(|(table_day_id, people_ids)| self.table_value(*table_day_id, &people_ids))
            .sum()
    }

    pub fn table_value(&self, table_day_id: TableDayId, people_ids: &[PersonId]) -> f64 {
        let table = self
            .table_map
            .get(&table_day_id)
            .expect("Failed to get table details");
        let people: Vec<&Person> = people_ids
            .iter()
            .map(|id| *self.people_map.get(id).expect("Failed to get person id"))
            .collect_vec();

        let mut result = 0.0;
        for (seat, person) in people.iter().enumerate() {
            if people.len() != 1 {
                let next_seat = (seat + 1) % people.len();
                result += self.get_relation_score(&person.id, &people[next_seat].id);
            }
            if let Some(most_recent_visit) = person.visits.iter().max_by_key(|v| v.at) {
                result += (((table.date - most_recent_visit.at).num_days() - 15) as f64 / 15.0)
                    .min(-1.0)
                    .max(1.0)
            } else {
                result += 1.0
            }
            if !person.visits.iter().any(|v| v.table_id == table.table_id) {
                result += 0.5;
            }
            if !person
                .visits
                .iter()
                .any(|v| v.at.weekday() == table.date.weekday())
            {
                result += 0.5;
            }
        }

        result
    }

    fn get_relation_score(&self, person_1_id: &PersonId, person_2_id: &PersonId) -> f64 {
        self.relations
            .get(&person_1_id.min(person_2_id))
            .and_then(|v| v.get(&person_1_id.max(person_2_id)))
            .cloned()
            .unwrap_or_default()
    }
}
