#![allow(unused)]
use std::collections::HashSet;

use crate::util::DaySolver;

pub enum Dir {
    U,
    D,
    L,
    R,
}
impl From<&str> for Dir {
    fn from(value: &str) -> Self {
        match value {
            "U" => Dir::U,
            "D" => Dir::D,
            "L" => Dir::L,
            "R" => Dir::R,
            _ => unreachable!(),
        }
    }
}

pub struct Step {
    dir: Dir,
    distance: usize,
}

pub struct Day18();

impl Day18 {
    pub fn parse(input: &[String]) -> Vec<Step> {
        input
            .iter()
            .map(|line| {
                let dir = Dir::from(&line[..1]);
                let distance = &line[2..4].trim_end().parse().unwrap();
                let color = 0;
                Step {
                    dir,
                    distance: *distance,
                }
            })
            .collect()
    }
    pub fn border(steps: &[Step]) -> HashSet<(i32, i32)> {
        let mut x = 0;
        let mut y = 0;
        let mut visited = HashSet::new();
        for step in steps.iter() {
            let (dx, dy) = match step.dir {
                Dir::U => (0, -1),
                Dir::D => (0, 1),
                Dir::L => (-1, 0),
                Dir::R => (1, 0),
            };
            for _ in 0..step.distance {
                x += dx;
                y += dy;
                visited.insert((x, y));
            }
        }
        visited
    }
    pub fn display(steps: &[Step]) {
        let visited = Self::border(steps);
        let min_x = visited.iter().min_by_key(|(x, y)| x).unwrap().0;
        let max_x = visited.iter().max_by_key(|(x, y)| x).unwrap().0;
        let min_y = visited.iter().min_by_key(|(x, y)| y).unwrap().1;
        let max_y = visited.iter().max_by_key(|(x, y)| y).unwrap().1;
        for y in min_y..=max_y {
            for x in min_x..=max_x {
                if visited.contains(&(x, y)) {
                    print!("#");
                } else {
                    print!(" ")
                }
            }
            println!();
        }
        println!("{min_x} {max_x} {min_y} {max_y} {}", visited.len());
    }
}

type Solution = usize;
impl DaySolver<Solution> for Day18 {
    fn part1(input: Vec<String>) -> Option<Solution> {
        let steps = Self::parse(&input);
        let border = Self::border(&steps);
        // I know this is inside because I looked at it real good
        let mut to_check = vec![(1, 0)];
        let mut visited = HashSet::new();
        while let Some((x, y)) = to_check.pop() {
            visited.insert((x, y));
            if !border.contains(&(x + 1, y))
                && !visited.contains(&(x + 1, y))
                && !to_check.contains(&(x + 1, y))
            {
                to_check.push((x + 1, y));
            }
            if !border.contains(&(x - 1, y))
                && !visited.contains(&(x - 1, y))
                && !to_check.contains(&(x - 1, y))
            {
                to_check.push((x - 1, y));
            }
            if !border.contains(&(x, y + 1))
                && !visited.contains(&(x, y + 1))
                && !to_check.contains(&(x, y + 1))
            {
                to_check.push((x, y + 1));
            }
            if !border.contains(&(x, y - 1))
                && !visited.contains(&(x, y - 1))
                && !to_check.contains(&(x, y - 1))
            {
                to_check.push((x, y - 1));
            }
        }
        Some(visited.len() + border.len())
    }
    fn part2(input: Vec<String>) -> Option<Solution> {
        None
    }
}
