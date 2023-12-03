#![allow(unused)]
use std::collections::HashMap;

use regex::Regex;

use crate::util::DaySolver;

type Solution = u32;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct Part {
    symbol: char,
    x: usize,
    y: usize,
}

#[derive(Debug)]
struct PartNumber {
    number: u32,
    part: Part,
}

pub struct Day3();

impl Day3 {
    fn find_part(input: &[String], row: usize, col: usize, len: usize) -> Option<Part> {
        let width = input[0].len() as i32;
        let height = input.len() as i32;
        for y in 0.max(row as i32 - 1)..height.min(row as i32 + 2) {
            let line = &input[y as usize];
            for x in 0.max(col as i32 - 1)..width.min(col as i32 + len as i32 + 1) {
                let c = line.chars().nth(x as usize).unwrap();
                if !['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '.'].contains(&c) {
                    return Some(Part {
                        symbol: c,
                        x: x as usize,
                        y: y as usize,
                    });
                }
            }
        }
        None
    }
    fn parse(input: Vec<String>) -> Vec<PartNumber> {
        let mut parts = vec![];
        let digits_pattern = Regex::new("[0-9]+").unwrap();
        for (row, line) in input.iter().enumerate() {
            for capture in digits_pattern.captures_iter(line) {
                let capture = capture.get(0).unwrap();
                let number = capture.as_str().parse().unwrap();
                if let Some(part) = Self::find_part(&input, row, capture.start(), capture.len()) {
                    parts.push(PartNumber { number, part });
                }
            }
        }
        parts
    }
}

impl DaySolver<Solution> for Day3 {
    fn part1(input: Vec<String>) -> Option<Solution> {
        let data = Self::parse(input);
        Some(data.iter().map(|p| p.number).sum())
    }
    fn part2(input: Vec<String>) -> Option<Solution> {
        let data = Self::parse(input);
        let mut gears = HashMap::new();
        data.iter().filter(|p| p.part.symbol == '*').for_each(|p| {
            if !gears.contains_key(&p.part) {
                gears.insert(p.part.clone(), vec![]);
            }
            gears.get_mut(&p.part).unwrap().push(p.number);
        });
        Some(
            gears
                .iter()
                .filter(|(_, v)| v.len() == 2)
                .map(|(_, v)| v[0] * v[1])
                .sum(),
        )
    }
}
