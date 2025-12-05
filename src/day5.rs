use std::cmp::{max, min};

use crate::day::{Day, Answer};

#[derive(Eq, PartialEq, Ord, PartialOrd)]
struct FreshRange {
    low: usize,
    high: usize,
}


impl FreshRange {
    fn is_fresh(&self, ingredient: usize) -> bool {
        ingredient >= self.low && ingredient <= self.high
    }
}

// A representation of the puzzle inputs.
struct Input {
    fresh_ranges: Vec<FreshRange>,
    ingredients: Vec<usize>,
}

impl Input {
    fn read(text: &str) -> Input 
    {
        let mut fresh_ranges = Vec::new();
        let mut ingredients = Vec::new();
        let mut in_ranges = true;

        // Process fresh ranges
        for line in text.lines() {
            if in_ranges {
                if !line.contains("-") {
                    // separator line
                    in_ranges = false;
                }
                else {
                    let mut numbers = line.split("-");
                    let low = numbers.nth(0).unwrap().parse::<usize>().unwrap();
                    let high = numbers.nth(0).unwrap().parse::<usize>().unwrap();
                    fresh_ranges.push(FreshRange{low, high});
                }
            }
            else {
                let ingredient = line.parse::<usize>().unwrap();
                ingredients.push(ingredient);
            }
        }

        Input { fresh_ranges, ingredients }
    }


    fn is_fresh(&self, ingredient: usize) -> bool {
        for range in &self.fresh_ranges {
            if range.is_fresh(ingredient) {
                return true;
            }
        }
        
        false
    }
}

pub struct Day5 {
}

impl Day5 {
    pub const fn new() -> Self {
        Self { }
    }
}

impl Day for Day5 {

    // Compute Part 1 solution
    fn part1(&self, text: &str) -> Answer {
        // Read input file into Input struct
        let input = Input::read(text);

        let num_fresh = input.ingredients.iter()
            .filter(|ingredient| { input.is_fresh(**ingredient) })
            .count();

        Answer::Numeric(num_fresh)
    }

    fn part2(&self, text: &str) -> Answer {

        // Read input file into Input struct
        let mut input = Input::read(text);
        let mut reduced = true;
        while reduced {
            reduced = false;
            'outer:
            for i in 0 .. input.fresh_ranges.len()-1 {
                for j in i+1..input.fresh_ranges.len() {
                    // See if we can merge ranges i and j
                    if (input.fresh_ranges[i].low >= input.fresh_ranges[j].low &&
                        input.fresh_ranges[i].low <= input.fresh_ranges[j].high) ||
                       (input.fresh_ranges[i].high >= input.fresh_ranges[j].low &&
                        input.fresh_ranges[i].high <= input.fresh_ranges[j].high) ||
                       (input.fresh_ranges[j].low >= input.fresh_ranges[i].low &&
                        input.fresh_ranges[j].low <= input.fresh_ranges[i].high) ||
                       (input.fresh_ranges[j].high >= input.fresh_ranges[i].low &&
                        input.fresh_ranges[j].high <= input.fresh_ranges[i].high) {
                            // They overlap.
                            let new_low = min(input.fresh_ranges[i].low, input.fresh_ranges[j].low);
                            let new_high = max(input.fresh_ranges[i].high, input.fresh_ranges[j].high);

                            // Delete former ranges
                            input.fresh_ranges.remove(j);
                            input.fresh_ranges.remove(i);
                            input.fresh_ranges.push(FreshRange{ low: new_low, high: new_high });
                            reduced = true;
                            break 'outer;
                        }
                }
            }
        }

        input.fresh_ranges.sort();

        // Count all the values in all the ranges
        let num_fresh = input.fresh_ranges.iter()
            .map(|r| { r.high - r.low + 1 })
            .sum();

        Answer::Numeric(num_fresh)
    }
}

#[cfg(test)]
mod test {
    use crate::day5::{Day5, Input};
    use crate::day::{Day, Answer};
    
    const EXAMPLE1: &str = "\
3-5
10-14
16-20
12-18

1
5
8
11
17
32
";

    #[test]
    // Read part 1 example and confirm inputs
    fn test_read() {
        let input = Input::read(EXAMPLE1);

        assert_eq!(input.fresh_ranges.len(), 4);
        assert_eq!(input.ingredients.len(), 6);
    }

    #[test]
    fn test_freshness() {
        let input = Input::read(EXAMPLE1);

        assert!(!input.is_fresh(1));
        assert!(input.is_fresh(5));
        assert!(!input.is_fresh(8));
        assert!(input.is_fresh(11));
        assert!(input.is_fresh(17));
        assert!(!input.is_fresh(32));
    }

    #[test]
    // Compute part 1 result on example 1 and confirm expected value.
    fn test_part1() {
        // Based on the example in part 1.
        let d = Day5::new();
        assert_eq!(d.part1(EXAMPLE1), Answer::Numeric(3));
    }


    #[test]
    // Compute part 2 result on example 2 and confirm expected value.
    fn test_part2() {
        // Based on the example in part 2.
        let d = Day5::new();
        assert_eq!(d.part2(EXAMPLE1), Answer::Numeric(14));
    }
    
}
