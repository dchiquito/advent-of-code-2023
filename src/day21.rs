#![allow(unused)]
use std::collections::{HashMap, HashSet};

use crate::util::DaySolver;

#[derive(Clone, Eq, PartialEq)]
pub enum Tile {
    Rock,
    Plot(bool),
}
#[derive(Clone)]
pub struct Field {
    tiles: HashMap<(i32, i32), Tile>,
    width: i32,
    height: i32,
    start: (i32, i32),
}
impl Field {
    pub fn visit(&mut self, x: i32, y: i32) -> bool {
        let x_mod = ((x % self.width) + self.width) % self.width;
        let y_mod = ((y % self.height) + self.height) % self.height;
        if self.tiles.get(&(x_mod, y_mod)) == Some(&Tile::Rock) {
            false
        } else {
            self.tiles.insert((x, y), Tile::Plot(true));
            true
        }
    }
    pub fn print(&self, even: bool) {
        for y in -self.height..self.height * 2 {
            for x in -self.width..self.width * 2 {
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
                    _ => print!(" "),
                }
            }
            println!();
        }
    }
    pub fn count(&self, start: (i32, i32), steps: i32, even: bool) -> u64 {
        let mut field: Field = self.clone();
        let mut to_visit = HashSet::from([start]);
        for _ in 0..=steps {
            let mut next_visits = HashSet::new();
            for (x, y) in to_visit {
                if field.visit(x, y) {
                    next_visits.insert((x + 1, y));
                    next_visits.insert((x - 1, y));
                    next_visits.insert((x, y + 1));
                    next_visits.insert((x, y - 1));
                }
            }
            to_visit = next_visits;
        }
        // field.print(even);
        field
            .tiles
            .iter()
            .filter(|((x, y), tile)| {
                if tile == &&Tile::Plot(true) {
                    even == ((x + y) % 2 == 0)
                } else {
                    false
                }
            })
            .count() as u64
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

type Solution = u64;
impl DaySolver<Solution> for Day21 {
    fn part1(input: Vec<String>) -> Option<Solution> {
        let mut field = Self::parse(&input);
        Some(field.count(field.start, 64, true))
    }
    fn part2(input: Vec<String>) -> Option<Solution> {
        let field = Self::parse(&input);
        let w = field.width;
        let w2 = w / 2;
        println!("{w} {w2}");
        let all_steps = 26501365;
        // let all_steps = 589;
        let all_steps = (4 * w) + w2;
        assert_eq!((all_steps - (w + w2)) % (w * 3), 0);
        let diagonal_tile = field.count(field.start, w + w2, true);
        println!("a tile has {diagonal_tile}");
        let diagonal_tile_x9 = field.count(field.start, (4 * w) + w2, true);
        println!("{all_steps} steps => {diagonal_tile_x9}");
        let diagonal_width_2 = ((all_steps - (w + w2)) / (w * 3)) as u64;
        let diagonal_width = (diagonal_width_2 * 2) + 1;
        let diagonal_tile_count = diagonal_width * diagonal_width;
        println!("{diagonal_width_2} {diagonal_width} => {diagonal_tile_count} tiles");
        Some(diagonal_tile_count * diagonal_tile)
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
    }
}
