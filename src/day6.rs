use crate::day::{Day, Answer};
use regex::Regex;

#[derive(PartialEq, Debug)]
enum Operation {
    ADD,
    MUL,
}

// A representation of the puzzle inputs.
struct Input {
    operands: Vec<Vec<usize>>,
    operations: Vec<Operation>,
    op_cols: Vec<usize>,
    operand_text: Vec<Vec<char>>,
}

impl Input {
    
    fn read(text: &str) -> Input 
    {
        let operand_re: Regex = Regex::new("([0-9]+)").unwrap();
        let operation_re: Regex = Regex::new("([\\*\\+])").unwrap();
        let mut operands= Vec::new();
        let mut operations = Vec::new();
        let mut op_cols = Vec::new();
        let mut operand_text = Vec::new();

        for line in text.lines() {
            // Process one line

            if operand_re.is_match(line) {
                let operand_line = line.chars().collect::<Vec<char>>();
                operand_text.push(operand_line);

                let mut line_operands = Vec::new();
                for operand in operand_re.find_iter(line) {
                    let n = operand.as_str().parse::<usize>().unwrap();
                    line_operands.push(n);
                }
                operands.push(line_operands);

            }
            else if operation_re.is_match(line) {
                for operation in operation_re.find_iter(line) {
                    let col = operation.start();
                    op_cols.push(col);

                    let op = match operation.as_str() {
                        "+" => Operation::ADD,
                        "*" => Operation::MUL,
                        _ => { panic!("Bad operation in input.")}
                    };
                    operations.push(op);
                }
            }
            else {
                panic!("Couldn't parse input line.");
            }
        }

        Input { operands, operations, op_cols, operand_text }
    }

    fn eval(&self, n: usize) -> usize {
        let value = match self.operations[n] {
            Operation::ADD => {
                self.operands.iter()
                    .map(|row| { row[n] })
                    .sum()
            }
            Operation::MUL => {
                self.operands.iter()
                    .map(|row| { row[n] })
                    .product()
            }
        };

        value
    }

    fn eval2(&self, n: usize) -> usize {
        // TODO: Find column number for operation n
        // form operands, in weird ordering.
        let mut v_operands = Vec::new();

        let mut col = self.op_cols[n];
        while col < self.operand_text[0].len() {
            // Form number for this col
            let mut value = 0_usize;
            for row in 0..self.operands.len() {
                let c = self.operand_text[row][col];
                if c.is_digit(10) {
                    value *= 10;
                    value += self.operand_text[row][col].to_digit(10).unwrap() as usize;
                }
            }
            if value == 0 {
                break;
            }
            else {
                v_operands.push(value);
                // println!("operand: {value}")
            }
            col += 1;
        }

        let value = match self.operations[n] {
            Operation::ADD => {
                v_operands.iter()
                    .sum()
            }
            Operation::MUL => {
                v_operands.iter()
                    .product()
            }
        };

        // println!("    result: {value}");

        value
    }
}

pub struct Day6 {
}

impl Day6 {
    pub const fn new() -> Self {
        Self { }
    }
}

impl Day for Day6 {

    // Compute Part 1 solution
    fn part1(&self, text: &str) -> Answer {
        // Read input file into Input struct
        let input = Input::read(text);

        let mut sum = 0;
        for n in 0..input.operations.len() {
            sum += input.eval(n);
        }

        Answer::Numeric(sum)
    }

    fn part2(&self, text: &str) -> Answer {

        // Read input file into Input struct
        let input = Input::read(text);

        let mut sum = 0;
        for n in 0..input.operations.len() {
            sum += input.eval2(n);
        }

        Answer::Numeric(sum)
    }
}

#[cfg(test)]
mod test {
    use crate::day6::{Day6, Input, Operation};
    use crate::day::{Day, Answer};
    
    const EXAMPLE1: &str = "\
123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  
";

    #[test]
    // Read part 1 example and confirm inputs
    fn test_read_part1() {
        let input = Input::read(EXAMPLE1);

        assert_eq!(input.operands.len(), 3);
        assert_eq!(input.operands[0].len(), 4);
        assert_eq!(input.operands[0][0], 123);
        assert_eq!(input.operands[0][1], 328);
        assert_eq!(input.operands[1][0], 45);
        assert_eq!(input.operations.len(), 4);
        assert_eq!(input.operations[0], Operation::MUL);
        assert_eq!(input.operations[1], Operation::ADD);
        assert_eq!(input.operand_text.len(), 3);
        assert_eq!(input.operand_text[0].len(), 15);
    }

    #[test]
    fn test_eval() {
        let input = Input::read(EXAMPLE1);

        assert_eq!(input.eval(0), 33210);
        assert_eq!(input.eval(1), 490);
        assert_eq!(input.eval(2), 4243455);
        assert_eq!(input.eval(3), 401);
    }

    #[test]
    fn test_eval2() {
        let input = Input::read(EXAMPLE1);

        assert_eq!(input.eval2(3), 1058);
        assert_eq!(input.eval2(2), 3253600);
        assert_eq!(input.eval2(1), 625);
        assert_eq!(input.eval2(0), 8544);
    }

    #[test]
    // Compute part 1 result on example 1 and confirm expected value.
    fn test_part1() {
        // Based on the example in part 1.
        let d = Day6::new();
        assert_eq!(d.part1(EXAMPLE1), Answer::Numeric(4277556));
    }


    #[test]
    // Compute part 2 result on example 2 and confirm expected value.
    fn test_part2() {
        // Based on the example in part 2.
        let d = Day6::new();
        assert_eq!(d.part2(EXAMPLE1), Answer::Numeric(3263827));
    }
    
}
