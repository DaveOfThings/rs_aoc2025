use std::{cmp::Ordering, collections::{HashMap, HashSet, VecDeque}};
use std::cmp::min;
use std::mem::swap;
use num_rational::Rational32;
use regex::Regex;
use num_traits::Signed;

use crate::day::{Day, Answer};

struct MachineDesc {
    start_indicator: usize,
    button_masks: Vec<usize>,
    button_vecs: Vec<Vec<usize>>,
    joltage: Vec<usize>,
}

impl MachineDesc {

    const ACTIVATION_RE: &str = "\\[([\\.\\#]+)\\]";  // Matches [..##.], cap[1]: ..##.
    const BUTTON_RE: &str = "\\(([0-9](,[0-9])*)\\)";  // Matches (1,2,3), cap[1]: 1,2,3
    const JOLTAGE_RE: &str = "\\{([0-9]+(,[0-9]+)*)\\}"; // Matches {10,8,2}, cap[1]: 10,8,2

    fn from_line(line: &str) -> MachineDesc {

        let mut start_indicator = 0;
        let mut button_masks = Vec::new();
        let mut button_vecs = Vec::new();
        let mut joltage = Vec::new();

        let activation_re = Regex::new(Self::ACTIVATION_RE).unwrap();
        let button_re = Regex::new(Self::BUTTON_RE).unwrap();
        let joltage_re = Regex::new(Self::JOLTAGE_RE).unwrap();

        // Process start indicator
        if let Some(cap) = activation_re.captures(line) {
            // Got start indicator, e.g. "..##.", in cap[1]
            let mut mask = 1;
            for c in cap[1].chars() {
                match c {
                    '.' => {
                        mask <<= 1;
                    }
                    '#' => {
                        start_indicator |= mask;
                        mask <<= 1;
                    }
                    _ => ()
                }
            }
        }
        else {
            panic!("No activation pattern found.")
        }

        // Process buttons
        for cap in button_re.captures_iter(line) {
            let mut button_bitmask = 0;
            let mut button_vec = Vec::new();

            // cap[1] should be the internals of one button, e.g. "1,2,4"
            // note that only single digit values appear so we use to_digit, not parse.
            for c in cap[1].chars() {
                if let Some(n) = c.to_digit(10) {
                    button_bitmask |= 1 << n;
                    button_vec.push(n as usize);
                }
            }
            button_masks.push(button_bitmask);
            button_vecs.push(button_vec);
        }
        assert!(button_masks.len() > 0, "No buttons found.");

        // Process joltage
        if let Some(cap) = joltage_re.captures(line) {
            // Got joltage, e.g. "1,12,9", in cap[1]
            // Note multi-digit values appear, so we use parse(), not to_digit.
            for s in cap[1].split(",") {
                if let Ok(n) = s.parse::<usize>() {
                    joltage.push(n);
                }
                else {
                    panic!("Bad parse for joltage: '{s}'")
                }
            }
        }
        else {
            panic!("No joltage found.");
        }

        MachineDesc { start_indicator, button_masks, button_vecs, joltage }
    }

    // Find the shortest activation sequence
    fn act_seq(&self) -> Vec<usize> {
        // println!("Looking for activation state: 0x{:02x}", self.start_indicator);

        // states to explore from
        let mut to_explore: VecDeque<usize> = VecDeque::new();

        // states we found and how we got to them (vec of button values)
        let mut states_found: HashMap<usize, Vec<usize>> = HashMap::new();

        to_explore.push_back(0);
        states_found.insert(0, Vec::new());
        let mut found = None;
        'outer:
        while to_explore.len() > 0 {
            // pop next state to explore
            let state = to_explore.pop_front().unwrap();

            // println!("Exploring from 0x{state:02x}");
            
            // find all new things we can generate
            for (id, button) in self.button_masks.iter().enumerate() {
                let next_state = state ^ button;
                // println!("  Applying button 0x{button:02x} to get 0x{next_state:02x}");

                if next_state == self.start_indicator {
                    // We hit on the solution, break out of loop
                    // println!("  Found the start state!");

                    let mut seq = states_found.get(&state).unwrap().clone();
                    seq.push(id);
                    found = Some(seq);
                    break 'outer;
                }
                else if !states_found.contains_key(&next_state) {
                    // This state is unexplored, push it to be explored further
                    // println!("  Found a new state to explore");
                    let mut seq = states_found.get(&state).unwrap().clone();
                    seq.push(id);
                    states_found.insert(next_state, seq);
                    to_explore.push_back(next_state);
                }
                else {
                    // We already found this state, ignore it
                    // println!("  Already found this.");
                }
                
            }
        }

        if let Some(seq) = found {
            // println!("Found activation seq: {seq:?}");
            seq
        }
        else {
            panic!("No activation sequence found.")
        }
    }

