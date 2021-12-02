mod day1;

use std::io;
use std::fs;

fn main() {
    day1::run(read_input("day1/input.txt"));
}

fn read_input(filename: &str) -> String {
    fs::read_to_string(filename).unwrap()
}