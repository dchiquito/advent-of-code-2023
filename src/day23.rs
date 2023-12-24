#![allow(unused)]
use std::collections::{HashMap, HashSet};

use crate::util::DaySolver;

pub struct Trail {
    tiles: Vec<Vec<char>>,
    start: (usize, usize),
    finish: (usize, usize),
}
impl Trail {
    pub fn get(&self, (x, y): (usize, usize)) -> char {
        self.tiles[y][x]
    }
}

pub type GraphEdge = (usize, (usize, usize));
pub type GraphNode = Vec<GraphEdge>;
pub type Graph = HashMap<(usize, usize), GraphNode>;

pub struct Day23();

impl Day23 {
    pub fn parse(input: &[String]) -> Trail {
        let tiles: Vec<Vec<char>> = input.iter().map(|line| line.chars().collect()).collect();
        Trail {
            start: (1, 0),
            finish: (tiles[0].len() - 2, tiles.len() - 1),
            tiles,
        }
    }
    pub fn longest_path_given(
        trail: &Trail,
        (x, y): (usize, usize),
        mut visited: &mut HashSet<(usize, usize)>,
    ) -> usize {
        if visited.contains(&(x, y)) {
            return 0;
        } else if (x, y) == trail.finish {
            return 1;
        }
        visited.insert((x, y));
        let longest_path = match trail.get((x, y)) {
            '#' => 0,
            '>' => Self::longest_path_given(trail, (x + 1, y), visited),
            '<' => Self::longest_path_given(trail, (x - 1, y), visited), // TODO does not exist?
            'v' => Self::longest_path_given(trail, (x, y + 1), visited),
            '^' => Self::longest_path_given(trail, (x, y - 1), visited), // TODO does not exist?
            '.' => Self::longest_path_given(trail, (x + 1, y), visited)
                .max(Self::longest_path_given(trail, (x - 1, y), visited))
                .max(Self::longest_path_given(trail, (x, y + 1), visited))
                .max(Self::longest_path_given(trail, (x, y - 1), visited)),
            _ => unreachable!(),
        };
        visited.remove(&(x, y));
        longest_path + 1
    }
    pub fn longest_path(trail: &Trail) -> usize {
        Self::longest_path_given(trail, (1, 1), &mut HashSet::from([(1, 0)]))
    }
    pub fn follow_path(trail: &Trail, start: (usize, usize), next: (usize, usize)) -> GraphEdge {
        let mut last = start;
        let (mut x, mut y) = next;
        let mut steps = 0;
        while (x, y) != trail.finish && (x, y) != trail.start {
            steps += 1;
            let right = (x + 1, y) != last && trail.get((x + 1, y)) != '#';
            let left = (x - 1, y) != last && trail.get((x - 1, y)) != '#';
            let down = (x, y + 1) != last && trail.get((x, y + 1)) != '#';
            let up = (x, y - 1) != last && trail.get((x, y - 1)) != '#';
            if (right && !left && !down && !up) {
                last = (x, y);
                (x, y) = (x + 1, y);
                continue;
            }
            if (!right && left && !down && !up) {
                last = (x, y);
                (x, y) = (x - 1, y);
                continue;
            }
            if (!right && !left && down && !up) {
                last = (x, y);
                (x, y) = (x, y + 1);
                continue;
            }
            if (!right && !left && !down && up) {
                last = (x, y);
                (x, y) = (x, y - 1);
                continue;
            }
            break;
        }
        (steps, (x, y))
    }
    pub fn build_graph(trail: &Trail) -> Graph {
        let mut graph = HashMap::new();
        let mut nodes_to_check = vec![]; // TODO record direction here so we don't have to
                                         // backtrack
        let edge = Self::follow_path(trail, (1, 0), (1, 1));
        graph.insert((1, 0), vec![edge]);
        nodes_to_check.push(edge.1);
        while let Some((x, y)) = nodes_to_check.pop() {
            if (x, y) == trail.finish {
                continue;
            }
            let mut edges = vec![];
            if trail.get((x + 1, y)) != '#' {
                let edge = Self::follow_path(trail, (x, y), (x + 1, y));
                if !nodes_to_check.contains(&edge.1) && !graph.contains_key(&edge.1) {
                    nodes_to_check.push(edge.1);
                }
                edges.push(edge);
            }
            if trail.get((x - 1, y)) != '#' {
                let edge = Self::follow_path(trail, (x, y), (x - 1, y));
                if !nodes_to_check.contains(&edge.1) && !graph.contains_key(&edge.1) {
                    nodes_to_check.push(edge.1);
                }
                edges.push(edge);
            }
            if trail.get((x, y + 1)) != '#' {
                let edge = Self::follow_path(trail, (x, y), (x, y + 1));
                if !nodes_to_check.contains(&edge.1) && !graph.contains_key(&edge.1) {
                    nodes_to_check.push(edge.1);
                }
                edges.push(edge);
            }
            if trail.get((x, y - 1)) != '#' {
                let edge = Self::follow_path(trail, (x, y), (x, y - 1));
                if !nodes_to_check.contains(&edge.1) && !graph.contains_key(&edge.1) {
                    nodes_to_check.push(edge.1);
                }
                edges.push(edge);
            }
            graph.insert((x, y), edges);
        }
        graph
    }
    pub fn longest_path_in_graph_given(
        trail: &Trail,
        graph: &Graph,
        pos: (usize, usize),
        mut visited: &mut Vec<(usize, usize)>,
    ) -> Option<usize> {
        if pos == trail.finish {
            return Some(0);
        }
        if pos == trail.finish || visited.contains(&pos) {
            return None;
        }
        visited.push(pos);
        let distance = graph
            .get(&pos)
            .unwrap()
            .iter()
            .map(|(distance, adj)| {
                (
                    distance,
                    Self::longest_path_in_graph_given(trail, graph, *adj, visited),
                )
            })
            .filter_map(|(d, path)| path.map(|p| d + p))
            .max();
        visited.pop();
        distance
    }
    pub fn longest_path_in_graph(trail: &Trail, graph: &Graph) -> usize {
        Self::longest_path_in_graph_given(trail, graph, trail.start, &mut vec![]).unwrap() + 1
    }
}

type Solution = usize;
impl DaySolver<Solution> for Day23 {
    fn part1(input: Vec<String>) -> Option<Solution> {
        let trail = Self::parse(&input);
        Some(Self::longest_path(&trail))
    }
    fn part2(input: Vec<String>) -> Option<Solution> {
        let trail = Self::parse(&input);
        let graph = Self::build_graph(&trail);
        Some(Self::longest_path_in_graph(&trail, &graph))
    }
}
