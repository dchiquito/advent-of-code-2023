#![allow(unused)]
use std::collections::{HashMap, HashSet};

use crate::util::DaySolver;

#[derive(Eq, PartialEq)]
pub enum Tile {
    Rock,
    Plot(bool),
}
pub struct Field {
    tiles: HashMap<(i32, i32), Tile>,
    width: i32,
    height: i32,
    start: (i32, i32),
}
impl Field {
    pub fn print(&self, even: bool) {
        for y in 0..self.height {
            for x in 0..self.width {
                match self.tiles.get(&(x, y)) {
                    Some(Tile::Rock) => print!("#"),
                    Some(Tile::Plot(false)) => print!("."),
                    Some(Tile::Plot(true)) => {
                        if even == ((x + y) % 2 == 0) {
                            print!("O");
                        } else {
                            print!(".");
                        }
                    }
                    _ => {}
                }
            }
            println!();
        }
    }
}

pub struct Day21();

impl Day21 {
    pub fn parse(input: &[String]) -> Field {
        let (sy, line) = input
            .iter()
            .enumerate()
            .find(|(_, line)| line.contains('S'))
            .unwrap();
        let sx = line.chars().enumerate().find(|(_, c)| c == &'S').unwrap().0;
        let start = (sx as i32, sy as i32);
        let tiles = input
            .iter()
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars().enumerate().map(move |(x, c)| {
                    (
                        (x as i32, y as i32),
                        match c {
                            'S' => Tile::Plot(false),
                            '#' => Tile::Rock,
                            '.' => Tile::Plot(false),
                            _ => unreachable!(),
                        },
                    )
                })
            })
            .collect();
        Field {
            tiles,
            width: input[0].len() as i32,
            height: input.len() as i32,
            start,
        }
    }
}

type Solution = usize;
impl DaySolver<Solution> for Day21 {
    fn part1(input: Vec<String>) -> Option<Solution> {
        let mut field = Self::parse(&input);
        field.print(true);
        let mut to_visit = HashSet::new();
        to_visit.insert(field.start);
        for _ in 0..=64 {
            let mut next_visits = HashSet::new();
            for (x, y) in to_visit {
                if let Some(&Tile::Plot(visited)) = field.tiles.get(&(x, y)) {
                    if !visited {
                        field.tiles.insert((x, y), Tile::Plot(true));
                    }
                    next_visits.insert((x + 1, y));
                    next_visits.insert((x - 1, y));
                    next_visits.insert((x, y + 1));
                    next_visits.insert((x, y - 1));
                }
            }
            to_visit = next_visits;
        }
        field.print(true);
        Some(
            field
                .tiles
                .iter()
                .filter(|((x, y), tile)| {
                    if tile == &&Tile::Plot(true) {
                        (x + y) % 2 == 0
                    } else {
                        false
                    }
                })
                .count(),
        )
    }
    fn part2(input: Vec<String>) -> Option<Solution> {
        let field = Self::parse(&input);
        let w = field.width;
        // Some interesting notes:
        // There is an unobstructed vertical and horizontal corridor starting exactly in the
        // center. This means we don't have to worry about any obstructions when figuring out the
        // shortest path to any plot on the axes.
        // There is also an unobstructed row around every edge of the plot.
        // The rocks are sparse enough that for any sufficiently long distance, the minimum path is
        // basically just the manhattan distance, especially from edge to edge.
        // Let's just assume that's true. You can always get from one edge tile to another edge
        // tile in the manhattan distance steps.
        // The dimensions of the field are 131 by 131. So 130 steps to walk across, or 130 steps to
        // walk from the center to a corner.
        // 26501365 steps / 130 steps per field = 203856 fields, in one dimension of course
        //          (rounded down)
        // I do see regions that are completely blocked off by rocks, so we must do a flood fill.
        // It is not enough to just calculate the area and subtract the number of rocks in it.
        // 26501365 steps from the origin
        // It takes 130/2 = 65 steps to get to the edge from the middle, so 26501300 steps in every
        // cardinal direction to get to the tips of the diamond.
        //
        None
    }
}
