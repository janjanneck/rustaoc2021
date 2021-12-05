mod day1;
mod day2;
mod day3;
mod day4;
mod day5;

//use std::io;
use std::fs;

fn main() {
    day5::run(read_input("day5/input.txt"));
}

fn read_input(filename: &str) -> String {
    fs::read_to_string(filename).unwrap()
}