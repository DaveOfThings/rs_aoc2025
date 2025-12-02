use std::{cmp::{max, min}, collections::HashSet};

use crate::day::{Day, Answer};

struct Range {
    low: usize,
    high: usize,
}

impl Range {
    const MAX_DIGITS: u32 = 10;

    fn invalids(&self) -> HashSet<usize> {
        let mut invalids = HashSet::new();
        println!("Looking for invalid values in range {} - {}", self.low, self.high);

        for exp in 1..6 {
            let base = 10_usize.pow(exp);          // 10, 100, ... 1000000
            let min_kernel = base/10;              //  1,  10, ...  100000
            let max_kernel = base-1;               //  9,  99, ...  999999
            let multiplier = base + 1;             // 11, 101, ... 1000001
            let lowest = min_kernel * multiplier;  // 11, 1010, ... 100000100000
            let highest = max_kernel * multiplier; // 99, 9999, ... 999999999999

            if (lowest <= self.high) && (highest >= self.low) {
                // Look for invalid numbers with this base.
                let bottom_kernel = max(min_kernel, (self.low as f64 / multiplier as f64).ceil() as usize);
                let top_kernel = min(max_kernel, (self.high as f64 / multiplier as f64).floor() as usize);

                for k in bottom_kernel ..= top_kernel {
                    println!("    found {}", k * multiplier);
                    invalids.insert(k * multiplier);
                }
            }
        }

        invalids
    }

    fn invalids2(&self) -> HashSet<usize> {
        let mut invalids = HashSet::new();
        println!("Looking for invalid values (part 2) in range {} - {}", self.low, self.high);

        for repeats in 2..=Self::MAX_DIGITS {
            for exp in 1..Self::MAX_DIGITS/repeats+1 {
                let base = 10_usize.pow(exp);          // 10, 100, ... 1000000
                let min_kernel = base/10;              //  1,  10, ...  100000
                let max_kernel = base-1;               //  9,  99, ...  999999
                let mut multiplier = 1;                // 111, 10101, ... 1000001000001 (e.g. for repeats = 3)
                for _ in 1..repeats {
                    multiplier *= base;
                    multiplier += 1;
                }
                println!("Checking with multiplier {multiplier}");
                let lowest = min_kernel * multiplier;  // 11, 1010, ... 100000100000
                let highest = max_kernel * multiplier; // 99, 9999, ... 999999999999

                if (lowest <= self.high) && (highest >= self.low) {
                    // Look for invalid numbers with this base.
                    let bottom_kernel = max(min_kernel, (self.low as f64 / multiplier as f64).ceil() as usize);
                    let top_kernel = min(max_kernel, (self.high as f64 / multiplier as f64).floor() as usize);

                    for k in bottom_kernel ..= top_kernel {
                        println!("    found {}", k * multiplier);
                        invalids.insert(k * multiplier);
                    }
                }
            }

        }

        invalids
    }
}

// A representation of the puzzle inputs.
struct Input {
    ranges: Vec<Range>,
}

impl Input {
    fn read(text: &str) -> Input 
    {
        let mut ranges = Vec::new();

        for line in text.lines() {
            // Process one line
            for part in line.split(",") {
                let components: Vec<usize> = part.split("-")
                    .map(|s| {
                        s.parse::<usize>().unwrap()
                     })
                     .collect();
                assert_eq!(components.len(), 2);
                let low = components[0];
                let high = components[1];

                ranges.push(Range { low, high })
            }
        }

        Input { ranges }
    }
}

pub struct Day2 {
}

// Day2
impl Day2 {
    pub const fn new() -> Self {
        Self { }
    }
}

impl Day for Day2 {

    // Compute Part 1 solution
    fn part1(&self, text: &str) -> Answer {
        // Read input file into Input struct
        let input = Input::read(text);

        let sum = input.ranges.iter()
            .map(|range| {
                range.invalids().iter()
                    .sum::<usize>()
            }).sum();

        Answer::Numeric(sum)
    }

    fn part2(&self, text: &str) -> Answer {

        // Read input file into Input struct
        let input = Input::read(text);

        let sum = input.ranges.iter()
            .map(|range| {
                range.invalids2().iter()
                    .sum::<usize>()
            }).sum();

        Answer::Numeric(sum)
    }
}

#[cfg(test)]
mod test {
    use crate::day2::{Day2, Input, Range};
    use crate::day::{Day, Answer};
    
    const EXAMPLE1: &str = "\
11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124
";

    #[test]
    // Read part 1 example and confirm inputs
    fn test_read_part1() {
        let input = Input::read(EXAMPLE1);

        assert_eq!(input.ranges.len(), 11);
        assert_eq!(input.ranges[0].low, 11);
        assert_eq!(input.ranges[0].high, 22);
    }

