use reqwest::blocking::Client;
use std::fmt::Display;
use std::fs::{remove_file, File};
use std::io::{stdin, BufRead, Read, Write};
use std::path::PathBuf;

const YEAR: usize = 2023;
const COOKIE_FILE: &str = ".cookie";

// This might be useful for getting the current level
// fn problem_page_url(day: usize) -> String {
//     format!("https://adventofcode.com/{YEAR}/day/{day}")
// }
fn input_url(day: usize) -> String {
    format!("https://adventofcode.com/{YEAR}/day/{day}/input")
}
// This will be useful if I ever decide to submit answers automagically
// fn submit_url(day: usize) -> String {
//     format!("https://adventofcode.com/{YEAR}/day/{day}/answer")
// }
fn input_file(day: usize) -> PathBuf {
    format!("inputs/day{day}.txt").into()
}

/// Delete the cached cookie file
pub fn clear_cookie() {
    println!("Clearing the session cookie");
    // Failing is fine, maybe the file doesn't exist
    let _ = remove_file(COOKIE_FILE);
}

/// Get the session cookie from the cache, or query the user for it
fn cookie() -> String {
    let path: PathBuf = COOKIE_FILE.into();
    let mut buffer = String::new();
    if path.exists() {
        let mut f = File::open(path).unwrap();
        f.read_to_string(&mut buffer).unwrap();
    } else {
        println!("Please paste the contents of the session cookie: ");
        stdin().read_line(&mut buffer).unwrap();
        let mut f = File::create(path).unwrap();
        f.write_all(buffer.as_bytes()).unwrap();
    }
    buffer.trim().into()
}

/// Get the raw input from the Advent of Code website
pub fn download_input(day: usize) -> Vec<String> {
    println!("Downloading the input for day {day}");
    // Let it fail if there's a failure
    let raw_input = Client::new()
        .get(input_url(day))
        .header("Cookie", format!("session={}", cookie()))
        .send()
        .unwrap()
        .text()
        .unwrap();
    // Write the input to a file for later usage
    let mut file = File::create(input_file(day)).unwrap();
    file.write_all(raw_input.as_bytes()).unwrap();
    raw_input.lines().map(String::from).collect()
}

/// Get the input from the cache, or download it if necessary
fn get_input(day: usize) -> Vec<String> {
    let path = input_file(day);
    if path.exists() {
        let file = File::open(path).unwrap();
        std::io::BufReader::new(file)
            .lines()
            .map(Result::unwrap)
            .collect()
    } else {
        download_input(day)
    }
}

pub trait DaySolver<T>
where
    T: Display,
{
    /// The implemented solution to part 1
    fn part1(input: Vec<String>) -> Option<T>;
    /// The implemented solution to part 2
    fn part2(input: Vec<String>) -> Option<T>;
    /// Get the input for the day, call part1, and cast the result to a String
    fn solve_part1(day: usize) -> Option<String> {
        let input = get_input(day);
        Self::part1(input).map(|solution| format!("{}", solution))
    }
    /// Get the input for the day, call part2, and cast the result to a String
    fn solve_part2(day: usize) -> Option<String> {
        let input = get_input(day);
        Self::part2(input).map(|solution| format!("{}", solution))
    }
}
