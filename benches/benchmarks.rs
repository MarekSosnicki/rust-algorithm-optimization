use criterion::{criterion_group, criterion_main, Criterion};
use itertools::Itertools;
use rand::{Rng, thread_rng};
use table_problem::generator::generate_problem;
use table_problem::objective_value_calculator::v5::ObjectiveValueCalculator;


fn objective_function_v1_benchmark(c: &mut Criterion) {
    let no_of_people = 1200;
    let no_of_tables = 220;
    let problem = generate_problem(1200, 220);
    let calculator = ObjectiveValueCalculator::new(&problem);
    c.bench_function("objective function 1 person", |b| b.iter(|| {
        let table_day_id = thread_rng().gen_range(0..no_of_tables);
        let person_id = thread_rng().gen_range(0..no_of_people);
        calculator.table_value(table_day_id, &[person_id]);
    }));

    c.bench_function("objective function 2 people", |b| b.iter(|| {
        let table_day_id = thread_rng().gen_range(0..no_of_tables);
        let person_id_1 = thread_rng().gen_range(0..no_of_people);
        let person_id_2 = thread_rng().gen_range(0..no_of_people);

        calculator.table_value(table_day_id, &[person_id_1, person_id_2]);
    }));


    c.bench_function("objective function 4 people", |b| b.iter(|| {
        let table_day_id = thread_rng().gen_range(0..no_of_tables);
        let people = (0..4).map(|_|thread_rng().gen_range(0..no_of_people)).collect_vec();
        calculator.table_value(table_day_id, &people);
    }));

    c.bench_function("objective function 6 people", |b| b.iter(|| {
        let table_day_id = thread_rng().gen_range(0..no_of_tables);
        let people = (0..6).map(|_|thread_rng().gen_range(0..no_of_people)).collect_vec();
        calculator.table_value(table_day_id, &people);
    }));
}

criterion_group!(benches, objective_function_v1_benchmark);
criterion_main!(benches);