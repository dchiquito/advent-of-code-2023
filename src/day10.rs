#![allow(unused)]
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

#[derive(Debug, Eq, PartialEq)]
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
}

pub struct Field {
    pipes: Vec<Vec<Option<Pipe>>>,
    /// (y, x)
    start: (usize, usize),
}

impl Field {
    pub fn transit(&self, transit: &Transit) -> Transit {
        println!("Walkin {transit:?}");
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
        println!("Here we go {x} {y} {a:?} {b:?}");
        let mut steps = 1;
        while a.x != b.x || a.y != b.y {
            a = field.transit(&a);
            b = field.transit(&b);
            steps += 1;
            println!("   {steps} {a:?} {b:?}");
        }
        Some(steps)
    }
    fn part2(input: Vec<String>) -> Option<Solution> {
        // Here's the algorithm:
        // Walk the loop using the part 1 code, marking all the cells that are on the path as PATH.
        // Walk the loop again, this time marking all the cells to the right of the path as RIGHT,
        // if they are not already marked PATH.
        // Flood fill: Iterate over the whole field. if you find a RIGHT cell that has not been
        // marked as filled yet, recursively check if it's adjacents are not RIGHT or PATH and set
        // them to RIGHT if they are neither.
        // Count the number of RIGHTs.
        // If at any point you encounter an out of bounds coordinate, then the right side is the
        // outside. Simply do it again, but traverse in the opposite direction so the right is the
        // inside.
        None
    }
}
