use std::collections::{HashMap, HashSet, VecDeque};
use std::cmp::min;
use std::mem::swap;
use num_rational::Rational32;
use regex::Regex;
use num_traits::Signed;

use crate::day::{Day, Answer};

// Represents one of the machines described in the Advent of Code Day 10, 2025 puzzle.
struct MachineDesc {
    start_indicator: usize,
    button_masks: Vec<usize>,
    button_vecs: Vec<Vec<usize>>,
    joltage: Vec<usize>,
}

// Implementation of the machine
impl MachineDesc {

    // Regular expressions used to parse parts of the input lines
    const ACTIVATION_RE: &str = "\\[([\\.\\#]+)\\]";  // Matches [..##.], cap[1]: ..##.
    const BUTTON_RE: &str = "\\(([0-9](,[0-9])*)\\)";  // Matches (1,2,3), cap[1]: 1,2,3
    const JOLTAGE_RE: &str = "\\{([0-9]+(,[0-9]+)*)\\}"; // Matches {10,8,2}, cap[1]: 10,8,2

    // Take a line from input file, construct the corresponding machine.
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

    // Print a matrix (for debugging.)
    #[expect(unused)]
    fn print_m(m: &Vec<Vec<Rational32>>) {
        for row in m.iter() {
            for cell in row.iter() {
                print!("{} ", (*cell.numer() as f32)/(*cell.denom() as f32));
            }
            println!("");
        }
    }

    // Print a vector (for debugging)
    #[expect(unused)]
    fn print_v(m: &Vec<Rational32>) {
        for cell in m.iter() {
            print!("{} ", (*cell.numer() as f32)/(*cell.denom() as f32));
        }
        println!("");
    }

    // Gaussian Elimination step used to solve the joltage sequence.
    // Since we ultimately want non-negative integer solutions, this routine uses Rational32
    // to represent the components.
    fn gauss_elim(m: &mut Vec<Vec<Rational32>>) {

        let n_rows = m.len();
        let n_cols = m[0].len();

        // diagonal length is minimum of rows or columns (not counting augmented column)
        let diag_len = min(n_rows, n_cols-1);

        for n in 0..diag_len {
            // Pivot rows and columns to get largest remaining value in m[n][n]
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
            for row in 0..n_rows {
                m[row].swap(n, max_col);  // swap cols
                // col_seq.swap(n, max_col);    // keep track of col swaps
            }

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

            // Subtract some multiple of row n from each row under row n.
            // This produces the zero in column n for those lower rows.
            for row in n+1..n_rows {
                if m[row][n] != Rational32::ZERO {
                    let factor = m[row][n];
                    for col in n..n_cols {
                        m[row][col] = m[row][col] - factor * m[n][col];
                    }  
                }
            }

            // println!("After elimination:");
            // MachineDesc::print_m(m);

        }

        // println!("After Gaussian elimination:");
        // MachineDesc::print_m(m);
    }

    // Back Substitution with a search over the solution space for the optimal solution.
    //
    // After Gaussian Elimination, this performs back substitution to find the solution.
    // In many cases, there are unconstrained variables.  And in that case, the code will 
    // test all combinations of those free variables with non-negative values to find the
    // set that produces a minimal value in the vector sum.
    // 
    // When looping over the possible values of the unconstrained part, it starts with all 
    // zeros and goes up through combinations totalling 1, then 2, etc.
    // The solutions produced at each stage may be invalid if they have negative or 
    // non-integer components.  The loop will continue until it finds the first valid one, then
    // it computes how much further (worst case) it must search to exhaust all valid combinations.
    // It loops through all those then returns the best one found.

