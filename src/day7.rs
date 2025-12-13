use std::{collections::HashSet, mem};

use crate::day::{Day, Answer};

// A representation of the puzzle inputs.
struct Input {
    rows: Vec<Vec<char>>,
}

impl Input {
    fn read(text: &str) -> Input 
    {
        let mut rows = Vec::new();
        for line in text.lines() {
            let mut row = Vec::new();
            for c in line.chars() {
                row.push(c);
            }
            rows.push(row);
        }

        Input { rows }
    }

    fn splits(&self) -> usize {
        let mut beams: HashSet<usize> = HashSet::new();
        let mut splits = 0;

        for row in &self.rows {
            for (n, c) in row.iter().enumerate() {
                if *c == 'S' {
                    beams.insert(n);
                }
                if *c == '^' && beams.contains(&n) {
                    // split the beam
                    splits += 1;
                    beams.remove(&n);
                    beams.insert(n-1);
                    beams.insert(n+1);
                }
            }
        }

        splits
    }

    fn timelines(&self) -> usize {
        // Ping pong buffers for prior_timelines and curr_timelines
        let mut timelines_a = vec![0; self.rows[0].len()];
        let mut timelines_b = vec![0; self.rows[0].len()];

        let prior_timelines = &mut timelines_a;
        let curr_timelines = &mut timelines_b;

        for row in &self.rows {
            // Clear old values
            for col_no in 0..row.len() {
                curr_timelines[col_no] = 0;
            }

            // Accumulate timelines
            for col_no in 0..row.len() {
                // Get number of cols flowing from prior
                match row[col_no] {
                    'S' => {
                        // Start a tachyon beam with 1 timeline
                        curr_timelines[col_no] += 1;
                    }
                    '.' => {
                        // Timelines can flow straight down
                        curr_timelines[col_no] += prior_timelines[col_no];
                    }
                    '^' => {
                        // A splitter here duplicates timelines
                        curr_timelines[col_no-1] += prior_timelines[col_no];
                        curr_timelines[col_no+1] += prior_timelines[col_no];
                    }
                    _ => {
                        panic!("Unexpected character in input array: '{}', col:{col_no}", row[col_no]);
                    }
                }
            }

            mem::swap(prior_timelines, curr_timelines);
        }

        prior_timelines.iter().sum()
    }
}

pub struct Day7 {
}

impl Day7 {
    pub const fn new() -> Self {
        Self { }
    }
}

impl Day for Day7 {

    // Compute Part 1 solution
    fn part1(&self, text: &str) -> Answer {
        // Read input file into Input struct
        let input = Input::read(text);

        Answer::Numeric(input.splits())
    }

    fn part2(&self, text: &str) -> Answer {

        // Read input file into Input struct
        let input = Input::read(text);

        Answer::Numeric(input.timelines())
    }
}

#[cfg(test)]
mod test {
    use crate::day7::{Day7, Input};
    use crate::day::{Day, Answer};
    
    const EXAMPLE1: &str = "\
.......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............
";

    #[test]
    // Read part 1 example and confirm inputs
    fn test_read_part1() {
        let input = Input::read(EXAMPLE1);

        assert_eq!(input.rows.len(), 16);
        assert_eq!(input.rows[0].len(), 15);
        assert_eq!(input.rows[0][0], '.');
        assert_eq!(input.rows[0][7], 'S');
        assert_eq!(input.rows[2][7], '^');
    }

    #[test]
    fn test_splits() {
        let input = Input::read(EXAMPLE1);

        assert_eq!(input.splits(), 21);
    }

    #[test]
    fn test_timelines() {
        let input = Input::read(EXAMPLE1);

        assert_eq!(input.timelines(), 40);
    }

    #[test]
    // Compute part 1 result on example 1 and confirm expected value.
    fn test_part1() {
        // Based on the example in part 1.
        let d = Day7::new();
        assert_eq!(d.part1(EXAMPLE1), Answer::Numeric(21));
    }


    #[test]
    // Compute part 2 result on example 2 and confirm expected value.
    fn test_part2() {
        // Based on the example in part 2.
        let d = Day7::new();
        assert_eq!(d.part2(EXAMPLE1), Answer::Numeric(40));
    }
    
}
