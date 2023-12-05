use advent_of_code_2023::day4::Day4;
use advent_of_code_2023::day5;
use advent_of_code_2023::day5::Day5;
use advent_of_code_2023::util::{get_input, DaySolver};
use criterion::{criterion_group, criterion_main, BatchSize, Criterion};

pub fn all_benchmarks(c: &mut Criterion) {
    // Day 4
    let input1 = get_input(4);
    let input2 = input1.clone();
    let input3 = input1.clone();
    let input4 = input1.clone();
    c.bench_function("day 4 parse", |b| {
        b.iter_batched(|| &input1, Day4::parse, BatchSize::SmallInput)
    });
    c.bench_function("day 4 part 1", move |b| {
        b.iter_batched(|| input2.clone(), Day4::part1, BatchSize::SmallInput)
    });
    c.bench_function("day 4 part 2", move |b| {
        b.iter_batched(|| input3.clone(), Day4::part2, BatchSize::SmallInput)
    });
    let cards = Day4::parse(&input4);
    c.bench_function("day 4 part 1 no parse", |b| {
        b.iter_batched(|| &cards, Day4::_part1, BatchSize::SmallInput)
    });
    c.bench_function("day 4 part 2 no parse", |b| {
        b.iter_batched(|| &cards, Day4::_part2, BatchSize::SmallInput)
    });

    // Day 5
    let input1 = get_input(5);
    let input2 = input1.clone();
    let input3 = input1.clone();
    c.bench_function("day 5 parse", |b| {
        b.iter_batched(|| &input1, day5::Almanac::from, BatchSize::SmallInput)
    });
    c.bench_function("day 5 part 1", move |b| {
        b.iter_batched(|| input2.clone(), Day5::part1, BatchSize::SmallInput)
    });
    c.bench_function("day 5 part 2", move |b| {
        b.iter_batched(|| input3.clone(), Day5::part2, BatchSize::SmallInput)
    });
}

criterion_group!(benches, all_benchmarks);
criterion_main!(benches);
