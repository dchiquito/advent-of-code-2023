#![allow(unused)]
use std::array;

use crate::util::DaySolver;

pub struct Day15();

#[allow(clippy::upper_case_acronyms)]
pub struct HASHMAP {
    boxes: [Vec<(u64, u64)>; 256],
}
impl Default for HASHMAP {
    fn default() -> Self {
        HASHMAP {
            boxes: array::from_fn(|_| Vec::with_capacity(20)),
        }
    }
}
impl HASHMAP {
    pub fn remove(&mut self, key: &str) {
        let hash = Day15::big_hash(0, key);
        let index = Day15::hash(0, key);
        for i in 0..self.boxes[index].len() {
            if self.boxes[index][i].0 == hash {
                self.boxes[index].remove(i);
                return;
            }
        }
    }
    pub fn insert(&mut self, key: &str, value: u64) {
        let hash = Day15::big_hash(0, key);
        let index = Day15::hash(0, key);
        for i in 0..self.boxes[index].len() {
            if self.boxes[index][i].0 == hash {
                self.boxes[index][i].1 = value;
                return;
            }
        }
        self.boxes[index].push((hash, value));
    }
    pub fn operate(&mut self, operation: &str) {
        if operation.ends_with('-') {
            self.remove(&operation[0..operation.len() - 1]);
        } else {
            let (key, value) = operation.split_once('=').unwrap();
            self.insert(key, value.parse().unwrap());
        }
    }
    pub fn focusing_power(&self) -> u64 {
        self.boxes
            .iter()
            .enumerate()
            .map(|(i, boxxy)| {
                (i + 1) as u64
                    * boxxy
                        .iter()
                        .enumerate()
                        .map(|(j, lens)| (j + 1) as u64 * lens.1)
                        .sum::<u64>()
            })
            .sum::<u64>()
    }
}

impl Day15 {
    pub fn hash(current: usize, value: &str) -> usize {
        value
            .as_bytes()
            .iter()
            .map(|c| *c as usize)
            .fold(0, |c, v| ((c + v) * 17) % 256)
    }
    pub fn big_hash(current: u64, value: &str) -> u64 {
        value
            .as_bytes()
            .iter()
            .map(|c| *c as u64)
            .enumerate()
            .map(|(i, c)| (2 << i) * c)
            .sum()
        // .fold(0, |c, v| (c + v) * 17)
    }
    pub fn part2_load(input: &[String]) -> HASHMAP {
        let mut hm = HASHMAP::default();
        input[0].split(',').for_each(|step| hm.operate(step));
        hm
    }
}

type Solution = u64;
impl DaySolver<Solution> for Day15 {
    fn part1(input: Vec<String>) -> Option<Solution> {
        Some(
            input[0]
                .split(',')
                .map(|step| Self::hash(0, step) as u64)
                .sum(),
        )
    }
    fn part2(input: Vec<String>) -> Option<Solution> {
        let mut hm = Self::part2_load(&input);
        Some(hm.focusing_power())
    }
}
