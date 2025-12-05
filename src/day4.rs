use std::collections::HashSet;

use crate::day::{Day, Answer};

// A representation of the puzzle inputs.
struct Input {
    rows: isize,
    cols: isize,
    occupied: HashSet<(isize, isize)>,
}

impl Input {
    fn read(text: &str) -> Input 
    {
        let mut rows: isize = 0;
        let mut cols: isize = 0;
        let mut occupied = HashSet::new();

        for line in text.lines() {
            // Process one line
            let mut col = 0;
            for c in line.chars() {
                match c {
                    '.' => {
                        col += 1;
                    }
                    '@' => {
                        occupied.insert((col, rows));
                        col += 1;
                    }
                    _ => ()
                }
                if col > cols {
                    cols = col;
                }
            }

            rows += 1;
            // Process one line
        }

        Input { rows, cols, occupied }
    }

    fn neighbors(&self, loc: &(isize, isize)) -> usize {
        let (x, y) = loc;
        let mut count = 0;
        for (dx, dy) in 
            [(-1, -1), (0, -1), (1, -1),
             (-1,  0),          (1,  0),
             (-1,  1), (0,  1), (1,  1)] {
            if self.occupied.contains(&(x+dx, y+dy)) {
                count += 1;
            }
        }

        count
    }
}

pub struct Day4 {
}

impl Day4 {
    pub const fn new() -> Self {
        Self { }
    }
}

impl Day for Day4 {

    // Compute Part 1 solution
    fn part1(&self, text: &str) -> Answer {
        // Read input file into Input struct
        let input = Input::read(text);

        let mut count = 0;
        for x in 0..input.cols {
            for y in 0..input.rows {
                if input.occupied.contains(&(x, y)) && input.neighbors(&(x, y)) < 4 {
                    count += 1;
                }
            }
        }

        Answer::Numeric(count)
    }

    fn part2(&self, text: &str) -> Answer {

        // Read input file into Input struct
        let mut input = Input::read(text);
        let mut removed = true;
        let mut num_removed = 0;

        while removed {
            removed = false;

            for x in 0..input.cols {
                for y in 0..input.rows {
                    if input.occupied.contains(&(x, y)) && (input.neighbors(&(x, y)) < 4) {
                        // remove this roll
                        input.occupied.remove(&(x, y));
                        removed = true;
                        num_removed += 1;
                    }
                }
            }
        }

        Answer::Numeric(num_removed)
    }
}

#[cfg(test)]
mod test {
    use crate::day4::{Day4, Input};
    use crate::day::{Day, Answer};
    
    const EXAMPLE1: &str = "\
..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.
";

    #[test]
    // Read part 1 example and confirm inputs
    fn test_read() {
        let input = Input::read(EXAMPLE1);

        assert_eq!(input.rows, 10);
        assert_eq!(input.cols, 10);
        assert!(input.occupied.contains(&(0, 2)));
        assert!(!input.occupied.contains(&(0, 0)));
    }

    #[test]
    fn test_neighbors() {
        let input = Input::read(EXAMPLE1);

        assert_eq!(input.neighbors(&(0, 0)), 2);
        assert_eq!(input.neighbors(&(1, 0)), 4);
        assert_eq!(input.neighbors(&(1, 3)), 7);
    }

    #[test]
    // Compute part 1 result on example 1 and confirm expected value.
    fn test_part1() {
        // Based on the example in part 1.
        let d = Day4::new();
        assert_eq!(d.part1(EXAMPLE1), Answer::Numeric(13));
    }


    #[test]
    // Compute part 2 result on example 2 and confirm expected value.
    fn test_part2() {
        // Based on the example in part 2.
        let d = Day4::new();
        assert_eq!(d.part2(EXAMPLE1), Answer::Numeric(43));
    }
    
}
