mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;

//use std::io;
use std::fs;

fn main() {
    day7::run(read_input("day7/input.txt"));
}

fn read_input(filename: &str) -> String {
    fs::read_to_string(filename).unwrap()
}