use crate::day::{Day, Answer};

// A representation of the puzzle inputs.
struct Input {
}

impl Input {
    fn read(text: &str) -> Input 
    {
        for _line in text.lines() {
            // Process one line
        }

        Input { }
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
        let _input = Input::read(text);

        Answer::None
    }

    fn part2(&self, text: &str) -> Answer {

        // Read input file into Input struct
        let _input = Input::read(text);

        Answer::None
    }
}

#[cfg(test)]
mod test {
    use crate::day5::{Day5, Input};
    use crate::day::{Day, Answer};
    
    const EXAMPLE1: &str = "\
Example input
";

    #[test]
    // Read part 1 example and confirm inputs
    fn test_read_part1() {
        let _input = Input::read(EXAMPLE1);
    }

    #[test]
    // Compute part 1 result on example 1 and confirm expected value.
    fn test_part1() {
        // Based on the example in part 1.
        let d = Day5::new();
        assert_eq!(d.part1(EXAMPLE1), Answer::None);
    }


    #[test]
    // Compute part 2 result on example 2 and confirm expected value.
    fn test_part2() {
        // Based on the example in part 2.
        let d = Day5::new();
        assert_eq!(d.part2(EXAMPLE1), Answer::None);
    }
    
}
