use chrono::serde::ts_seconds;
use chrono::{DateTime, Utc};
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

#[derive(Debug, Clone)]
pub struct AlgorithmResults {
    pub solution: Solution,
    pub no_of_iterations: usize,
    pub elapsed: chrono::Duration
}
