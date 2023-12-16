#![allow(unused)]
use crate::util::DaySolver;

#[derive(Debug, Eq, PartialEq)]
pub enum Dir {
    N,
    S,
    E,
    W,
}

impl Dir {
    pub fn is_ns(&self) -> bool {
        self == &Dir::N || self == &Dir::S
    }
    pub fn apply(&self, x: i32, y: i32) -> (i32, i32) {
        match self {
            Dir::N => (x, y - 1),
            Dir::S => (x, y + 1),
            Dir::E => (x + 1, y),
            Dir::W => (x - 1, y),
        }
    }
    pub fn index(&self) -> usize {
        match self {
            Dir::N => 0,
            Dir::S => 1,
            Dir::E => 2,
            Dir::W => 3,
        }
    }
}

pub struct Visitations {
    /// (NS, EW)
    tiles: Vec<Vec<[bool; 4]>>,
}
impl From<&Contraption> for Visitations {
    fn from(value: &Contraption) -> Self {
        Visitations {
            tiles: vec![vec![[false; 4]; value.tiles[0].len()]; value.tiles.len()],
        }
    }
}

impl Visitations {
    pub fn visits(&self) -> usize {
        self.tiles
            .iter()
            .map(|row| row.iter().filter(|v| v.iter().any(|x| *x)).count())
            .sum()
    }
    pub fn visit(&mut self, x: i32, y: i32, dir: &Dir) {
        self.tiles[y as usize][x as usize][dir.index()] = true;
    }
    pub fn has_visited(&self, x: i32, y: i32, dir: &Dir) -> bool {
        self.tiles[y as usize][x as usize][dir.index()]
    }
}

pub struct Contraption {
    tiles: Vec<Vec<u8>>,
    width: usize,
    height: usize,
}
impl From<&[String]> for Contraption {
    fn from(value: &[String]) -> Self {
        let tiles: Vec<Vec<u8>> = value.iter().map(|line| line.as_bytes().into()).collect();
        Contraption {
            width: tiles[0].len(),
            height: tiles.len(),
            tiles,
        }
    }
}
impl Contraption {
    pub fn get(&self, x: i32, y: i32) -> u8 {
        self.tiles[y as usize][x as usize]
    }
    pub fn find_visits(&self, visitations: &mut Visitations, mut x: i32, mut y: i32, mut dir: Dir) {
        while !(x < 0 || x >= self.tiles[0].len() as i32 || y < 0 || y >= self.tiles.len() as i32)
            && !visitations.has_visited(x, y, &dir)
        {
            visitations.visit(x, y, &dir);
            match self.get(y, x) {
                b'.' => (x, y) = dir.apply(x, y),
                b'/' => {
                    dir = match dir {
                        Dir::N => Dir::E,
                        Dir::S => Dir::W,
                        Dir::E => Dir::N,
                        Dir::W => Dir::S,
                    };
                    (x, y) = dir.apply(x, y);
                }
                b'\\' => {
                    dir = match dir {
                        Dir::N => Dir::W,
                        Dir::S => Dir::E,
                        Dir::E => Dir::S,
                        Dir::W => Dir::N,
                    };
                    (x, y) = dir.apply(x, y);
                }
                b'|' => {
                    if dir.is_ns() {
                        (x, y) = dir.apply(x, y);
                    } else {
                        self.find_visits(visitations, x, y + 1, Dir::S);
                        self.find_visits(visitations, x, y - 1, Dir::N);
                        return;
                    }
                }
                b'-' => {
                    if !dir.is_ns() {
                        (x, y) = dir.apply(x, y);
                    } else {
                        self.find_visits(visitations, x + 1, y, Dir::E);
                        self.find_visits(visitations, x - 1, y, Dir::W);
                        return;
                    }
                }
                _ => panic!(),
            }
        }
    }
    pub fn visits(&self, x: usize, y: usize, dir: Dir) -> usize {
        let mut visitations = Visitations::from(self);
        self.find_visits(&mut visitations, x as i32, y as i32, dir);
        visitations.visits()
    }
}

pub struct Day16();

type Solution = usize;
impl Day16 {
    pub fn parse(input: &[String]) -> Contraption {
        Contraption::from(input)
    }
}

impl DaySolver<Solution> for Day16 {
    fn part1(input: Vec<String>) -> Option<Solution> {
        let contraption = Self::parse(&input);
        let mut visitations = Visitations::from(&contraption);
        contraption.find_visits(&mut visitations, 0, 0, Dir::E);
        Some(visitations.visits())
    }
    fn part2(input: Vec<String>) -> Option<Solution> {
        let contraption = Self::parse(&input);
        let mut max: usize = 0;
        for x in 0..contraption.width {
            max = max.max(contraption.visits(x, 0, Dir::N));
            max = max.max(contraption.visits(x, 0, Dir::S));
        }
        for y in 0..contraption.height {
            max = max.max(contraption.visits(0, y, Dir::E));
            max = max.max(contraption.visits(0, y, Dir::W));
        }
        Some(max)
    }
}
