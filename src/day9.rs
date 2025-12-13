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

    /*
    fn biggest_rect(&self) -> usize {
        // Coordinates of biggest rectangle
        let mut a = 0;
        let mut b = 0;
        let mut biggest_area = 1;

        let mut stable = false;
        while !stable {
            // declare stability until we find a change
            stable = true;

            // On this iteration, try replacing the one that was stable last time.
            (a, b) = (b, a);

            // Check all nodes to see if they make a better 'b' with a.
            for n in 0..self.coords.len() {
                let dx = self.coords[a].0.abs_diff(self.coords[n].0) + 1;
                let dy = self.coords[a].1.abs_diff(self.coords[n].1) + 1;
                let area = dx * dy;
                if area > biggest_area {
                    stable = false;
                    biggest_area = area;
                    b = n;
                }
            }
        }

        println!("Stabilized at {a}: ({},{}) to {b}: ({},{}), area: {biggest_area}",
                 self.coords[a].0, self.coords[a].1,
                self.coords[b].0, self.coords[b].1);

        biggest_area
    }
    */

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

        Answer::Numeric(input.biggest_rect())
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
        assert_eq!(d.part2(EXAMPLE1), Answer::None);
    }
    
}
