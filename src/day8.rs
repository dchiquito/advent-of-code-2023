#![allow(unused)]
use crate::util::DaySolver;

type Solution = usize;

pub type Directions = Vec<usize>;

#[derive(Debug)]
pub struct Graph {
    start: usize,
    end: usize,
    nodes: Vec<[usize; 2]>,
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
        let nodes = value[2..]
            .iter()
            .map(|line| {
                [
                    node_names.iter().position(|x| x == &&line[7..10]).unwrap(),
                    node_names.iter().position(|x| x == &&line[12..15]).unwrap(),
                ]
            })
            .collect();
        let graph = Graph { start, end, nodes };
        Docs { directions, graph }
    }
}

pub struct Day8();

impl DaySolver<Solution> for Day8 {
    fn part1(input: Vec<String>) -> Option<Solution> {
        let docs = Docs::from(&input[..]);
        let mut node = docs.graph.start;
        let mut steps = 0;
        for dir in docs.directions.iter().cycle() {
            if node == docs.graph.end {
                break;
            }
            node = docs.graph.nodes[node][*dir];
            steps += 1;
        }
        Some(steps)
    }
    fn part2(input: Vec<String>) -> Option<Solution> {
        None
    }
}
