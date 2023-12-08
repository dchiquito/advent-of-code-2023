#![allow(unused)]
use std::collections::HashMap;

use num::Integer;

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
        let node_name_map: HashMap<&str, usize> = value[2..]
            .iter()
            .enumerate()
            .map(|(i, line)| (&line[0..3], i))
            .collect();
        let start = *node_name_map.get(&"AAA").unwrap();
        let end = *node_name_map.get(&"ZZZ").unwrap();
        let all_starts = node_name_map
            .iter()
            .filter_map(|(line, i)| {
                if line.chars().nth(2) == Some('A') {
                    Some(i)
                } else {
                    None
                }
            })
            .copied()
            .collect();
        let nodes = value[2..]
            .iter()
            .map(|line| {
                (
                    [
                        *node_name_map.get(&line[7..10]).unwrap(),
                        *node_name_map.get(&line[12..15]).unwrap(),
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

        Some(
            nodes
                .iter()
                // Walk every node and identify when they find the first Z and how long it takes to find the second
                .map(|node| {
                    let mut steps: u64 = 0;
                    let mut node = *node;
                    let mut first: u64 = 0;
                    for (steps, dir) in (0_u64..).zip(docs.directions.iter().cycle()) {
                        let (next, is_z) = docs.graph.nodes[node];
                        if is_z {
                            if first == 0 {
                                first = steps;
                            } else {
                                // First element is how many steps it took to find the first Z
                                // Second element is how many steps from there it took to find the second,
                                // which is the same thing as the length of the cycle.
                                return (first, steps - first);
                            }
                        }
                        node = next[*dir];
                    }
                    // The loop runs forever, it's not possible to get here.
                    panic!()
                })
                // For the first pair of ghosts, figure out the first moment they are both on a Z.
                // Use that to create a "virtual" ghost with a much longer cycle.
                // Repeat the process until there is only one ghost remaining.
                .reduce(|(start, incr), (a, b)| {
                    let mut steps = start;
                    while (steps - a) % b != 0 {
                        steps += incr;
                    }
                    // The length of the cycle is the least common multiple of the two subcycles
                    (steps, incr.lcm(&a))
                })
                .unwrap()
                .0,
        )
    }
}
