use criterion::{criterion_group, criterion_main, Criterion};
use itertools::Itertools;
use rand::seq::SliceRandom;
use rand::{thread_rng, Rng};
use table_problem::generator::generate_problem;
use table_problem::objective_value_calculator::v6::ObjectiveValueCalculator;

fn objective_function_v1_benchmark(c: &mut Criterion) {
    let no_of_people = 1200;
    let no_of_tables = 220;
    let problem = generate_problem(1200, 220);
    let calculator = ObjectiveValueCalculator::new(&problem);

    let all_people_ids = {
        let mut r = (0..no_of_people).collect_vec();
        r.shuffle(&mut thread_rng());
        r
    };

    c.bench_function("objective function 1 person", |b| {
        b.iter(|| {
            let table_day_id = thread_rng().gen_range(0..no_of_tables);
            let person_id = thread_rng().gen_range(0..no_of_people);
            calculator.table_value(table_day_id, &[person_id]);
        })
    });

    c.bench_function("objective function 2 people", |b| {
        b.iter(|| {
            let table_day_id = thread_rng().gen_range(0..no_of_tables);
            let people = all_people_ids
                .choose_multiple(&mut thread_rng(), 2)
                .cloned()
                .collect_vec();

            calculator.table_value(table_day_id, &people);
        })
    });

    c.bench_function("objective function 4 people", |b| {
        b.iter(|| {
            let table_day_id = thread_rng().gen_range(0..no_of_tables);
            let people = all_people_ids
                .choose_multiple(&mut thread_rng(), 4)
                .cloned()
                .collect_vec();
            calculator.table_value(table_day_id, &people);
        })
    });

    c.bench_function("objective function 6 people", |b| {
        b.iter(|| {
            let table_day_id = thread_rng().gen_range(0..no_of_tables);
            let people = all_people_ids
                .choose_multiple(&mut thread_rng(), 6)
                .cloned()
                .collect_vec();
            calculator.table_value(table_day_id, &people);
        })
    });
}

criterion_group!(benches, objective_function_v1_benchmark);
criterion_main!(benches);
