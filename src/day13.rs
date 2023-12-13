#![allow(unused)]
use crate::util::DaySolver;

type Tile = char;

#[derive(Default)]
pub struct Field {
    tiles: Vec<Vec<Tile>>,
}

impl Field {
    pub fn horizontal_reflection(&self) -> Option<usize> {
        for split in 0..self.tiles.len() - 1 {
            // check if we have found a mirrored row
            if self.tiles[split] == self.tiles[split + 1] {
                // check every other row pair
                if (1..(split + 1).min(self.tiles.len() - split - 1))
                    .all(|i| self.tiles[split - i] == self.tiles[split + 1 + i])
                {
                    return Some(split);
                }
            }
        }
        None
    }
    pub fn vertical_reflection(&self) -> Option<usize> {
        self.invert().horizontal_reflection()
    }
    pub fn horizontal_reflection_2(&self) -> Option<usize> {
        (0..self.tiles.len() - 1).find(|&split| {
            (0..(split + 1).min(self.tiles.len() - split - 1))
                .map(|y| {
                    (0..self.tiles[0].len())
                        .filter(|x| self.tiles[split - y][*x] != self.tiles[split + 1 + y][*x])
                        .count()
                })
                .sum::<usize>()
                == 1
        })
    }
    pub fn vertical_reflection_2(&self) -> Option<usize> {
        (0..self.tiles[0].len() - 1).find(|&split| {
            (0..(split + 1).min(self.tiles[0].len() - split - 1))
                .map(|x| {
                    (0..self.tiles.len())
                        .filter(|y| self.tiles[*y][split - x] != self.tiles[*y][split + 1 + x])
                        .count()
                })
                .sum::<usize>()
                == 1
        })
    }
    pub fn invert(&self) -> Field {
        Field {
            tiles: (0..self.tiles[0].len())
                .map(|x| (0..self.tiles.len()).map(|y| self.tiles[y][x]).collect())
                .collect(),
        }
    }
}

pub struct Day13();

impl Day13 {
    pub fn parse(input: &[String]) -> Vec<Field> {
        let mut fields = vec![];
        let mut field = Field::default();
        for line in input.iter() {
            if line.is_empty() {
                fields.push(field);
                field = Field::default();
            } else {
                field.tiles.push(line.chars().collect());
            }
        }
        fields.push(field);
        fields
    }

    fn solve(input: Vec<String>) -> Option<Solution> {
        None
    }
}

type Solution = usize;
impl DaySolver<Solution> for Day13 {
    fn part1(input: Vec<String>) -> Option<Solution> {
        let fields = Self::parse(&input);
        Some(
            fields
                .iter()
                .map(|field| {
                    if let Some(h) = field.horizontal_reflection() {
                        100 * (h + 1)
                    } else if let Some(v) = field.vertical_reflection() {
                        v + 1
                    } else {
                        panic!()
                    }
                })
                .sum(),
        )
    }
    fn part2(input: Vec<String>) -> Option<Solution> {
        let fields = Self::parse(&input);
        Some(
            fields
                .iter()
                .map(|field| {
                    if let Some(h) = field.horizontal_reflection_2() {
                        100 * (h + 1)
                    } else if let Some(v) = field.vertical_reflection_2() {
                        v + 1
                    } else {
                        panic!()
                    }
                })
                .sum(),
        )
    }
}
