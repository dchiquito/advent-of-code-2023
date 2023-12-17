#![allow(unused)]
use std::collections::{BinaryHeap, HashMap};
use std::hash::{Hash, Hasher};

use crate::util::DaySolver;

#[derive(Debug)]
pub struct Weights {
    blocks: Vec<Vec<usize>>,
    width: i32,
    height: i32,
}
impl From<&[String]> for Weights {
    fn from(value: &[String]) -> Self {
        let blocks: Vec<Vec<usize>> = value
            .iter()
            .map(|row| row.chars().map(|c| (c as u8 - b'0') as usize).collect())
            .collect();
        Self {
            width: blocks[0].len() as i32,
            height: blocks.len() as i32,
            blocks,
        }
    }
}
impl Weights {
    pub fn get(&self, (x, y): (i32, i32)) -> Option<usize> {
        if x < 0 || y < 0 || x >= self.width || y >= self.height {
            // This won't matter, the next search step will also bounds check and fail
            None
        } else {
            Some(self.blocks[y as usize][x as usize])
        }
    }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Dir {
    N,
    S,
    E,
    W,
}
impl Dir {
    pub fn step(&self, (x, y): (i32, i32)) -> (i32, i32) {
        match self {
            Dir::N => (x, y - 1),
            Dir::S => (x, y + 1),
            Dir::E => (x + 1, y),
            Dir::W => (x - 1, y),
        }
    }
    pub fn turns(&self) -> (Dir, Dir) {
        match self {
            Dir::N => (Dir::E, Dir::W),
            Dir::S => (Dir::E, Dir::W),
            Dir::E => (Dir::S, Dir::N),
            Dir::W => (Dir::S, Dir::N),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Prospect {
    distance: usize,
    xy: (i32, i32),
    dir: Dir,
    consecutive: usize,
}

impl Ord for Prospect {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // Intentionally reversed so that the shortest distances are first in the queue
        other.distance.cmp(&self.distance)
    }
}
impl PartialOrd for Prospect {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Eq for Prospect {}
impl PartialEq for Prospect {
    fn eq(&self, other: &Self) -> bool {
        (self.xy, &self.dir, self.consecutive).eq(&(other.xy, &other.dir, other.consecutive))
    }
}
impl Hash for Prospect {
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        (&self.xy, &self.dir, self.consecutive).hash(state);
    }
}

#[derive(Default)]
pub struct Distances {
    // distance from origin, x, y, direction moved to get here, consecutive steps in that direction
    unvisited: BinaryHeap<Prospect>,
    visited: HashMap<Prospect, usize>,
}
impl Distances {
    pub fn iterate(&mut self, weights: &Weights) -> Option<usize> {
        let prospect = self.unvisited.pop().unwrap();
        if prospect.xy == (weights.width - 1, weights.height - 1) {
            return Some(prospect.distance);
        }
        if let Some(prev_distance) = self.visited.get(&prospect) {
            if prospect.distance < *prev_distance {
                let ddd = prospect.distance;
                self.visited.insert(prospect, ddd);
            }
            return None;
        }
        self.visited.insert(prospect.clone(), prospect.distance);
        let (first_turn, second_turn) = prospect.dir.turns();
        // First turn
        let xy = first_turn.step(prospect.xy);
        if let Some(weight) = weights.get(xy) {
            self.unvisited.push(Prospect {
                distance: prospect.distance + weight,
                xy,
                dir: first_turn,
                consecutive: 1,
            });
        }
        // Second turn
        let xy = second_turn.step(prospect.xy);
        if let Some(weight) = weights.get(xy) {
            self.unvisited.push(Prospect {
                distance: prospect.distance + weight,
                xy,
                dir: second_turn,
                consecutive: 1,
            });
        }
        // Straight ahead, if possible
        if prospect.consecutive < 3 {
            let xy = prospect.dir.step(prospect.xy);
            if let Some(weight) = weights.get(xy) {
                self.unvisited.push(Prospect {
                    distance: prospect.distance + weight,
                    xy,
                    dir: prospect.dir,
                    consecutive: prospect.consecutive + 1,
                });
            }
        }

        None
    }
}

pub struct Day17();

impl Day17 {
    pub fn parse(input: &[String]) -> Weights {
        Weights::from(input)
    }
    pub fn shortest_path_length(weights: &Weights, dir: Dir) -> usize {
        let mut distances = Distances::default();
        distances.unvisited.push(Prospect {
            distance: 0,
            xy: (0, 0),
            dir: Dir::E,
            consecutive: 1,
        });
        loop {
            if let Some(distance) = distances.iterate(weights) {
                return distance;
            }
        }
        unreachable!();
    }
}

type Solution = usize;
impl DaySolver<Solution> for Day17 {
    fn part1(input: Vec<String>) -> Option<Solution> {
        let weights = Self::parse(&input);
        Some(Self::shortest_path_length(&weights, Dir::E))
    }
    fn part2(input: Vec<String>) -> Option<Solution> {
        None
    }
}
