#![allow(unused)]
use crate::util::DaySolver;

type Solution = usize;

pub struct Day${DAY}();

impl Day${DAY} {
    fn parse(input: Vec<String>) -> usize {
        0
    }

    fn solve(input: Vec<String>) -> Option<Solution> {
        let data = Self::parse(input);
        None
    }
}

impl DaySolver<Solution> for Day${DAY} {
    fn part1(input: Vec<String>) -> Option<Solution> {
        Self::solve(input)
    }
    fn part2(input: Vec<String>) -> Option<Solution> {
        Self::solve(input)
    }
}
