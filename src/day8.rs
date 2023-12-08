#![allow(unused)]
use crate::util::DaySolver;

pub type Directions = Vec<usize>;

#[derive(Debug)]
pub struct Graph {
    start: usize,
    end: usize,
    // All node indices that end in A
    all_starts: Vec<usize>,
    // The left node, the right node, and whether or not this node ends in Z
    nodes: Vec<([usize; 2], bool)>,
}

#[derive(Debug)]
pub struct Docs {
    directions: Directions,
    graph: Graph,
}

impl From<&[String]> for Docs {
    fn from(value: &[String]) -> Self {
        let directions = value[0]
            .chars()
            .map(|c| match c {
                'L' => 0,
                'R' => 1,
                _ => panic!(),
            })
            .collect();
        let node_names: Vec<&str> = value[2..].iter().map(|line| &line[0..3]).collect();
        let start = node_names.iter().position(|x| x == &"AAA").unwrap();
        let end = node_names.iter().position(|x| x == &"ZZZ").unwrap();
        let all_starts = node_names
            .iter()
            .enumerate()
            .filter_map(|(i, line)| {
                if line.chars().nth(2) == Some('A') {
                    Some(i)
                } else {
                    None
                }
            })
            .collect();
        let nodes = value[2..]
            .iter()
            .map(|line| {
                (
                    [
                        node_names.iter().position(|x| x == &&line[7..10]).unwrap(),
                        node_names.iter().position(|x| x == &&line[12..15]).unwrap(),
                    ],
                    line.chars().nth(2) == Some('Z'),
                )
            })
            .collect();
        let graph = Graph {
            start,
            end,
            all_starts,
            nodes,
        };
        Docs { directions, graph }
    }
}

type Solution = u64;
pub struct Day8();

impl Day8 {
    pub fn parse(input: &[String]) -> Docs {
        Docs::from(input)
    }
}

impl DaySolver<Solution> for Day8 {
    fn part1(input: Vec<String>) -> Option<Solution> {
        let docs = Self::parse(&input[..]);
        let mut node = docs.graph.start;
        let mut steps = 0;
        for dir in docs.directions.iter().cycle() {
            if node == docs.graph.end {
                break;
            }
            node = docs.graph.nodes[node].0[*dir];
            steps += 1;
        }
        Some(steps)
    }
    fn part2(input: Vec<String>) -> Option<Solution> {
        let docs = Self::parse(&input[..]);
        let mut nodes = docs.graph.all_starts.clone();

        // First element is how many steps it took to find the first Z
        // Second element is how many steps from there it took to find the second,
        // which is the same thing as the length of the cycle.
        let mut loop_info: Vec<(u64, u64)> = nodes.iter().map(|_| (0, 0)).collect();
        (0..nodes.len()).for_each(|i| {
            let mut steps = 0;
            let mut node = nodes[i];
            #[allow(clippy::explicit_counter_loop)]
            for dir in docs.directions.iter().cycle() {
                let (next, is_z) = docs.graph.nodes[node];
                if is_z {
                    if loop_info[i].0 == 0 {
                        loop_info[i].0 = steps;
                    } else {
                        loop_info[i].1 = steps - loop_info[i].0;
                        break;
                    }
                }
                node = next[*dir];
                steps += 1;
            }
        });
        // Find the path with the longest cycle, it will get us to the answer the fastest
        let (start, incr) = *loop_info.iter().max_by_key(|(_, incr)| incr).unwrap();
        // Remove that path from the loops we are checking
        let loop_info: Vec<(u64, u64)> = loop_info
            .iter()
            .filter(|(a, _b)| start != *a)
            .copied()
            .collect();
        let mut steps = start;
        let mut all_done = false;
        while loop_info.iter().any(|(a, b)| (steps - a) % b != 0) {
            steps += incr;
        }
        Some(steps)
    }
}
