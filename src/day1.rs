use crate::day::{Day, Answer};

// A representation of the puzzle inputs.
struct Input {
    moves: Vec<isize>,
}

impl Input {
    fn read(text: &str) -> Input 
    {
        let mut moves = Vec::new();
        for line in text.lines() {
            // Process a line
            let thismove = match &line[0..1] {
                "L" => -line[1..].parse::<isize>().unwrap(),
                "R" => line[1..].parse::<isize>().unwrap(),
                _ => panic!("Bad move in input file."),
            };

            moves.push(thismove);
        }

        Input { moves }
    }
}

pub struct Day1 {
}

// Day1
impl Day1 {
    pub const fn new() -> Self {
        Self { }
    }
}

impl Day for Day1 {

    // Compute Part 1 solution
    fn part1(&self, text: &str) -> Answer {
        let input = Input::read(text);

        let mut position = 50;
        let mut zeros = 0;
        for m in input.moves {
            position = (position + m) % 100;
            if position == 0 {
                zeros += 1;
            }
        }

        Answer::Numeric(zeros)
    }

    fn part2(&self, text: &str) -> Answer {
        let input = Input::read(text);

        let mut position = 50;
        let mut zeros: usize = 0;
        for m in input.moves {
            let full_rotations = (m / 100).abs() as usize;
            let partial_rotation = m % 100;

            // count all those full rotation crossings.
            zeros += full_rotations;
            let mut new_position = position + partial_rotation;

            if new_position == 0 {
                // landed on zero.
                zeros += 1;
            } else if new_position > 99 {
                // crossed in positive dir
                zeros += 1;
                new_position -= 100;
            } else if new_position < 0 {
                // crossed in negative dir
                if position != 0 {
                     zeros += 1;
                }
                new_position += 100
            }

            position = new_position;
        }

        Answer::Numeric(zeros)
    }
}

#[cfg(test)]
mod test {

    use crate::day1::{Day1, Input};
    use crate::day::{Day, Answer};
    
    const EXAMPLE1: &str = "\
L68
L30
R48
L5
R60
L55
L1
L99
R14
L82
";

    #[test]
    // Read part 1 example and confirm inputs
    fn test_read_part1() {
        let input = Input::read(EXAMPLE1);
                
        assert_eq!(input.moves.len(), 10);
        assert_eq!(input.moves[0], -68);
        assert_eq!(input.moves[2], 48);
    }

    #[test]
    // Compute part 1 result on example 1 and confirm expected value.
    fn test_part1() {
        // Based on the example in part 1.
        let d: Day1 = Day1::new();
        assert_eq!(d.part1(EXAMPLE1), Answer::Numeric(3));
    }


    #[test]
    // Compute part 2 result on example 2 and confirm expected value.
    fn test_part2() {
        // Based on the example in part 2.
        let d: Day1 = Day1::new();
        assert_eq!(d.part2(EXAMPLE1), Answer::Numeric(6));
    }
    
}
