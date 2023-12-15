use advent_of_code_2023::day4::Day4;
use advent_of_code_2023::day5;
use advent_of_code_2023::day5::Day5;
use advent_of_code_2023::day6::Day6;
use advent_of_code_2023::day7::Day7;
use advent_of_code_2023::day8::Day8;
use advent_of_code_2023::day9::Day9;

use advent_of_code_2023::day11::Day11;
use advent_of_code_2023::day12::Day12;
use advent_of_code_2023::day13::Day13;
use advent_of_code_2023::day14::Day14;
use advent_of_code_2023::day15::Day15;

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

    // Day 6
    let input1 = get_input(6);
    let input2 = input1.clone();
    c.bench_function("day 6 parse 1", |b| {
        b.iter_batched(|| &input1, Day6::parse1, BatchSize::SmallInput)
    });
    c.bench_function("day 6 part 1", move |b| {
        b.iter_batched(|| input1.clone(), Day6::part1, BatchSize::SmallInput)
    });
    c.bench_function("day 6 parse 2", |b| {
        b.iter_batched(|| &input2, Day6::parse2, BatchSize::SmallInput)
    });
    c.bench_function("day 6 part 2", move |b| {
        b.iter_batched(|| input2.clone(), Day6::part2, BatchSize::SmallInput)
    });
    //
    // Day 7
    let input1 = get_input(7);
    let input2 = input1.clone();
    c.bench_function("day 7 parse 1", |b| {
        b.iter_batched(|| &input1, Day7::parse1, BatchSize::SmallInput)
    });
    c.bench_function("day 7 part 1", move |b| {
        b.iter_batched(|| input1.clone(), Day7::part1, BatchSize::SmallInput)
    });
    c.bench_function("day 7 parse 2", |b| {
        b.iter_batched(|| &input2, Day7::parse2, BatchSize::SmallInput)
    });
    c.bench_function("day 7 part 2", move |b| {
        b.iter_batched(|| input2.clone(), Day7::part2, BatchSize::SmallInput)
    });

    // Day 8
    let input1 = get_input(8);
    let input2 = input1.clone();
    c.bench_function("day 8 parse", |b| {
        b.iter_batched(|| &input1, Day8::parse, BatchSize::SmallInput)
    });
    c.bench_function("day 8 part 1", move |b| {
        b.iter_batched(|| input1.clone(), Day8::part1, BatchSize::SmallInput)
    });
    c.bench_function("day 8 part 2", move |b| {
        b.iter_batched(|| input2.clone(), Day8::part2, BatchSize::SmallInput)
    });

    // Day 9
    let input1 = get_input(9);
    let input2 = input1.clone();
    c.bench_function("day 9 parse", |b| {
        b.iter_batched(|| &input1, Day9::parse, BatchSize::SmallInput)
    });
    c.bench_function("day 9 part 1", move |b| {
        b.iter_batched(|| input1.clone(), Day9::part1, BatchSize::SmallInput)
    });
    c.bench_function("day 9 part 2", move |b| {
        b.iter_batched(|| input2.clone(), Day9::part2, BatchSize::SmallInput)
    });

    // Day 11
    let input1 = get_input(11);
    let input2 = input1.clone();
    c.bench_function("day 11 parse", |b| {
        b.iter_batched(|| &input1, Day11::parse, BatchSize::SmallInput)
    });
    c.bench_function("day 11 part 1", move |b| {
        b.iter_batched(|| input1.clone(), Day11::part1, BatchSize::SmallInput)
    });
    c.bench_function("day 11 part 2", move |b| {
        b.iter_batched(|| input2.clone(), Day11::part2, BatchSize::SmallInput)
    });

    // Day 12
    let input1 = get_input(12);
    let input2 = input1.clone();
    c.bench_function("day 12 parse 1", |b| {
        b.iter_batched(|| &input1, Day12::parse, BatchSize::SmallInput)
    });
    c.bench_function("day 12 parse 2", |b| {
        b.iter_batched(|| &input1, Day12::parse2, BatchSize::SmallInput)
    });
    c.bench_function("day 12 part 1", move |b| {
        b.iter_batched(|| input1.clone(), Day12::part1, BatchSize::SmallInput)
    });
    c.bench_function("day 12 part 2", move |b| {
        b.iter_batched(|| input2.clone(), Day12::part2, BatchSize::SmallInput)
    });

    // Day 13
    let input1 = get_input(13);
    let input2 = input1.clone();
    c.bench_function("day 13 parse", |b| {
        b.iter_batched(|| &input1, Day13::parse, BatchSize::SmallInput)
    });
    c.bench_function("day 13 part 1", move |b| {
        b.iter_batched(|| input1.clone(), Day13::part1, BatchSize::SmallInput)
    });
    c.bench_function("day 13 part 2", move |b| {
        b.iter_batched(|| input2.clone(), Day13::part2, BatchSize::SmallInput)
    });

    // Day 14
    let input1 = get_input(14);
    let input2 = input1.clone();
    c.bench_function("day 14 parse", |b| {
        b.iter_batched(|| &input1, Day14::parse, BatchSize::SmallInput)
    });
    c.bench_function("day 14 part 1", move |b| {
        b.iter_batched(|| input1.clone(), Day14::part1, BatchSize::SmallInput)
    });
    c.bench_function("day 14 part 2", move |b| {
        b.iter_batched(|| input2.clone(), Day14::part2, BatchSize::SmallInput)
    });
    //
    // Day 15
    let input1 = get_input(15);
    let input2 = input1.clone();
    c.bench_function("day 15 part 1", move |b| {
        b.iter_batched(|| input1.clone(), Day15::part1, BatchSize::SmallInput)
    });
    c.bench_function("day 15 part 2", move |b| {
        b.iter_batched(|| input2.clone(), Day15::part2, BatchSize::SmallInput)
    });
}

criterion_group!(benches, all_benchmarks);
criterion_main!(benches);
