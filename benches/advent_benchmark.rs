use advent_of_code_2023::day4::Day4;
use advent_of_code_2023::util::{get_input, DaySolver};
use criterion::{criterion_group, criterion_main, BatchSize, Criterion};

pub fn day4_benchmark(c: &mut Criterion) {
    let input = get_input(4);
    let input2 = input.clone();
    let input3 = input.clone();
    c.bench_function("day 4 part 1", move |b| {
        b.iter_batched(|| input.clone(), Day4::part1, BatchSize::SmallInput)
    });
    c.bench_function("day 4 part 2", move |b| {
        b.iter_batched(|| input2.clone(), Day4::part2, BatchSize::SmallInput)
    });
    c.bench_function("day 4 parse", move |b| {
        b.iter_batched(|| input3.clone(), Day4::parse, BatchSize::SmallInput)
    });
}

criterion_group!(benches, day4_benchmark);
criterion_main!(benches);
