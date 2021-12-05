mod day1;
mod day2;
mod day3;
mod day4;

//use std::io;
use std::fs;

fn main() {
    day4::run(read_input("day4/input.txt"));
}

fn read_input(filename: &str) -> String {
    fs::read_to_string(filename).unwrap()
}