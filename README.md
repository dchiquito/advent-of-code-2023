# advent-of-code-2023

My [Advent of Code 2023](https://adventofcode.com/2023) solutions and toolkit.

## Usage

### Setting up
The `populate.sh` script uses `template.rs` to generate the solution files, which all live in `src/day*.rs`. If `populate.sh` is called with an argument between 1 and 25, it will generate that day's file. If it is called with no arguments, it will generate all files. Beware that this will erase any work done on those solutions.

### The Solution File
Each day has a solution file that initially looks something like this:
```rust
use crate::util::DaySolver;

type Solution = usize;

pub struct Day7();

impl Day7 {
    fn parse(input: Vec<String>) -> usize {
        0
    }

    fn solve(input: Vec<String>) -> Option<Solution> {
        let data = Self::parse(input);
        None
    }
}

impl DaySolver<Solution> for Day7 {
    fn part1(input: Vec<String>) -> Option<Solution> {
        Self::solve(input)
    }
    fn part2(input: Vec<String>) -> Option<Solution> {
        Self::solve(input)
    }
}
```
The only thing that is strictly required is a struct called `Day7` (or whichever day it might be) which implements `DaySolver<T> where T: Display`. For convenience, any `Display`able type may be used as the solution.

The `part1` and `part2` methods take the puzzle input split into lines and return an `Option<Solution>`. If `None` is returned, they are assumed to be unimplemented.

When working on past Advent of Codes, I generally parsed the input into a more accessible data structure, then passed that input to a solver function. I included stubs for those methods for convenience. You should anticipate removing or modifying them.

### CLI
Once you have completed (or at least started) on a solution, you can run it like so:

```bash
# Run the code for day 7
cargo run 7
```

If it has not yet done so, it will prompt you for your session cookie. You can find this by logging in to [Advent of Code 2023](https://adventofcode.com/2023), opening up the browser's DevTools, and checking the Cookies in the Storage tab. The cookie is stored in a `.cookie` file in the repo.

The session cookie is required to download the input file, which practically all days have if I recall correctly. The file will be stored in `inputs/day*.txt`. Once downloaded, the CLI will use this file rather than redownloading every time the solution is run. This allows you to edit the contents of the file for debugging and testing.

Here are some more things the CLI can do:

```bash
# Clear the .cookie file
# Note the -- used to differentiate between arguments for cargo and arguments for the CLI
cargo run -- --cookie

# Recalculate all solutions
cargo run -- --all

# Run day 15, but download the input file again
cargo run 15 --pull
```