    fn back_sub(m: & Vec<Vec<Rational32>>) -> Vec<Rational32> {
        let n_rows = m.len();
        let n_cols = m[0].len();

        let mut soln = vec![Rational32::ZERO; n_cols-1];
        let mut min_soln = vec![Rational32::ZERO; n_cols-1];
        let mut min_soln_sum: Option<usize> = None;

        // count dependent variables
        let mut d_vars = 0;
        for n in 0..min(n_rows, n_cols-1) {
            if m[n][n] == Rational32::ONE {
                d_vars += 1;
            }
        }

        // number of independent variables
        let i_vars = n_cols-1 - d_vars;

        // println!("There are {d_vars} dependent variables.  {i_vars} are free.");

        // All the sets of unconstrained values to test on one iteration through loop.
        let mut test_vectors = HashSet::<Vec<Rational32>>::new();
        test_vectors.insert(vec![Rational32::ZERO; i_vars]);

        // All the sets of unconstrained values to test on the next iteration through the loop.
        let mut next_test_vectors = HashSet::<Vec<Rational32>>::new();

        // Max number of loop iterations necessary.  (This is unknown until the first solution pops out.)
        let mut max_loops = None;

        // how many times the loop has iterated.
        let mut loops = 0;

        // Iterate over all possible values of unconstrained variables.
        loop {
            // Iterate over contents of test_vectors (all combos of unconstrained values with a particular sum, 0, 1, 2...)
            for v in &test_vectors {

                // Generate the solution vector
                // This works by stuffing the unconstrained values for this round then using back substitution
                // to find all the other components of the solution.
                for n in 0..i_vars {
                    soln[d_vars+n] = v[n];
                }
                for n in 0..d_vars {
                    let r = d_vars - n - 1;  // row to solve with back substitution
                    let mut sum = Rational32::ZERO;
                    for c in r+1..n_cols-1 {
                        sum += m[r][c] * soln[c];
                    }
                    soln[r] = m[r][n_cols-1] - sum;
                }

                // Test whether solution is valid (non-negative integers.)
                // If solution is valid, return it.
                let all_good = soln.iter()
                    .map(|c| { 
                        (*c >= Rational32::ZERO) && c.is_integer()
                    })
                    .fold(true, |acc, good| { acc && good });

                // Test whether good solutions are better than previous ones.
                if all_good {
                    // This is a good candidate.  Check it's sum and see if it's less that what we had
                    let sum = soln.iter().map(|c| {*c.numer() as usize}).sum();
                    match min_soln_sum {
                        Some(old_min) => {
                            // We have a previous min, check to see if this one is better.
                            if sum < old_min {
                                min_soln_sum = Some(sum);
                                min_soln = soln.clone();
                            }
                        }
                        None => {
                            // This is the first solution seen, register it as the min so far.
                            min_soln_sum = Some(sum);
                            min_soln = soln.clone();
                        }
                    }
                }
            }

            // Break out of outer loop if there are no free variables -- we only have one possible solution
            if i_vars < 1 {
                break;
            }

            // That's one more iteration complete.  If that's all we need, break out.
            loops += 1;
            if let Some(max) = max_loops {
                if loops > max {
                    break;
                }
            }

            // If we haven't determined max_loops yet but we found a solution on this iteration,
            // We can determin max_loops now.
            if max_loops.is_none() && min_soln_sum.is_some() {
                // The max number of loops should be loops + the largest component of the initial solution.
                let max = min_soln.iter().map(|c| { c.numer().abs() as usize }).max().unwrap();
                max_loops = Some(loops+max);
            }

            // Generate next set of test vectors.
            // We take the last set of test vectors, add one to each potential position, then put
            // the result into the HashSet.  Duplicates are generated and discarded in the process.
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

        min_soln
    }

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
        // MachineDesc::print_m(&m);

        // Gaussian Elimination followed by back substitution
        MachineDesc::gauss_elim(&mut m);
        let soln = MachineDesc::back_sub(&m);

        // Convert the solution from Vec<Rational32> to Vec<usize>
        let seq = soln.iter().map(|c| {
                *c.numer() as usize
            }).collect();

        // Return the optimal sequence that was found
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

    // Compute part 2 solution
    fn part2(&self, text: &str) -> Answer {

        // Read input file into Input struct
        let input = Input::read(text);

        // compute the optimal joltage sequence for each machine and sum all the components
        let sum = input.machines.iter()
            .map(|m| {
                m.joltage_seq().iter().sum::<usize>()
            })
            .sum();

        Answer::Numeric(sum)
    }
}

#[cfg(test)]
mod test {
    use crate::day10::{Day10, Input};
    use crate::day::{Day, Answer};
    
    // Examples from the challenge
    const EXAMPLE1: &str = "\
[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}
";

    // One actual input I had to debug
    const EXAMPLE2: &str = "\
[#.#..##] (1,3,6) (4,5) (1,2) (1,2,4,5) (0,1,4,5,6) (0,1) (3) (1,2,4) (1,5) {21,73,20,28,28,36,19}
";

    // A second input I used for debugging.
    const EXAMPLE3: &str = "\
[..#####.##] (1,3,4,6,7,8,9) (4) (0,2) (0,2,3,4,5,7,8,9) (3,6,7,9) (1,5,8) (0,1,2,3,4,6,8) (1,7,8) (0,1,2,3,4,8,9) (2,7,9) (0,1,2,5,6,8) (2,4,7,8) (6,8,9) {73,75,92,46,75,30,45,37,109,39}
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
    // Test with part 1 examples
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
    // Test with part 2 examples.
    fn test_set_joltage() {
        let input = Input::read(EXAMPLE1);

        let joltage_seq = input.machines[0].joltage_seq();
        assert_eq!(joltage_seq.iter().sum::<usize>(), 10);

        let joltage_seq = input.machines[1].joltage_seq();
        assert_eq!(joltage_seq.iter().sum::<usize>(), 12);

        let joltage_seq = input.machines[2].joltage_seq();
        assert_eq!(joltage_seq.iter().sum::<usize>(), 11); 
    }

    #[test]
    // Test case with one of my actual inputs
    fn test2_set_joltage() {
        let input = Input::read(EXAMPLE2);

        let joltage_seq = input.machines[0].joltage_seq();
        assert_eq!(joltage_seq.iter().sum::<usize>(), 90);
    }

    #[test]
    // Test part 2 with a second actual input
    fn test3_set_joltage() {
        let input = Input::read(EXAMPLE3);

        let joltage_seq = input.machines[0].joltage_seq();
        assert_eq!(joltage_seq.iter().sum::<usize>(), 128);
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
