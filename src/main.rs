use clap::Parser;

mod util;

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

use crate::util::DaySolver;

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    /// Run all days
    #[arg(long)]
    all: bool,

    /// The day to run
    #[arg()]
    day: Option<usize>,

    /// Clear the cookie
    #[arg(long)]
    cookie: bool,

    /// Download input again
    #[arg(long)]
    pull: bool,
}

fn solve_part1(day: usize) -> Option<String> {
    match day {
        1 => crate::day1::Day1::solve_part1(day),
        2 => crate::day2::Day2::solve_part1(day),
        3 => crate::day3::Day3::solve_part1(day),
        4 => crate::day4::Day4::solve_part1(day),
        5 => crate::day5::Day5::solve_part1(day),
        6 => crate::day6::Day6::solve_part1(day),
        7 => crate::day7::Day7::solve_part1(day),
        8 => crate::day8::Day8::solve_part1(day),
        9 => crate::day9::Day9::solve_part1(day),
        10 => crate::day10::Day10::solve_part1(day),
        11 => crate::day11::Day11::solve_part1(day),
        12 => crate::day12::Day12::solve_part1(day),
        13 => crate::day13::Day13::solve_part1(day),
        14 => crate::day14::Day14::solve_part1(day),
        15 => crate::day15::Day15::solve_part1(day),
        16 => crate::day16::Day16::solve_part1(day),
        17 => crate::day17::Day17::solve_part1(day),
        18 => crate::day18::Day18::solve_part1(day),
        19 => crate::day19::Day19::solve_part1(day),
        20 => crate::day20::Day20::solve_part1(day),
        21 => crate::day21::Day21::solve_part1(day),
        22 => crate::day22::Day22::solve_part1(day),
        23 => crate::day23::Day23::solve_part1(day),
        24 => crate::day24::Day24::solve_part1(day),
        25 => crate::day25::Day25::solve_part1(day),
        _ => panic!("Invalid day {}", day),
    }
}

fn solve_part2(day: usize) -> Option<String> {
    match day {
        1 => crate::day1::Day1::solve_part2(day),
        2 => crate::day2::Day2::solve_part2(day),
        3 => crate::day3::Day3::solve_part2(day),
        4 => crate::day4::Day4::solve_part2(day),
        5 => crate::day5::Day5::solve_part2(day),
        6 => crate::day6::Day6::solve_part2(day),
        7 => crate::day7::Day7::solve_part2(day),
        8 => crate::day8::Day8::solve_part2(day),
        9 => crate::day9::Day9::solve_part2(day),
        10 => crate::day10::Day10::solve_part2(day),
        11 => crate::day11::Day11::solve_part2(day),
        12 => crate::day12::Day12::solve_part2(day),
        13 => crate::day13::Day13::solve_part2(day),
        14 => crate::day14::Day14::solve_part2(day),
        15 => crate::day15::Day15::solve_part2(day),
        16 => crate::day16::Day16::solve_part2(day),
        17 => crate::day17::Day17::solve_part2(day),
        18 => crate::day18::Day18::solve_part2(day),
        19 => crate::day19::Day19::solve_part2(day),
        20 => crate::day20::Day20::solve_part2(day),
        21 => crate::day21::Day21::solve_part2(day),
        22 => crate::day22::Day22::solve_part2(day),
        23 => crate::day23::Day23::solve_part2(day),
        24 => crate::day24::Day24::solve_part2(day),
        25 => crate::day25::Day25::solve_part2(day),
        _ => panic!("Invalid day {}", day),
    }
}

fn main() {
    let args = Args::parse();
    if args.cookie {
        util::clear_cookie();
    }
    if args.all {
        for day in 1..=25 {
            if args.pull {
                util::download_input(day);
            }
            if let Some(solution1) = solve_part1(day) {
                println!("Day {day}");
                println!("  Part 1: {solution1}");
                if let Some(solution2) = solve_part2(day) {
                    println!("  Part 2: {solution2}");
                }
            }
        }
    }
    if let Some(day) = args.day {
        if args.pull {
            util::download_input(day);
        }
        if let Some(solution1) = solve_part1(day) {
            println!("Part 1: {solution1}");
            if let Some(solution2) = solve_part2(day) {
                println!("Part 2: {solution2}");
            }
        } else {
            println!("At least one solution must be implemented before it can be run");
        }
    }
}
