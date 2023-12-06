#![allow(unused)]
use crate::util::DaySolver;

type Solution = usize;

pub struct Day6();

pub struct Race {
    time: usize,
    distance: usize,
}

impl Race {
    fn ways_to_win(&self) -> usize {
        let width = (((self.time * self.time) - (4 * self.distance)) as f64).sqrt();
        // We are cheating and using epsilon values to find only values that win, not just tie
        if self.time % 2 == 0 {
            ((((width - 0.0001) / 2.0).floor() as usize) * 2) + 1
        } else {
            2 * ((width + 0.9999) / 2.0).floor() as usize
        }
        // let center = self.time as f64 / 2.0;
        // let min = center - (width / 2.0);
        // let max = center + (width / 2.0);
        // println!("[{min}..{max}]");
        // let min = (min + 0.0001).ceil() as usize;
        // let max = (max - 0.0001).floor() as usize;
        // println!("[{min}..{max}]");
        // println!("{}", max - min + 1);
        // max - min + 1
    }
}

impl Day6 {
    pub fn parse1(input: &[String]) -> [Race; 4] {
        let time1 = input[0][10..15].trim_start().parse().unwrap();
        let dist1 = input[1][10..15].trim_start().parse().unwrap();
        let time2 = input[0][17..22].trim_start().parse().unwrap();
        let dist2 = input[1][17..22].trim_start().parse().unwrap();
        let time3 = input[0][24..29].trim_start().parse().unwrap();
        let dist3 = input[1][24..29].trim_start().parse().unwrap();
        let time4 = input[0][31..36].trim_start().parse().unwrap();
        let dist4 = input[1][31..36].trim_start().parse().unwrap();
        [
            Race {
                time: time1,
                distance: dist1,
            },
            Race {
                time: time2,
                distance: dist2,
            },
            Race {
                time: time3,
                distance: dist3,
            },
            Race {
                time: time4,
                distance: dist4,
            },
        ]
    }
    pub fn parse2(input: &[String]) -> Race {
        let time = input[0][10..]
            .chars()
            .filter(|c| c != &' ')
            .collect::<String>()
            .parse()
            .unwrap();
        let distance = input[1][10..]
            .chars()
            .filter(|c| c != &' ')
            .collect::<String>()
            .parse()
            .unwrap();
        Race { time, distance }
    }
}

impl DaySolver<Solution> for Day6 {
    fn part1(input: Vec<String>) -> Option<Solution> {
        let races = Self::parse1(&input);
        Some(
            races
                .iter()
                .map(Race::ways_to_win)
                .reduce(|a, b| a * b)
                .unwrap(),
        )
    }
    fn part2(input: Vec<String>) -> Option<Solution> {
        let race = Self::parse2(&input);
        Some(race.ways_to_win())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_part1() {
        test_part1_helper(7, 9, 4);
        test_part1_helper(15, 40, 8);
        // This test is somehow incorrect, despite being given in the problem
        test_part1_helper(30, 200, 9);
        test_part1_helper(71530, 940200, 71503);
    }

    fn test_part1_helper(time: usize, distance: usize, ways_to_win: usize) {
        assert_eq!(Race { time, distance }.ways_to_win(), ways_to_win);
    }
}
