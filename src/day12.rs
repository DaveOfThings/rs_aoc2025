use regex::Regex;

use crate::day::{Day, Answer};

#[derive(Clone)]
struct Piece {
    id: usize,
    coverage: Vec<(usize, usize)>,  // vector of covered spaces (row, col)
}

impl Piece {
    pub fn default() -> Piece {
        let coverage = Vec::new();
        Piece { id: 0, coverage }
    }

    pub fn set_id(&mut self, id: usize) {
        self.id = id;
    }

    pub fn add_row(&mut self, row_no: usize, row: &str) {
        row.char_indices().for_each(|(col, c)| {
            if c == '#' {
                self.coverage.push((row_no, col));
            }
        })      
    }

    pub fn clear(&mut self) {
        self.coverage.clear();
    }
}

struct Board {
    width: usize,
    height: usize,
    counts: Vec<usize>,
}

// A representation of the puzzle inputs.
struct Input {
    pieces: Vec<Piece>,
    boards: Vec<Board>,
}

impl Input {
    fn read(text: &str) -> Input 
    {
        // matches the line that starts a piece def.  e.g. "3:".
        // match[1] contains the digit.
        let piece_start_re = Regex::new("^([0-9]):").unwrap();  

        // matches the line that is part of a piece definition. e.g. "##."
        // match[1] contains the string "##." in this case.
        let piece_row_re = Regex::new("([#\\.]+)").unwrap();

        // matches the line that is a board definition. e.g. "37x39: 41 36 41 38 28 38"
        // match[1] contains "37"
        // match[2] contains "39"
        // match[3] contains "41 36 41 38 28 38"
        let board_def_re = Regex::new("([0-9]+)x([0-9]+): (.*)").unwrap();

        let mut pieces = Vec::new();
        let mut boards = Vec::new();
        let mut piece_in_progress = Piece::default() ;
        let mut next_row = 0;

        for line in text.lines() {
            // Process one line
            if let Some(matches) = piece_start_re.captures(line) {
                // Start of a new piece
                piece_in_progress.set_id(matches[1].parse::<usize>().unwrap());
            }
            else if let Some(matches) = piece_row_re.captures(line) {
                // A new row of a piece
                piece_in_progress.add_row(next_row, &matches[1]);
                next_row += 1;
            }
            else if line.is_empty() {
                // A blank link marks the end of each piece definition.
                pieces.push(piece_in_progress.clone());
                piece_in_progress.clear();
                next_row = 0;
            }
            else if let Some(matches) = board_def_re.captures(line) {
                // A board definition
                let counts = matches[3].split(" ").map(|s| {s.parse::<usize>().unwrap()}).collect();
                let width = matches[1].parse::<usize>().unwrap();
                let height = matches[2].parse::<usize>().unwrap();
                let board = Board { width, height, counts };
                boards.push(board);
            }

        }

        Input { pieces, boards }
    }

    fn area_fits(&self, board_id: usize) -> bool {
        let board = &self.boards[board_id];
        let board_area = board.height * board.width;

        let pieces_area: usize = board.counts.iter()
            .enumerate()
            .map(|(index, count)| {
                self.pieces[index].coverage.len() * count
            })
            .sum();

        /*
        println!("Pieces area: {pieces_area}, Board area: {board_area}");
        if board_area > pieces_area {
            let pct_free = 100.0*((board_area - pieces_area) as f64)/(board_area as f64);
            println!("  {}% free", pct_free);
            assert!(pct_free > 26.0);
        }
        */

        pieces_area < board_area
    }

    fn trivial_fit(&self, board_id: usize) -> bool {
        let board = &self.boards[board_id];
        let pieces_h = board.height / 3;
        let pieces_w = board.width / 3;
        let easy_fit_pieces = pieces_h * pieces_w;

        easy_fit_pieces >= board.counts.iter().sum()
    }
}

pub struct Day12 {
}

impl Day12 {
    pub const fn new() -> Self {
        Self { }
    }
}

impl Day for Day12 {

