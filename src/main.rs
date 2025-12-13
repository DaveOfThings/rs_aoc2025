// #![feature(random)]

mod day;
mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod day10;
mod day11;
mod day12;

use day::{Day, Answer};
use day1::Day1;
use day2::Day2;
use day3::Day3;
use day4::Day4;
use day5::Day5;
use day6::Day6;
use day7::Day7;
use day8::Day8;
use day9::Day9;
use day10::Day10;
use day11::Day11;
use day12::Day12;

use data_aoc2025::DAY1_INPUT;
use data_aoc2025::DAY2_INPUT;
use data_aoc2025::DAY3_INPUT;
use data_aoc2025::DAY4_INPUT;
use data_aoc2025::DAY5_INPUT;
use data_aoc2025::DAY6_INPUT;
use data_aoc2025::DAY7_INPUT;
use data_aoc2025::DAY8_INPUT;
use data_aoc2025::DAY9_INPUT;
use data_aoc2025::DAY10_INPUT;
use data_aoc2025::DAY11_INPUT;
use data_aoc2025::DAY12_INPUT;

static DAYS: [(&dyn Day, &str); 12] = [
    (&Day1::new(), DAY1_INPUT),  // Dec 1
    (&Day2::new(), DAY2_INPUT),
    (&Day3::new(), DAY3_INPUT),
    (&Day4::new(), DAY4_INPUT),
    (&Day5::new(), DAY5_INPUT),
    (&Day6::new(), DAY6_INPUT),
    (&Day7::new(), DAY7_INPUT),
    (&Day8::new(), DAY8_INPUT),
    (&Day9::new(), DAY9_INPUT),
    (&Day10::new(), DAY10_INPUT),
    (&Day11::new(), DAY11_INPUT),
    (&Day12::new(), DAY12_INPUT),
];

fn report_day(day_no: usize) {
    
    let (day, text) = DAYS[day_no-1];

    let ans1 = day.part1(text);
    let msg1 = match ans1 {
        Answer::None => String::from("        -"),
        Answer::Numeric(n) => format!("{n}"),
        Answer::String(s) => format!("{s}"),
    };

    let ans2 = day.part2(text);
    let msg2 = match ans2 {
        Answer::None => String::from("        -"),
        Answer::Numeric(n) => format!("{n}"),
        Answer::String(s) => format!("{s}"),
    };
    println!("Day {day_no:2}: {msg1:>18} {msg2:>18}");
}


fn main() {
    println!("Advent of Code 2025!\n");

    let target_day = 0;

    match target_day {
        0 => {
            // report all days
            println!("{:7} {:>18} {:>18}", "", "Part 1", "Part 2");
            for day_no in 1..=12 {
                report_day(day_no);
            }
        }
        1..=25 => {
            // report a specific day
            report_day(target_day);
        }
        _ => {
            // invalid day 
            println!("Day {target_day} is invalid.\n");
        }
    }
    println!();

}



#[cfg(test)]
mod test {
    use crate::day::Answer;
    use crate::DAYS;
    use lazy_static::lazy_static;
    
    lazy_static! {

        static ref ANSWERS: [(Answer, Answer); 12] = [
            (Answer::Numeric(1105), Answer::Numeric(6599)),                 // Dec 1
            (Answer::Numeric(21139440284), Answer::Numeric(38731915928)),   // Dec 2
            (Answer::Numeric(17193), Answer::Numeric(171297349921310)),     // Dec 3
            (Answer::Numeric(1424), Answer::Numeric(8727)),                 // Dec 4
            (Answer::Numeric(737), Answer::Numeric(357485433193284)),       // Dec 5
            (Answer::Numeric(5733696195703), Answer::Numeric(10951882745757)),   // Dec 6
            (Answer::None, Answer::None),   // Dec 7
            (Answer::None, Answer::None),   // Dec 8
            (Answer::None, Answer::None),   // Dec 9
            (Answer::None, Answer::None),   // Dec 10
            (Answer::None, Answer::None),   // Dec 11
            (Answer::None, Answer::None),   // Dec 12
        ];
    }

    #[test]
    fn test_all() {
        for day in 1..12 {
            let (d, text) = DAYS[day-1];
            assert_eq!(d.part1(text), ANSWERS[day-1].0);
            assert_eq!(d.part2(text), ANSWERS[day-1].1);
        }
    }

    #[test]
    fn test_day() {
        let day = 6;
        let (d, text) = DAYS[day-1];

        assert_eq!(d.part1(text), ANSWERS[day-1].0);
        assert_eq!(d.part2(text), ANSWERS[day-1].1);
    }

}
