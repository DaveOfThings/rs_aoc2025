use crate::day::{Day, Answer};

struct Bank {
    cells: Vec<usize>,
}

impl Bank {
    fn from_line(line: &str) -> Bank {
        let mut cells = Vec::new();
        for c in line.chars() {
            if let Some(n) = c.to_digit(10) {
                cells.push(n as usize);
            }
        }

        Bank { cells }
    }

    fn max_jolts(&self, n: usize) -> usize {
        let mut result = 0;
        let mut first_avail_idx = 0;

        for place in 0..n {
            let mut best = 0;
            let mut best_idx = 0;

            // Find best digit for this place
            for n in first_avail_idx..self.cells.len()-(n-1)+place {
                if self.cells[n] > best {
                    best = self.cells[n];
                    best_idx = n;
                }
            }

            // Adopt the best digit we found
            result *= 10;
            result += best;
            first_avail_idx = best_idx+1;
        }

        result
    }
}


// A representation of the puzzle inputs.
struct Input {
    banks: Vec<Bank>,
}

impl Input {
    fn read(text: &str) -> Input 
    {
        let mut banks = Vec::new();

        for line in text.lines() {
            // Process one line
            let bank = Bank::from_line(line);
            banks.push(bank);
        }

        Input { banks }
    }
}

pub struct Day3 {
}

impl Day3 {
    pub const fn new() -> Self {
        Self { }
    }
}

impl Day for Day3 {

    // Compute Part 1 solution
    fn part1(&self, text: &str) -> Answer {
        // Read input file into Input struct
        let input = Input::read(text);

        let sum_jolts = input.banks.iter()
            .map(|b| {
                b.max_jolts(2) as usize
            })
            .sum();

        Answer::Numeric(sum_jolts)
    }

    fn part2(&self, text: &str) -> Answer {
        // Read input file into Input struct
        let input = Input::read(text);

        let sum_jolts = input.banks.iter()
            .map(|b| {
                b.max_jolts(12) as usize
            })
            .sum();

        Answer::Numeric(sum_jolts)
    }
}

#[cfg(test)]
mod test {
    use crate::day3::{Day3, Input};
    use crate::day::{Day, Answer};
    
    const EXAMPLE1: &str = "\
987654321111111
811111111111119
234234234234278
818181911112111
";

    #[test]
    // Read part 1 example and confirm inputs
    fn test_read_part1() {
        let input = Input::read(EXAMPLE1);

        assert_eq!(input.banks.len(), 4);
        assert_eq!(input.banks[0].cells.len(), 15);
        assert_eq!(input.banks[0].cells[0], 9);
    }

    #[test]
    fn test_max_jolts() {
        let input = Input::read(EXAMPLE1);

        assert_eq!(input.banks[0].max_jolts(2), 98);
        assert_eq!(input.banks[1].max_jolts(2), 89);
        assert_eq!(input.banks[2].max_jolts(2), 78);
        assert_eq!(input.banks[3].max_jolts(2), 92);
    }

    #[test]
    fn test_max_jolts2() {
        let input = Input::read(EXAMPLE1);

        assert_eq!(input.banks[0].max_jolts(12), 987654321111);
        assert_eq!(input.banks[1].max_jolts(12), 811111111119);
        assert_eq!(input.banks[2].max_jolts(12), 434234234278);
        assert_eq!(input.banks[3].max_jolts(12), 888911112111);
    }

    #[test]
    // Compute part 1 result on example 1 and confirm expected value.
    fn test_part1() {
        // Based on the example in part 1.
        let d = Day3::new();
        assert_eq!(d.part1(EXAMPLE1), Answer::Numeric(357));
    }


    #[test]
    // Compute part 2 result on example 2 and confirm expected value.
    fn test_part2() {
        // Based on the example in part 2.
        let d = Day3::new();
        assert_eq!(d.part2(EXAMPLE1), Answer::Numeric(3121910778619));
    }
    
}