    fn compare_rows(row1: &Vec<Rational32>, row2: &Vec<Rational32>) -> isize {
        assert_eq!(row1.len(), row2.len());

        for col in 0..row1.len() {
            if row1[col] > row2[col] { return 1; }   // row1 is greater
            if row1[col] < row2[col] { return -1; }  // row2 is greater
        }

        // Everything is equal
        return 0;
    }

    fn print_m(m: &Vec<Vec<Rational32>>) {
        for row in m.iter() {
            for cell in row.iter() {
                print!("{} ", (*cell.numer() as f32)/(*cell.denom() as f32));
            }
            println!("");
        }
    }

    fn print_v(m: &Vec<Rational32>) {
        for cell in m.iter() {
            print!("{} ", (*cell.numer() as f32)/(*cell.denom() as f32));
        }
        println!("");
    }

    // TODO-DW : This special gaussian elimination routine depends on all elements of incoming array to be 0 or 1.
    // (All except the augmented column, that is.)  It tries to preserve all elements as (-1, 0, 1) throughout the
    // computations below.  (Can we prove this?)
    fn ga(m: &mut Vec<Vec<Rational32>>) {
        // diagonal length is minimum of rows or columns (not counting augmented column)
        let n_rows = m.len();
        let n_cols = m[0].len();

        let diag_len = min(n_rows, n_cols-1);

        for n in 0..diag_len {
            /*
            // Sort the remaining rows
            for row1 in n..n_rows-1 {
                for row2 in row1+1..n_rows {
                    if Self::compare_rows(&m[row1], &m[row2]) < 0 {
                        m.swap(row1, row2);
                    }
                }
            }
            */

            /* */
            // Before pivoting, negate any row with a negative value in col n
            for row in n..n_rows {
                if m[row][n] < Rational32::ZERO {
                    for col in n..n_cols {
                        m[row][col] = -m[row][col];
                    }
                }
            }

            // Pivot rows and columns to get largest value in m[n][n]
            let mut max_row = n;
            let mut max_col = n;
            let mut max_val = m[n][n].abs();
            for row in n..n_rows {
                for col in n..n_cols-1 {
                    if m[row][col].abs() > max_val {
                        max_row = row;
                        max_col = col;
                        max_val = m[row][col].abs();
                    }
                }
            }

            // swap row and/or column to put max in pivot position
            m.swap(n, max_row);  // swap rows
            for row in 0..m.len() {  // swap cols
                m[row].swap(n, max_col);
            }
            /* */

            // println!("Pivoted for ({n}, {n}):");
            // MachineDesc::print_m(m);

            // Normalize row n
            let divisor = m[n][n];
            if divisor != Rational32::ZERO {
                m[n][n] = Rational32::ONE;
                for col in n+1..n_cols {
                    m[n][col] /= divisor;
                }
            }

            // Eliminate m[row][n] from all rows > n by subtracting row n
            // Plot twist: If the row being eliminated has a negative value in col n,
            // Negate that row before subtracting key row.
            for row in n+1..n_rows {
                if m[row][n] != Rational32::ZERO {
                    let factor = m[row][n];
                    for col in n..n_cols {
                        m[row][col] = m[row][col] - factor * m[n][col];
                    }  
                }
            }

            for row in n+1..n_rows {
                // If most sig remaining element is negative, negate the row
                let mut negate = false;
                for col in 0..n_cols-1 {
                    if m[row][col] > Rational32::ZERO {
                        negate = false;
                        break;
                    }
                    if m[row][col] < Rational32::ZERO {
                        negate = true;
                        break;
                    }
                }
                if negate {
                    for col in 0..n_cols {
                        m[row][col] = -m[row][col];
                    }  
                }
            }

            // println!("After elimination:");
            // MachineDesc::print_m(m);

        }

        println!("After Gaussian elimination:");
        MachineDesc::print_m(m);
    }

