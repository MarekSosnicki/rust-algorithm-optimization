use crate::problem::{
    Person, PersonId, ProblemDescription, Solution, TableDay, TableDayId, TableId,
    MAX_PEOPLE_FOR_TABLE,
};
use ahash::{AHashMap, AHashSet};
use chrono::{DateTime, Datelike, Utc, Weekday};
use smallvec::SmallVec;

/// PersonInner and TableDayInner structs introduction
pub struct ObjectiveValueCalculator {
    people_map: AHashMap<PersonId, PersonInner>,
    table_map: AHashMap<TableDayId, TableDayInner>,
    relations: AHashMap<PersonId, AHashMap<PersonId, f64>>,
}

struct PersonInner {
    pub id: PersonId,
    pub most_recent_visit: Option<DateTime<Utc>>,
    pub visited_tables: AHashSet<TableId>,
    pub visited_weekdays: AHashSet<Weekday>,
}

impl From<&Person> for PersonInner {
    fn from(p: &Person) -> Self {
        PersonInner {
            id: p.id,
            most_recent_visit: p.visits.iter().max_by_key(|v| v.at).map(|v| v.at),
            visited_tables: p.visits.iter().map(|v| v.table_id).collect(),
            visited_weekdays: p.visits.iter().map(|v| v.at.weekday()).collect(),
        }
    }
}

struct TableDayInner {
    pub table_id: TableId,
    pub date: DateTime<Utc>,
    pub weekday: Weekday,
}

impl From<&TableDay> for TableDayInner {
    fn from(value: &TableDay) -> Self {
        TableDayInner {
            table_id: value.table_id,
            date: value.date,
            weekday: value.date.weekday(),
        }
    }
}

impl ObjectiveValueCalculator {
    pub fn new(input: &ProblemDescription) -> Self {
        let people_map: AHashMap<PersonId, PersonInner> = input
            .people
            .iter()
            .map(|p| (p.id, PersonInner::from(p)))
            .collect();
        let table_map: AHashMap<TableDayId, TableDayInner> = input
            .tables
            .iter()
            .map(|t| (t.id, TableDayInner::from(t)))
            .collect();
        Self {
            people_map,
            table_map,
            relations: input
                .people_relations
                .iter()
                .map(|(id, r)| (*id, r.iter().map(|(id2, value)| (*id2, *value)).collect()))
                .collect(),
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
        let people: SmallVec<[&PersonInner; MAX_PEOPLE_FOR_TABLE]> = people_ids
            .iter()
            .map(|id| self.people_map.get(id).expect("Failed to get person id"))
            .collect();

        let mut result = 0.0;
        for (seat, person) in people.iter().enumerate() {
            if people.len() != 1 {
                let next_seat = (seat + 1) % people.len();
                result += self
                    .relations
                    .get(&person.id.min(people[next_seat].id))
                    .and_then(|v| v.get(&person.id.max(people[next_seat].id)))
                    .cloned()
                    .unwrap_or_default();
            }
            if let Some(most_recent_visit) = person.most_recent_visit {
                result += (((table.date - most_recent_visit).num_days() - 15) as f64 / 15.0)
                    .min(-1.0)
                    .max(1.0)
            } else {
                // Has not visited for more than 30 days
                result += 1.0
            }
            if !person.visited_tables.contains(&table.table_id) {
                // Add value if never visited this table in the past
                result += 0.5;
            }
            if !person.visited_weekdays.contains(&table.weekday) {
                // Add value if never visited this table in this weekday
                result += 0.5;
            }
        }
        result
    }
}
