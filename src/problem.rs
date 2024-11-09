use chrono::serde::ts_seconds;
use chrono::{DateTime, Datelike, Utc};
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap};

pub const MAX_PEOPLE_FOR_TABLE: usize = 6;

pub type PersonId = usize;
pub type TableId = usize;
pub type TableDayId = usize;

#[derive(Serialize, Deserialize, Debug)]
pub struct ProblemDescription {
    /// List of people to allocate to seats
    pub people: Vec<Person>,
    /// List of tables to allocate to
    pub tables: Vec<TableDay>,
    /// How much one person wants to sit next to another, the higher value the better
    pub people_relations: BTreeMap<PersonId, BTreeMap<PersonId, f64>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Person {
    /// Person unique ID
    pub id: PersonId,
    /// Past visits
    pub visits: Vec<PersonVisit>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PersonVisit {
    pub table_id: TableId,
    #[serde(with = "ts_seconds")]
    pub at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TableDay {
    /// Unique table id for a given day, this is not a table id
    pub id: TableDayId,
    /// Unique table id
    pub table_id: TableId,
    /// Table visit date
    #[serde(with = "ts_seconds")]
    pub date: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct Solution {
    pub solution_per_table: HashMap<TableDayId, Vec<PersonId>>,
}



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
            .map(|(table_day_id, people_ids)| {
                self.table_value(*table_day_id, &people_ids)
            })
            .sum()
    }

    pub fn table_value(&self, table_day_id: TableDayId, people_ids: &[PersonId]) -> f64 {
        let table = self.table_map
            .get(&table_day_id)
            .expect("Failed to get table details");
        let people: Vec<&Person> = people_ids
            .iter()
            .map(|id| *self.people_map.get(id).expect("Failed to get person id"))
            .collect_vec();

        if people.is_empty() {
            return 0.0;
        }

        let mut result = 0.0;
        for (seat, person) in people.iter().enumerate() {
            let next_seat = (seat + 1) % people.len();

            result += self.relations
                .get(&person.id)
                .and_then(|v| v.get(&people[next_seat].id))
                .cloned()
                .unwrap_or_default();

            if let Some(most_recent_visit) = person.visits.iter().max_by_key(|v| v.at) {
                result += (((table.date - most_recent_visit.at).num_days() - 15) as f64 / 15.0)
                    .min(-1.0)
                    .max(1.0)
            } else {
                // Has not visited for more than 30 days
                result += 1.0
            }

            if !person.visits.iter().any(|v| v.table_id == table.table_id) {
                // Add value if never visited this table in the past
                result += 0.5;
            }

            if !person
                .visits
                .iter()
                .any(|v| v.at.weekday() == table.date.weekday())
            {
                // Add value if never visited this table in this weekday
                result += 0.5;
            }
        }

        result
    }
}