    // Find solution with back substitution
    fn bs(m: & Vec<Vec<Rational32>>) -> Vec<Rational32> {
        let n_rows = m.len();
        let n_cols = m[0].len() - 1;

        let mut soln = vec![Rational32::ZERO; n_cols];

        // count dependent variables
        let mut d_vars = 0;
        for n in 0..min(n_rows, n_cols) {
            if m[n][n] == Rational32::ONE {
                d_vars += 1;
            }
        }

        // number of independent variables
        let i_vars = n_cols - d_vars;

        println!("There are {n_cols} variables.  {i_vars} are free.");

        // Iterate over sum value of free variables
        let mut test_vectors = HashSet::<Vec<Rational32>>::new();
        test_vectors.insert(vec![Rational32::ZERO; i_vars]);

        let mut next_test_vectors = HashSet::<Vec<Rational32>>::new();


        let mut loops = 0;

        'outer:
        loop {
            // Iterate over test_vectors
            for v in &test_vectors {
                println!("Generating solution with test vector: {v:?}");

                // Generate the solution vector
                for n in 0..i_vars {
                    soln[d_vars+n] = v[n];
                }
                for n in 0..d_vars {
                    let r = d_vars - n - 1;  // row to solve with back substitution
                    let mut sum = Rational32::ZERO;
                    for c in r+1..n_cols {
                        sum += m[r][c] * soln[c];
                    }
                    soln[r] = m[r][n_cols] - sum;
                }

                // Test whether solution is valid (non-negative integers.)
                // If solution is valid, return it.
                let mut all_good = true;
                for component in &soln {
                    if (*component < Rational32::ZERO) || (*component.denom() != 1) {
                        all_good = false;
                    }
                }
                if all_good {
                    break 'outer;
                }
            }

            /*
            // So my debug cases don't loop infinitely when there are no free variables.
            if i_vars < 1 {
                break 'outer;
            }
            */

            // Generate next set of test vectors
            next_test_vectors.clear();
            // Use each member of test_vectors to generate some next test vectors
            for v in &test_vectors {
                for position in 0..i_vars {
                    let mut new_v = v.clone();
                    new_v[position] += 1;
                    next_test_vectors.insert(new_v);
                }
            }

            // Swap next_test_vectors into test_vectors before repeating loop.
            swap(&mut test_vectors, &mut next_test_vectors);
        }

        soln
    }

    // TODO: Fix problem.  This function gives different answers on each call.  The reason is that the HashMap of test_vectors used
    // in bs() call doesn't iterate in a determinate order.  And we need to find the minimal solution so we should be trying all of them
    // until we know we've seen a minimum.
    
    fn joltage_seq(&self) -> Vec<usize> {
        // Create augmented matrix for gaussian elimination
        let mut m = vec![vec![Rational32::ZERO; self.button_vecs.len()+1]; self.joltage.len()];

        // Set elements from button_vecs
        for (col, vec) in self.button_vecs.iter().enumerate() {
            for elt in vec.iter() {
                m[*elt][col] = Rational32::ONE;
            }
        }
        for (row, value) in self.joltage.iter().enumerate() {
            m[row][self.button_vecs.len()] = Rational32::from_integer(*value as i32);
        }

        // Print the matrix
        MachineDesc::print_m(&m);

        // Gaussian Elimination
        MachineDesc::ga(&mut m);
        let soln = MachineDesc::bs(&m);

        println!("Solution vector:");
        MachineDesc::print_v(&soln);

        println!("===============================");

        // Find solutions to the reduced problem
        // TODO
        let seq = soln.iter().map(|c| {
                *c.numer() as usize
            }).collect();

        seq
    }

}

// A representation of the puzzle inputs.
struct Input {
    machines: Vec<MachineDesc>,
}

