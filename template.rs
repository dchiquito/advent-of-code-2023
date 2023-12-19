#![allow(unused)]
use crate::util::DaySolver;

pub struct Day${DAY}();

impl Day${DAY} {
    pub fn parse(input: &[String]) -> usize {
        0
    }
}

type Solution = usize;
impl DaySolver<Solution> for Day${DAY} {
    fn part1(input: Vec<String>) -> Option<Solution> {
        let data = Self::parse(&input);
        None
    }
    fn part2(input: Vec<String>) -> Option<Solution> {
        None
    }
}
