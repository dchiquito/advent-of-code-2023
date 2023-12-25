#![allow(unused)]
use std::{
    collections::{hash_map::DefaultHasher, HashMap, HashSet, VecDeque},
    hash::{Hash, Hasher},
};

use crate::util::DaySolver;

pub struct Graph {
    nodes: HashMap<u64, Vec<u64>>,
    keys: Vec<u64>,
}

impl Graph {
    fn hash(s: &str) -> u64 {
        let mut hasher = DefaultHasher::new();
        s.hash(&mut hasher);
        hasher.finish()
    }
    pub fn find_path(&self, start: u64, end: u64, excluding: &[Edge]) -> Result<Vec<Edge>, usize> {
        let mut to_visit: VecDeque<u64> = VecDeque::from([start]);
        let mut visited = HashSet::new();
        // For a given node, record the previous node in the path so we can reconstruct the path
        let mut path_links: HashMap<u64, u64> = HashMap::new();
        while let Some(node) = to_visit.pop_front() {
            visited.insert(node);
            for adj in self.nodes.get(&node).unwrap() {
                if *adj == end {
                    let mut path = vec![];
                    let mut node = node;
                    while node != start {
                        let next_node = *path_links.get(&node).unwrap();
                        path.push((node, next_node));
                        node = next_node;
                    }
                    return Ok(path);
                }
                if visited.contains(adj)
                    || is_edge_in(excluding, &(node, *adj))
                    || to_visit.contains(adj)
                {
                    continue;
                }
                to_visit.push_back(*adj);
                path_links.insert(*adj, node);
            }
        }
        Err(visited.len())
    }
}

pub type Edge = (u64, u64);
pub fn is_edge_in(edges: &[Edge], edge: &Edge) -> bool {
    let inv_edge = &(edge.1, edge.0);
    edges.iter().any(|e| e == edge || e == inv_edge)
}

pub struct Day25();

impl Day25 {
    pub fn parse(input: &[String]) -> Graph {
        let mut nodes: HashMap<u64, Vec<u64>> = input
            .iter()
            .map(|line| {
                let (id, adjs) = line.split_once(": ").unwrap();
                let id = Graph::hash(id);
                let adjs = adjs.split(' ').map(Graph::hash).collect();
                (id, adjs)
            })
            .collect();
        let keys: Vec<u64> = nodes.keys().copied().collect();
        for key in keys.iter() {
            let adjs = nodes.get(key).unwrap().clone();
            for adj in adjs {
                nodes.entry(adj).or_default();
                nodes.get_mut(&adj).unwrap().push(*key);
            }
        }
        Graph { nodes, keys }
    }
}

type Solution = usize;
impl DaySolver<Solution> for Day25 {
    fn part1(input: Vec<String>) -> Option<Solution> {
        let graph = Self::parse(&input);
        let start = graph.keys[0];
        for i in 1..graph.keys.len() {
            let end = graph.keys[i];
            let mut paths = vec![];
            let path1 = graph.find_path(start, end, &paths).unwrap();
            paths.append(&mut path1.clone());
            let path2 = graph.find_path(start, end, &paths).unwrap();
            paths.append(&mut path2.clone());
            let path3 = graph.find_path(start, end, &paths).unwrap();
            paths.append(&mut path3.clone());
            if graph.find_path(start, end, &paths).is_err() {
                for e1 in path1.iter() {
                    for e2 in path2.iter() {
                        for e3 in path3.iter() {
                            if let Err(size) = graph.find_path(start, end, &[*e1, *e2, *e3]) {
                                return Some((graph.nodes.len() - size) * size);
                            }
                        }
                    }
                }
            } else {
                println!("Failed :(");
            }
        }
        unreachable!();
    }
    fn part2(input: Vec<String>) -> Option<Solution> {
        None
    }
}
