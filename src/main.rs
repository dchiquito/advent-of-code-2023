use clap::Parser;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;

mod day20;
mod day21;
mod day22;
mod day23;
mod day24;
mod day25;

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    /// Run all days
    #[arg(long)]
    all: bool,

    /// The day to run
    #[arg()]
    day: Option<usize>,
}

fn download_input(day: usize) {}

fn get_input(day: usize) -> Vec<String> {
    todo!()
}

fn solve_part1(day: usize) -> Option<String> {
    let input = get_input(day);
    match day {
        1 => crate::day1::part1(input),
        2 => crate::day2::part1(input),
        3 => crate::day3::part1(input),
        4 => crate::day4::part1(input),
        5 => crate::day5::part1(input),
        6 => crate::day6::part1(input),
        7 => crate::day7::part1(input),
        8 => crate::day8::part1(input),
        9 => crate::day9::part1(input),
        10 => crate::day10::part1(input),
        11 => crate::day11::part1(input),
        12 => crate::day12::part1(input),
        13 => crate::day13::part1(input),
        14 => crate::day14::part1(input),
        15 => crate::day15::part1(input),
        16 => crate::day16::part1(input),
        17 => crate::day17::part1(input),
        18 => crate::day18::part1(input),
        19 => crate::day19::part1(input),
        20 => crate::day20::part1(input),
        21 => crate::day21::part1(input),
        22 => crate::day22::part1(input),
        23 => crate::day23::part1(input),
        24 => crate::day24::part1(input),
        25 => crate::day25::part1(input),
        _ => panic!("Invalid day {}", day),
    }
}

fn solve_part2(day: usize) -> Option<String> {
    let input = vec![];
    match day {
        1 => crate::day1::part2(input),
        2 => crate::day2::part2(input),
        3 => crate::day3::part2(input),
        4 => crate::day4::part2(input),
        5 => crate::day5::part2(input),
        6 => crate::day6::part2(input),
        7 => crate::day7::part2(input),
        8 => crate::day8::part2(input),
        9 => crate::day9::part2(input),
        10 => crate::day10::part2(input),
        11 => crate::day11::part2(input),
        12 => crate::day12::part2(input),
        13 => crate::day13::part2(input),
        14 => crate::day14::part2(input),
        15 => crate::day15::part2(input),
        16 => crate::day16::part2(input),
        17 => crate::day17::part2(input),
        18 => crate::day18::part2(input),
        19 => crate::day19::part2(input),
        20 => crate::day20::part2(input),
        21 => crate::day21::part2(input),
        22 => crate::day22::part2(input),
        23 => crate::day23::part2(input),
        24 => crate::day24::part2(input),
        25 => crate::day25::part2(input),
        _ => panic!("Invalid day {}", day),
    }
}

fn main() {
    let args = Args::parse();
    if args.all {
        todo!("Run all days in order")
    }
    if let Some(day) = args.day {
        solve_part1(day);
        solve_part2(day);
    }
}