    // Compute Part 1 solution
    fn part1(&self, text: &str) -> Answer {
        // Read input file into Input struct
        let _input = Input::read(text);

        // This relies on the assumption that all solutions are trivial solutions.

        // TODO : Make trivial_fit a method on board, not input, then use iter()
        let mut trivial_fits = 0;
        for n in 0.._input.boards.len() {
            if _input.trivial_fit(n) {
                trivial_fits += 1;
            }
        }

        Answer::Numeric(trivial_fits)
    }

    fn part2(&self, text: &str) -> Answer {

        // Read input file into Input struct
        let _input = Input::read(text);

        Answer::None
    }
}

#[cfg(test)]
mod test {
    use crate::day12::{Day12, Input};
    use crate::day::{Day, Answer};
    use data_aoc2025::DAY12_INPUT;
    
    const EXAMPLE1: &str = "\
0:
###
##.
##.

1:
###
##.
.##

2:
.##
###
##.

3:
##.
###
##.

4:
###
#..
###

5:
###
.#.
###

4x4: 0 0 0 0 2 0
12x5: 1 0 1 0 2 2
12x5: 1 0 1 0 3 2
";

    const EXAMPLE2: &str = "\
0:
#..
##.
.##

1:
###
.#.
###

2:
###
###
..#

3:
###
.##
##.

4:
###
##.
#..

5:
#.#
#.#
###

37x39: 41 36 41 38 28 38
43x43: 42 58 49 46 45 43
45x42: 42 52 44 61 39 50
42x45: 38 30 36 43 26 37
";

    #[test]
    // Read part 1 example and confirm inputs
    fn test_read_part1() {
        let input = Input::read(EXAMPLE1);

        assert_eq!(input.pieces.len(), 6);
        assert_eq!(input.pieces[0].coverage, vec![(0, 0), (0, 1), (0, 2), (1, 0), (1, 1), (2, 0), (2, 1)]);
        assert_eq!(input.pieces[1].coverage, vec![(0, 0), (0, 1), (0, 2), (1, 0), (1, 1), (2, 1), (2, 2)]);
        assert_eq!(input.pieces[1].id, 1);
        assert_eq!(input.boards.len(), 3);
        assert_eq!(input.boards[1].height, 5);
        assert_eq!(input.boards[1].width, 12);
        assert_eq!(input.boards[1].counts, vec![1, 0, 1, 0, 2, 2]);
    }

    #[test]
    fn test_area_fits() {
        let input = Input::read(EXAMPLE1);
        let input2 = Input::read(EXAMPLE2);

        assert_eq!(input.area_fits(0), true);
        assert_eq!(input.area_fits(1), true);
        assert_eq!(input.area_fits(2), true);

        assert_eq!(input2.area_fits(0), false);
        assert_eq!(input2.area_fits(1), false);
        assert_eq!(input2.area_fits(2), false);
        assert_eq!(input2.area_fits(3), true);
    }

    #[test]
    fn test_area_fits_real() {
        let input = Input::read(DAY12_INPUT);

        let misfits = input.boards.iter().enumerate()
            .filter(|(n, _board)| {
                !input.area_fits(*n)
            })
            .count();

        println!("{misfits} out of {} can't fit.", input.boards.len());

        assert_eq!(misfits, 0);
    }

      #[test]
    fn test_trivial_fits_real() {
        let input = Input::read(DAY12_INPUT);

        let trivial = input.boards.iter().enumerate()
            .filter(|(n, _board)| {
                input.trivial_fit(*n)
            })
            .count();

        println!("{trivial} out of {} fit trivially.", input.boards.len());

        assert_eq!(trivial, 0);
    }

    #[test]
    // Compute part 1 result on example 1 and confirm expected value.
    fn test_part1() {
        // Based on the example in part 1.
        let d = Day12::new();
        assert_eq!(d.part1(DAY12_INPUT), Answer::Numeric(577));
    }

    #[test]
    // Compute part 2 result on example 2 and confirm expected value.
    fn test_part2() {
        // Based on the example in part 2.
        let d = Day12::new();
        assert_eq!(d.part2(EXAMPLE1), Answer::None);
    }
    
}
