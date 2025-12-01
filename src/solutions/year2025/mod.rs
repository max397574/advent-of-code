use std::fmt::Display;

pub mod day1;
pub mod day10;
pub mod day11;
pub mod day12;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day7;
pub mod day8;
pub mod day9;

type YearSolutions = [[Box<dyn Fn(&str) -> String>; 2]; 12];

pub fn get_solutions_per_year() -> YearSolutions {
    fn get_string<D: Display + 'static>(function: fn(&str) -> D) -> Box<dyn Fn(&str) -> String> {
        Box::new(move |input| function(input).to_string())
    }
    [
        [get_string(day1::part1), get_string(day1::part2)],
        [get_string(day2::part1), get_string(day2::part2)],
        [get_string(day3::part1), get_string(day3::part2)],
        [get_string(day4::part1), get_string(day4::part2)],
        [get_string(day5::part1), get_string(day5::part2)],
        [get_string(day6::part1), get_string(day6::part2)],
        [get_string(day7::part1), get_string(day7::part2)],
        [get_string(day8::part1), get_string(day8::part2)],
        [get_string(day9::part1), get_string(day9::part2)],
        [get_string(day10::part1), get_string(day10::part2)],
        [get_string(day11::part1), get_string(day11::part2)],
        [get_string(day12::part1), get_string(day12::part2)],
    ]
}