impl Input {
    fn read(text: &str) -> Input 
    {
        let mut machines = Vec::new();

        for line in text.lines() {
            // Process one line
            machines.push(MachineDesc::from_line(line));
        }

        Input { machines }
    }
}

pub struct Day10 {
}

impl Day10 {
    pub const fn new() -> Self {
        Self { }
    }
}

impl Day for Day10 {

    // Compute Part 1 solution
    fn part1(&self, text: &str) -> Answer {
        // Read input file into Input struct
        let input = Input::read(text);

        let sum = input.machines.iter()
            .map(|m| {
                m.act_seq().len()
            })
            .sum();

        Answer::Numeric(sum)
    }

    fn part2(&self, text: &str) -> Answer {

        // Read input file into Input struct
        let input = Input::read(text);

        let sum = input.machines.iter()
            .map(|m| {
                let seq = m.joltage_seq();
                let s: usize = seq.iter().sum();
                s
            })
            .sum();

        Answer::Numeric(sum)
    }
}

#[cfg(test)]
mod test {
    use crate::day10::{Day10, Input};
    use crate::day::{Day, Answer};
    use data_aoc2025::DAY10_INPUT;
    
    const EXAMPLE1: &str = "\
[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}
";

    #[test]
    // Read part 1 example and confirm inputs
    fn test_read_part1() {
        let input = Input::read(EXAMPLE1);
        assert_eq!(input.machines.len(), 3);

        assert_eq!(input.machines[0].start_indicator, 0b0110);
        assert_eq!(input.machines[1].start_indicator, 0b01000);
        assert_eq!(input.machines[2].start_indicator, 0b101110);

        assert_eq!(input.machines[0].button_masks.len(), 6);
        assert_eq!(input.machines[0].button_masks[0], 0b1000);
        assert_eq!(input.machines[0].button_masks[1], 0b1010);
        assert_eq!(input.machines[0].button_masks[2], 0b0100);
        assert_eq!(input.machines[0].button_masks[3], 0b1100);
        assert_eq!(input.machines[0].button_masks[4], 0b0101);
        assert_eq!(input.machines[0].button_masks[5], 0b0011);

        assert_eq!(input.machines[0].joltage.len(), 4);
        assert_eq!(input.machines[0].joltage[0], 3);
        assert_eq!(input.machines[0].joltage[1], 5);
        assert_eq!(input.machines[0].joltage[2], 4);
        assert_eq!(input.machines[0].joltage[3], 7);
    }

    #[test]
    fn test_activate() {
        let input = Input::read(EXAMPLE1);

        let activation_seq = input.machines[0].act_seq();
        assert_eq!(activation_seq.len(), 2);

        let activation_seq = input.machines[1].act_seq();
        assert_eq!(activation_seq.len(), 3);

        let activation_seq = input.machines[2].act_seq();
        assert_eq!(activation_seq.len(), 2);   
    }

    #[test]
    fn test_set_joltage() {
        let input = Input::read(EXAMPLE1);

        let joltage_seq = input.machines[0].joltage_seq();
        assert_eq!(joltage_seq.iter().sum::<usize>(), 10); // 10

        let joltage_seq = input.machines[1].joltage_seq();
        assert_eq!(joltage_seq.iter().sum::<usize>(), 12); // 12

        let joltage_seq = input.machines[2].joltage_seq();
        assert_eq!(joltage_seq.iter().sum::<usize>(), 11); // 11     
    }

    #[test]
    fn test_real_set_joltage() {
        let input = Input::read(DAY10_INPUT);
        for n in 0..input.machines.len() {
            let joltage_seq = input.machines[n].joltage_seq();
            // assert_eq!(joltage_seq.iter().sum::<usize>(), 0);
        }
    }

    #[test]
    // Compute part 1 result on example 1 and confirm expected value.
    fn test_part1() {
        // Based on the example in part 1.
        let d = Day10::new();
        assert_eq!(d.part1(EXAMPLE1), Answer::Numeric(7));
    }

    #[test]
    // Compute part 2 result on example 2 and confirm expected value.
    fn test_part2() {
        // Based on the example in part 2.
        let d = Day10::new();
        assert_eq!(d.part2(EXAMPLE1), Answer::Numeric(33));
    }
    
}
