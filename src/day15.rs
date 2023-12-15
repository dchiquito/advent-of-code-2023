#![allow(unused)]
use crate::util::DaySolver;

pub struct Day15();

#[allow(clippy::upper_case_acronyms)]
pub struct HASHMAP {
    boxes: [Vec<(String, u8)>; 256],
}
impl Default for HASHMAP {
    fn default() -> Self {
        const SEED: Vec<(String, u8)> = vec![];
        HASHMAP { boxes: [SEED; 256] }
    }
}
impl HASHMAP {
    pub fn remove(&mut self, key: &str) {
        let index = Day15::hash(0, key);
        for i in 0..self.boxes[index].len() {
            if self.boxes[index][i].0 == key {
                self.boxes[index].remove(i);
                return;
            }
        }
    }
    pub fn insert(&mut self, key: &str, value: u8) {
        let index = Day15::hash(0, key);
        for i in 0..self.boxes[index].len() {
            if self.boxes[index][i].0 == key {
                self.boxes[index][i].1 = value;
                return;
            }
        }
        self.boxes[index].push((key.to_string(), value));
    }
    pub fn operate(&mut self, operation: &str) {
        if operation.ends_with('-') {
            self.remove(&operation[0..operation.len() - 1]);
        } else {
            let (key, value) = operation.split_once('=').unwrap();
            self.insert(key, value.parse().unwrap());
        }
    }
    pub fn focusing_power(&self) -> usize {
        self.boxes
            .iter()
            .enumerate()
            .map(|(i, boxxy)| {
                (i + 1)
                    * boxxy
                        .iter()
                        .enumerate()
                        .map(|(j, lens)| (j + 1) * (lens.1 as usize))
                        .sum::<usize>()
            })
            .sum()
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
}

type Solution = usize;
impl DaySolver<Solution> for Day15 {
    fn part1(input: Vec<String>) -> Option<Solution> {
        Some(input[0].split(',').map(|step| Self::hash(0, step)).sum())
    }
    fn part2(input: Vec<String>) -> Option<Solution> {
        let mut hm = HASHMAP::default();
        input[0].split(',').for_each(|step| hm.operate(step));
        Some(hm.focusing_power())
    }
}
