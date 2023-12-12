#![allow(unused)]
use crate::util::DaySolver;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Spring {
    Good,
    Bad,
    Dunno,
}

impl From<char> for Spring {
    fn from(value: char) -> Self {
        match value {
            '.' => Spring::Good,
            '#' => Spring::Bad,
            '?' => Spring::Dunno,
            _ => panic!(),
        }
    }
}

impl Spring {
    pub fn maybe_good(&self) -> bool {
        match self {
            Spring::Good => true,
            Spring::Bad => false,
            Spring::Dunno => true,
        }
    }
    pub fn maybe_bad(&self) -> bool {
        match self {
            Spring::Good => false,
            Spring::Bad => true,
            Spring::Dunno => true,
        }
    }
}

#[derive(Debug)]
pub struct Row {
    springs: Vec<Spring>,
    bads: Vec<usize>,
}
impl From<&String> for Row {
    fn from(value: &String) -> Self {
        let (left, right) = value.split_once(' ').unwrap();
        Row {
            springs: left.chars().map(Spring::from).collect(),
            bads: right
                .split(',')
                .map(|digits| digits.parse().unwrap())
                .collect(),
        }
    }
}

type State<'a> = (&'a [Spring], &'a [usize]);

pub struct Day12();

impl Day12 {
    fn parse(input: &[String]) -> Vec<Row> {
        input.iter().map(Row::from).collect()
    }
    fn solve_1(springs: &[Spring], bads: &[usize]) -> u64 {
        // There are no bad sections, verify that there are no bad springs
        if bads.is_empty() {
            if springs.iter().all(|s| s != &Spring::Bad) {
                return 1;
            } else {
                return 0;
            }
        }
        // Check that we can still hypothetically cram the bad sections into the springs we have
        // TODO can cache this sum and pass it in
        if springs.len() < bads.iter().sum::<usize>() + bads.len() - 1 {
            return 0;
        }
        let bad = bads[0];
        let valid_start = springs[0..bad].iter().all(Spring::maybe_bad);
        if bad == springs.len() {
            if valid_start {
                return 1;
            } else {
                return 0;
            }
        }
        let invalid_possibilities = if springs[0].maybe_good() {
            Self::solve_1(&springs[1..], bads)
        } else {
            0
        };
        if valid_start && springs[bad].maybe_good() {
            invalid_possibilities + Self::solve_1(&springs[bad + 1..], &bads[1..])
        } else {
            invalid_possibilities
        }
    }
}

type Solution = u64;
impl DaySolver<Solution> for Day12 {
    fn part1(input: Vec<String>) -> Option<Solution> {
        let rows = Self::parse(&input);
        Some(
            rows.iter()
                .map(|row| Self::solve_1(&row.springs, &row.bads))
                .sum(),
        )
    }
    fn part2(input: Vec<String>) -> Option<Solution> {
        let rows = Self::parse(&input);
        let rows: Vec<Row> = rows
            .iter()
            .map(|row| {
                let mut springs = vec![];
                springs.extend(row.springs.iter().copied());
                springs.push(Spring::Dunno);
                springs.extend(row.springs.iter().copied());
                springs.push(Spring::Dunno);
                springs.extend(row.springs.iter().copied());
                springs.push(Spring::Dunno);
                springs.extend(row.springs.iter().copied());
                springs.push(Spring::Dunno);
                springs.extend(row.springs.iter().copied());
                let mut bads = vec![];
                bads.extend(row.bads.iter().copied());
                bads.extend(row.bads.iter().copied());
                bads.extend(row.bads.iter().copied());
                bads.extend(row.bads.iter().copied());
                bads.extend(row.bads.iter().copied());
                Row { springs, bads }
            })
            .collect();
        // Don't even try
        // Some(
        //     rows.iter()
        //         .map(|row| {
        //             let x = Self::solve_1(&row.springs, &row.bads);
        //             println!("{row:?} {x}");
        //             x
        //         })
        //         .sum(),
        // );
        None
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_12_a() {
        let row = Row::from(&"???.### 1,1,3".to_string());
        assert_eq!(Day12::solve_1(&row.springs, &row.bads), 1);
    }
    #[test]
    fn test_12_b() {
        let row = Row::from(&".??..??...?##. 1,1,3".to_string());
        assert_eq!(Day12::solve_1(&row.springs, &row.bads), 4);
    }
    #[test]
    fn test_12_c() {
        let row = Row::from(&"?#?#?#?#?#?#?#? 1,3,1,6".to_string());
        assert_eq!(Day12::solve_1(&row.springs, &row.bads), 1);
    }
    #[test]
    fn test_12_d() {
        let row = Row::from(&"????.#...#... 4,1,1".to_string());
        assert_eq!(Day12::solve_1(&row.springs, &row.bads), 1);
    }
    #[test]
    fn test_12_e() {
        let row = Row::from(&"????.######..#####. 1,6,5".to_string());
        assert_eq!(Day12::solve_1(&row.springs, &row.bads), 4);
    }
    #[test]
    fn test_12_f() {
        let row = Row::from(&"?###???????? 3,2,1".to_string());
        assert_eq!(Day12::solve_1(&row.springs, &row.bads), 10);
    }
}