    #[test]
    fn test_invalid() {
        let range = Range{low: 11, high:22};
        let invalid_set = range.invalids();
        assert_eq!(invalid_set.len(), 2);
        assert!(invalid_set.contains(&11));
        assert!(invalid_set.contains(&22));

        let range = Range{low: 98, high:115};
        let invalid_set = range.invalids();
        assert_eq!(invalid_set.len(), 1);
        assert!(invalid_set.contains(&99));

        let range = Range{low: 998, high:1012};
        let invalid_set = range.invalids();
        assert_eq!(invalid_set.len(), 1);
        assert!(invalid_set.contains(&1010));

        let range = Range{low: 1188511880, high:1188511890};
        let invalid_set = range.invalids();
        assert_eq!(invalid_set.len(), 1);
        assert!(invalid_set.contains(&1188511885));

        let range = Range{low: 222220, high:222224};
        let invalid_set = range.invalids();
        assert_eq!(invalid_set.len(), 1);
        assert!(invalid_set.contains(&222222));

        let range = Range{low: 1698522, high:1698528};
        let invalid_set = range.invalids();
        assert_eq!(invalid_set.len(), 0);

        let range = Range{low: 446443, high:446449};
        let invalid_set = range.invalids();
        assert_eq!(invalid_set.len(), 1);
        assert!(invalid_set.contains(&446446));

        let range = Range{low: 38593856, high:38593862};
        let invalid_set = range.invalids();
        assert_eq!(invalid_set.len(), 1);
        assert!(invalid_set.contains(&38593859));

        let range = Range{low: 565653, high:565659};
        let invalid_set = range.invalids();
        assert_eq!(invalid_set.len(), 0);
        
        let range = Range{low: 824824821, high:824824827};
        let invalid_set = range.invalids();
        assert_eq!(invalid_set.len(), 0);

        let range = Range{low: 2121212118, high:2121212124};
        let invalid_set = range.invalids();
        assert_eq!(invalid_set.len(), 0);
    }

    
    #[test]
    fn test_invalid2() {
        let range = Range{low: 11, high:22};
        let invalid_set = range.invalids2();
        assert_eq!(invalid_set.len(), 2);
        assert!(invalid_set.contains(&11));
        assert!(invalid_set.contains(&22));

        let range = Range{low: 98, high:115};
        let invalid_set = range.invalids2();
        assert_eq!(invalid_set.len(), 2);
        assert!(invalid_set.contains(&99));
        assert!(invalid_set.contains(&111));

        let range = Range{low: 998, high:1012};
        let invalid_set = range.invalids2();
        assert_eq!(invalid_set.len(), 2);
        assert!(invalid_set.contains(&1010));
        assert!(invalid_set.contains(&999));

        let range = Range{low: 1188511880, high:1188511890};
        let invalid_set = range.invalids2();
        assert_eq!(invalid_set.len(), 1);
        assert!(invalid_set.contains(&1188511885));

        let range = Range{low: 222220, high:222224};
        let invalid_set = range.invalids2();
        assert_eq!(invalid_set.len(), 1);
        assert!(invalid_set.contains(&222222));

        let range = Range{low: 1698522, high:1698528};
        let invalid_set = range.invalids2();
        assert_eq!(invalid_set.len(), 0);

        let range = Range{low: 446443, high:446449};
        let invalid_set = range.invalids2();
        assert_eq!(invalid_set.len(), 1);
        assert!(invalid_set.contains(&446446));

        let range = Range{low: 38593856, high:38593862};
        let invalid_set = range.invalids2();
        assert_eq!(invalid_set.len(), 1);
        assert!(invalid_set.contains(&38593859));

        let range = Range{low: 565653, high:565659};
        let invalid_set = range.invalids2();
        assert_eq!(invalid_set.len(), 1);
        assert!(invalid_set.contains(&565656));
        
        let range = Range{low: 824824821, high:824824827};
        let invalid_set = range.invalids2();
        assert_eq!(invalid_set.len(), 1);
        assert!(invalid_set.contains(&824824824));

        let range = Range{low: 2121212118, high:2121212124};
        let invalid_set = range.invalids2();
        assert_eq!(invalid_set.len(), 1);
        assert!(invalid_set.contains(&2121212121));
    }

    #[test]
    // Compute part 1 result on example 1 and confirm expected value.
    fn test_part1() {
        // Based on the example in part 1.
        let d: Day2 = Day2::new();
        assert_eq!(d.part1(EXAMPLE1), Answer::Numeric(1227775554));
    }

    #[test]
    // Compute part 2 result on example 2 and confirm expected value.
    fn test_part2() {
        // Based on the example in part 2.
        let d: Day2 = Day2::new();
        assert_eq!(d.part2(EXAMPLE1), Answer::Numeric(4174379265));
    }
    
}
