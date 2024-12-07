mod day1;
mod day2;
mod day3;
mod day4;
mod day5;

use std::{env::args, fs::read_to_string};

fn main() {
    let args: Vec<_> = args().collect();
    let day: i32 = args[1].parse::<i32>().expect("Usage: cargo run <day>");

    let input = read_to_string(format!("inputs/input{day}.txt"))
        .expect("Input file for this day does not exist!");

    let solution = match day {
        1 => day1::solve(input),
        2 => day2::solve(input),
        3 => day3::solve(input),
        4 => day4::solve(input),
        5 => day5::solve(input),
        _ => "Day not implemented yet!".to_string(),
    };

    println!("{solution}")
}
