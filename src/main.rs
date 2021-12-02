mod day1;
mod day2;

//use std::io;
use std::fs;

fn main() {
    day2::run(read_input("day2/input.txt"));
}

fn read_input(filename: &str) -> String {
    fs::read_to_string(filename).unwrap()
}