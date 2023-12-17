#![allow(unused)]
use std::collections::HashMap;

use crate::util::DaySolver;

type Solution = usize;

pub struct Day10();

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Direction {
    N,
    S,
    E,
    W,
}

impl Direction {
    pub fn walk(&self, (y, x): (usize, usize)) -> (usize, usize) {
        match self {
            Direction::N => (y - 1, x),
            Direction::S => (y + 1, x),
            Direction::E => (y, x + 1),
            Direction::W => (y, x - 1),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Pipe {
    NS,
    NE,
    NW,
    SE,
    SW,
    EW,
}

impl TryFrom<char> for Pipe {
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '|' => Ok(Pipe::NS),
            'L' => Ok(Pipe::NE),
            'J' => Ok(Pipe::NW),
            'F' => Ok(Pipe::SE),
            '7' => Ok(Pipe::SW),
            '-' => Ok(Pipe::EW),
            'S' => Err(true),
            _ => Err(false),
        }
    }

    type Error = bool;
}

impl Pipe {
    /// dir indicates the direction walked to enter this pipe.
    /// The returned direction is where you would go if you entered the pipe walkin in dir
    /// direction.
    /// For example, entering a SE walking North would result in you walking East.
    pub fn follow(&self, dir: &Direction) -> Direction {
        match dir {
            Direction::N => match self {
                Pipe::NS => Direction::N,
                Pipe::SE => Direction::E,
                Pipe::SW => Direction::W,
                _ => panic!(),
            },
            Direction::S => match self {
                Pipe::NS => Direction::S,
                Pipe::NE => Direction::E,
                Pipe::NW => Direction::W,
                _ => panic!(),
            },
            Direction::E => match self {
                Pipe::EW => Direction::E,
                Pipe::NW => Direction::N,
                Pipe::SW => Direction::S,
                _ => panic!(),
            },
            Direction::W => match self {
                Pipe::EW => Direction::W,
                Pipe::NE => Direction::N,
                Pipe::SE => Direction::S,
                _ => panic!(),
            },
        }
    }
    pub fn c(&self) -> char {
        match self {
            Pipe::NS => '│',
            Pipe::NE => '└',
            Pipe::NW => '┘',
            Pipe::SE => '┌',
            Pipe::SW => '┐',
            Pipe::EW => '─',
        }
    }
}

pub struct Field {
    pipes: Vec<Vec<Option<Pipe>>>,
    /// (y, x)
    start: (usize, usize),
}

impl Field {
    pub fn transit(&self, transit: &Transit) -> Transit {
        let dir = self.pipes[transit.y][transit.x]
            .as_ref()
            .unwrap()
            .follow(&transit.dir);
        let (y, x) = dir.walk((transit.y, transit.x));
        Transit { x, y, dir }
    }
    pub fn starting_transits(&self) -> (Transit, Transit) {
        let (y, x) = self.start;
        let mut starts = [Transit {
            x: 0,
            y: 0,
            dir: Direction::N,
        }; 2];
        let mut index = 0;
        // N
        if [Some(Pipe::NS), Some(Pipe::SE), Some(Pipe::SW)].contains(&self.pipes[y - 1][x]) {
            starts[index] = Transit {
                x,
                y: y - 1,
                dir: Direction::N,
            };
            index += 1;
        }
        // S
        if [Some(Pipe::NS), Some(Pipe::NE), Some(Pipe::NW)].contains(&self.pipes[y + 1][x]) {
            starts[index] = Transit {
                x,
                y: y + 1,
                dir: Direction::S,
            };
            index += 1;
        }
        // E
        if [Some(Pipe::EW), Some(Pipe::NW), Some(Pipe::SW)].contains(&self.pipes[y][x + 1]) {
            starts[index] = Transit {
                x: x + 1,
                y,
                dir: Direction::E,
            };
            index += 1;
        }
        // W
        if [Some(Pipe::EW), Some(Pipe::NE), Some(Pipe::SE)].contains(&self.pipes[y][x - 1]) {
            starts[index] = Transit {
                x: x - 1,
                y,
                dir: Direction::W,
            };
            index += 1;
        }
        (starts[0], starts[1])
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Transit {
    x: usize,
    y: usize,
    dir: Direction,
}

impl Day10 {
    fn parse(input: &[String]) -> Field {
        let mut start = (0, 0);
        let pipes = input
            .iter()
            .enumerate()
            .map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .map(|(x, c)| {
                        Pipe::try_from(c)
                            .map_err(|is_start| {
                                if is_start {
                                    start = (y, x)
                                }
                            })
                            .ok()
                    })
                    .collect()
            })
            .collect();
        Field { pipes, start }
    }
}

impl DaySolver<Solution> for Day10 {
    fn part1(input: Vec<String>) -> Option<Solution> {
        let field = Self::parse(&input);
        let (y, x) = field.start;
        let (mut a, mut b) = field.starting_transits();
        let mut steps = 1;
        while a.x != b.x || a.y != b.y {
            a = field.transit(&a);
            b = field.transit(&b);
            steps += 1;
        }
        Some(steps)
    }
    fn part2(input: Vec<String>) -> Option<Solution> {
        // Here's the algorithm:
        // Walk the path, mark all pipes that are part of the path.
        // For every row, slide along until you find a |, L, or F pipe that is part of the path.
        // Continue sliding until finding a |, 7, or J pipe. You are now inside.
        // Continue sliding, counting squares until you find a |, L, or F pipe.
        // Continue sliding until finding a |, 7, or J pipe. You are now outside. Repeat.
        let field = Self::parse(&input);
        let mut visited: HashMap<(usize, usize), Pipe> = HashMap::new();
        let (start_y, start_x) = field.start;
        // I'm just hardcoding this, too lazy to figure out from first principles
        visited.insert((start_y, start_x), Pipe::NS);
        let mut transit = field.starting_transits().0;
        while transit.x != start_x || transit.y != start_y {
            visited.insert(
                (transit.y, transit.x),
                field.pipes[transit.y][transit.x].as_ref().unwrap().clone(),
            );
            transit = field.transit(&transit);
        }
        let mut sum = 0;
        for y in 0..field.pipes.len() {
            let mut inside = false;
            let mut border_start = None;
            for x in 0..field.pipes[0].len() {
                if let Some(p) = visited.get(&(y, x)) {
                    print!("{}", p.c());
                    if p == &Pipe::NS {
                        inside = !inside;
                    } else if let Some(start) = &border_start {
                        if start == &Pipe::SE && p == &Pipe::NW {
                            inside = !inside;
                        }
                        if start == &Pipe::NE && p == &Pipe::SW {
                            inside = !inside;
                        }
                        if p != &Pipe::EW {
                            border_start = None;
                        }
                    } else {
                        border_start = Some(p.clone());
                    }
                } else if inside {
                    print!(".");
                    sum += 1;
                } else {
                    print!(" ");
                }
            }
            println!();
        }
        Some(sum)
    }
}
