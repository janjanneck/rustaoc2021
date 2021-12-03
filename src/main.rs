mod day1;
mod day2;
mod day3;

//use std::io;
use std::fs;

fn main() {
    day3::run(read_input("day3/input.txt"));
}

fn read_input(filename: &str) -> String {
    fs::read_to_string(filename).unwrap()
}