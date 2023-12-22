#![allow(unused)]
use crate::util::DaySolver;

use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
};

#[derive(Debug)]
pub struct Brick {
    x1: usize,
    x2: usize,
    y1: usize,
    y2: usize,
    z1: u64,
    z2: u64,
    supports: HashSet<usize>,
    supported_by: HashSet<usize>,
}
impl From<&String> for Brick {
    fn from(value: &String) -> Self {
        // Conveniently, the coordinates are sorted, so x1 <= x2
        let x1 = value[0..1].parse().unwrap();
        let y1 = value[2..3].parse().unwrap();
        let (z1, value) = value[4..].split_once('~').unwrap();
        let z1 = z1.parse().unwrap();
        let x2 = value[0..1].parse().unwrap();
        let y2 = value[2..3].parse().unwrap();
        let z2 = value[4..].parse().unwrap();
        Brick {
            x1,
            x2,
            y1,
            y2,
            z1,
            z2,
            supports: HashSet::new(),
            supported_by: HashSet::new(),
        }
    }
}

pub struct HeightMap {
    // height, index
    heights: Vec<Vec<(u64, Option<usize>)>>,
}
impl HeightMap {
    pub fn new() -> HeightMap {
        HeightMap {
            heights: vec![vec![(0, None); 10]; 10],
        }
    }
    pub fn get_max_height_under(&self, brick: &Brick) -> (u64, HashSet<usize>) {
        let mut max = 0;
        let mut supported_by = HashSet::new();
        for x in brick.x1..=brick.x2 {
            for y in brick.y1..=brick.y2 {
                let (height, maybe_index) = self.heights[y][x];
                match height.cmp(&max) {
                    Ordering::Equal => {
                        if let Some(index) = maybe_index {
                            supported_by.insert(index);
                        }
                    }
                    Ordering::Greater => {
                        max = height;
                        supported_by = HashSet::from([maybe_index.unwrap()]);
                    }
                    Ordering::Less => continue,
                }
            }
        }
        (max, supported_by)
    }
    pub fn settle(&mut self, bricks: &mut [Brick]) {
        for i in 0..bricks.len() {
            let brick = &mut bricks[i];
            let (height, supported_by) = self.get_max_height_under(brick);
            brick.z2 = height + 1 + brick.z2 - brick.z1;
            brick.z1 = height + 1;
            brick.supported_by = supported_by.clone();
            for x in brick.x1..=brick.x2 {
                for y in brick.y1..=brick.y2 {
                    self.heights[y][x] = (brick.z2, Some(i))
                }
            }
            for support in supported_by.iter() {
                bricks[*support].supports.insert(i);
            }
        }
    }
}

impl Default for HeightMap {
    fn default() -> Self {
        Self::new()
    }
}

pub struct Day22();

impl Day22 {
    pub fn parse(input: &[String]) -> Vec<Brick> {
        let mut bricks: Vec<Brick> = input.iter().map(Brick::from).collect();
        bricks.sort_by_key(|b| b.z1);
        bricks
    }
}

type Solution = usize;
impl DaySolver<Solution> for Day22 {
    fn part1(input: Vec<String>) -> Option<Solution> {
        let mut bricks = Self::parse(&input);
        let mut heights = HeightMap::new();
        heights.settle(&mut bricks);
        let mut count = 0;
        Some(
            bricks
                .iter()
                .filter(|brick| {
                    brick
                        .supports
                        .iter()
                        .all(|index| bricks[*index].supported_by.len() >= 2)
                })
                .count(),
        )
    }
    fn part2(input: Vec<String>) -> Option<Solution> {
        let mut bricks = Self::parse(&input);
        let mut heights = HeightMap::new();
        heights.settle(&mut bricks);
        let mut sum = 0;
        for i in 0..bricks.len() {
            let mut knockouts = vec![i];
            let mut j = 0;
            while j < knockouts.len() {
                for b in bricks[knockouts[j]].supports.iter() {
                    if knockouts.contains(b) {
                        continue;
                    }
                    let might_knockout = &bricks[*b];
                    if might_knockout
                        .supported_by
                        .iter()
                        .all(|sb| knockouts.contains(sb))
                    {
                        knockouts.push(*b);
                    }
                }
                j += 1;
            }
            sum += knockouts.len() - 1; // don't count the original brick
        }
        Some(sum)
    }
}
