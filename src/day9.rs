use std::cmp::{max, min};

use regex::Regex;
use crate::day::{Day, Answer};

// A representation of the puzzle inputs.
struct Input {
    coords: Vec<(usize, usize)>,
}

impl Input {
    fn read(text: &str) -> Input 
    {
        let line_re = Regex::new("([0-9]+),([0-9]+)").unwrap();
        let mut coords = Vec::new();

        for line in text.lines() {
            // Process one line
            if let Some(m) = line_re.captures(line) {
                coords.push((m[1].parse::<usize>().unwrap(), 
                             m[2].parse::<usize>().unwrap()));
            }

        }

        Input { coords }
    }

    fn biggest_rect(&self) -> usize {
        let mut biggest_area = 1;

        for i in 0..self.coords.len()-1 {
            for j in i+1..self.coords.len() {
                let dx = self.coords[i].0.abs_diff(self.coords[j].0) + 1;
                let dy = self.coords[i].1.abs_diff(self.coords[j].1) + 1;
                let area = dx * dy;
                if area > biggest_area {
                    biggest_area = area;
                }
            }
        }

        biggest_area
    }

    fn biggest_rect2(&self) -> usize {
        let mut biggest_area = 1;
        let border = Border::new(&self);

        for i in 0..self.coords.len()-1 {
            for j in i+1..self.coords.len() {
                if !border.crosses(self.coords[i], self.coords[j]) {
                    let dx = self.coords[i].0.abs_diff(self.coords[j].0) + 1;
                    let dy = self.coords[i].1.abs_diff(self.coords[j].1) + 1;
                    let area = dx * dy;
                    if area > biggest_area {
                        biggest_area = area;
                        // println!("  Candidate: {i}, {j}");
                    }
                }
            }
        }

        biggest_area
    }
}

struct Border {
    verticals: Vec<(usize, usize, usize)>, // Vec of (col, start_row, end_row)
    horizontals: Vec<(usize, usize, usize)> // Vec of (row, start_col, end_col)
}

impl Border {
    fn new(input: &Input) -> Border {
        // Construct horizontals and verticals
        let mut verticals = Vec::new();
        let mut horizontals = Vec::new();

        for n in 0..input.coords.len() {
            let np1 = (n + 1) % (input.coords.len());
            if input.coords[n].0 == input.coords[np1].0 {
                // Vertical segment
                let x = input.coords[n].0;
                let min_y = min(input.coords[n].1, input.coords[np1].1);
                let max_y = max(input.coords[n].1, input.coords[np1].1);

                verticals.push((x, min_y, max_y));
            }
            else if input.coords[n].1 == input.coords[np1].1 {
                // Horizontal segment
                let y = input.coords[n].1;
                let min_x = min(input.coords[n].0, input.coords[np1].0);
                let max_x = max(input.coords[n].0, input.coords[np1].0);

                horizontals.push((y, min_x, max_x));
            }
            else {
                panic!("There shouldn't be any diagonal segments.");
            }
        }

        Border { verticals, horizontals }
    }

    fn crosses(&self, c1: (usize, usize), c2: (usize, usize)) -> bool {
        // For all four sides of a potential rect, check whether any verticals or horizontals
        // cross the boundary
        let h1 = min(c1.1, c2.1);
        let h2 = max(c1.1, c2.1);
        let v1 = min(c1.0, c2.0);
        let v2 = max(c1.0, c2.0);

        for v in &self.verticals {
            // Check for verticals crossing h1 or h2
            if v1 < v.0 && v2 > v.0 {
                if h1 >= v.1 && h1 <= v.2 {
                    // v crosses h1
                    return true
                }
                if h2 >= v.1 && h2 <= v.2 {
                    // v crosses h2
                    return true
                }
            } 
        }

        for h in &self.horizontals {
            // Check for horizontal h crossing v1 or v2
            if h1 < h.0 && h2 > h.0 {
                if v1 >= h.1 && v1 <= h.2 {
                    // h crosses v1
                    return true;
                }
                if v2 >= h.1 && v2 <= h.2 {
                    // h crosses v2
                    return true;
                }
            } 
        }

        false
    }
}

pub struct Day9 {
}

impl Day9 {
    pub const fn new() -> Self {
        Self { }
    }
}

impl Day for Day9 {

    // Compute Part 1 solution
    fn part1(&self, text: &str) -> Answer {
        // Read input file into Input struct
        let input = Input::read(text);

        Answer::Numeric(input.biggest_rect())
    }

    fn part2(&self, text: &str) -> Answer {

        // Read input file into Input struct
        let input = Input::read(text);

        Answer::Numeric(input.biggest_rect2())
    }
}

#[cfg(test)]
mod test {
    use crate::day9::{Day9, Input};
    use crate::day::{Day, Answer};
    
    const EXAMPLE1: &str = "\
7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3
";

    #[test]
    // Read part 1 example and confirm inputs
    fn test_read_part1() {
        let input = Input::read(EXAMPLE1);

        assert_eq!(input.coords.len(), 8);
        assert_eq!(input.coords[0], (7, 1));
        assert_eq!(input.coords[7], (7, 3));
    }

    #[test]
    fn test_biggest_rect() {
        let input = Input::read(EXAMPLE1);

        assert_eq!(input.biggest_rect(), 50);
    }

    #[test]
    fn test_biggest_rect2() {
        let input = Input::read(EXAMPLE1);

        assert_eq!(input.biggest_rect2(), 24);
    }

    #[test]
    // Compute part 1 result on example 1 and confirm expected value.
    fn test_part1() {
        // Based on the example in part 1.
        let d = Day9::new();
        assert_eq!(d.part1(EXAMPLE1), Answer::Numeric(50));
    }


    #[test]
    // Compute part 2 result on example 2 and confirm expected value.
    fn test_part2() {
        // Based on the example in part 2.
        let d = Day9::new();
        assert_eq!(d.part2(EXAMPLE1), Answer::Numeric(24));
    }
    
}
