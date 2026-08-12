use std::{collections::{HashMap, VecDeque}};

use regex::Regex;

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

    fn compare_rows(row1: &Vec<isize>, row2: &Vec<isize>) -> isize {
        assert_eq!(row1.len(), row2.len());

        for col in 0..row1.len() {
            if row1[col] > row2[col] { return 1; }   // row1 is greater
            if row1[col] < row2[col] { return -1; }  // row2 is greater
        }

        // Everything is equal
        return 0;
    }

    fn print_m(m: &Vec<Vec<isize>>) {
        for row in m.iter() {
            println!("{row:?}");
        }
    }

    fn ga(m: &mut Vec<Vec<isize>>) {
        let mut key_row = 0;

        while key_row < m.len()-1 {
            // sort rows from key row down
            for i in 0..m.len()-1 {
                for j in i+1..m.len() {
                    if MachineDesc::compare_rows(&m[i], &m[j]) < 0 {
                        // swap these rows
                        m.swap(i, j);
                    }
                }
            }

            println!("Sorted:");
            MachineDesc::print_m(m);

            // reduce rows below key row by subtracting key_row
            for row in key_row+1..m.len() {
                if m[row][key_row] == 1 {
                    // subtract key row from this
                    for col in key_row..m[0].len() {
                        m[row][col] -= m[key_row][col];
                    }
                    /*
                    if m[row][key_row+1] < 0 {
                        for col in key_row..m[0].len() {
                            m[row][col] *= -1;
                        }  
                    }
                    */
                }
            }

            // (if result is negative in most sig place, negate the row)
            for row in key_row+1..m.len() {
                let mut sign = None;
                for col in 0..m[0].len() {
                    if sign.is_none() && m[row][col] > 0 {
                        // sign is positive
                        sign = Some(1);
                    }
                    if sign.is_none() && m[row][col] < 0 {
                        // negative sign
                        sign = Some(-1);
                    }
                }

                if sign == Some(-1) {
                    for col in 0..m[0].len() {
                        m[row][col] *= -1;
                    }
                }
            }

            println!("Reduced:");
            MachineDesc::print_m(m);

            key_row += 1;
        }
        println!("===============================");
    }

    fn joltage_seq(&self) -> Vec<usize> {
        // Create augmented matrix for gaussian elimination
        let mut m = vec![vec![0_isize; self.button_vecs.len()+1]; self.joltage.len()];

        // Set elements from button_vecs
        for (col, vec) in self.button_vecs.iter().enumerate() {
            for elt in vec.iter() {
                m[*elt][col] = 1;
            }
        }
        for (row, value) in self.joltage.iter().enumerate() {
            m[row][self.button_vecs.len()] = *value as isize;
        }

        // Print the matrix
        MachineDesc::print_m(&m);

        MachineDesc::ga(&mut m);

        // Find solutions to the reduced problem
        // TODO
        let mut _seq = vec![0; self.joltage.len()];
        _seq
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
        let _input = Input::read(text);

        Answer::Numeric(0)
    }
}

#[cfg(test)]
mod test {
    use crate::day10::{Day10, Input};
    use crate::day::{Day, Answer};
    
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
        assert_eq!(joltage_seq.iter().sum::<usize>(), 0); // 10

        let joltage_seq = input.machines[1].joltage_seq();
        assert_eq!(joltage_seq.iter().sum::<usize>(), 0); // 12

        let joltage_seq = input.machines[2].joltage_seq();
        assert_eq!(joltage_seq.iter().sum::<usize>(), 0); // 11     
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
