use std::fmt::Display;

pub trait DaySolver<T>
where
    T: Display,
{
    fn part1(input: Vec<String>) -> Option<T>;
    fn part2(input: Vec<String>) -> Option<T>;
    fn get_input(day: usize) -> Vec<String> {
        // todo!()
        vec![]
    }
    fn solve_part1(day: usize) -> Option<String> {
        let input = Self::get_input(day);
        Self::part1(input).map(|solution| format!("{}", solution))
    }
    fn solve_part2(day: usize) -> Option<String> {
        let input = Self::get_input(day);
        Self::part2(input).map(|solution| format!("{}", solution))
    }
}
