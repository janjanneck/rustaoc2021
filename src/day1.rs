use std::io;

pub fn run(input: String) {
    day1_1(&input);
    day1_2(&input);
}

fn day1_1(input: &String) -> io::Result<()> {

    let num_strings = input.split("\n");

    let numbers : Vec<i32> = num_strings
    .filter(|s| s != &"")
    .map(|s| s.parse().expect("parse error"))
    .collect();

    let mut increases = 0;
    let mut previous = numbers[0];

    for num in numbers {
        if num > previous {
            increases += 1;
        } 
        previous = num;
    }

    println!("Number of increases: {}", increases);

    Ok(())
}

fn day1_2(input: &String) -> io::Result<()>{
    let num_strings = input.split("\n");

    let numbers : Vec<i32> = num_strings
    .filter(|s| s != &"")
    .map(|s| s.parse().expect("parse error"))
    .collect();

    let mut increases = 0;
    let mut previous = -1;

    for i in 0..numbers.len() -2 {
        let sum = numbers[i] + numbers[i+1] + numbers[i+2];
        if sum > previous && previous != -1 {
            increases += 1;
        }
        previous = sum;
    }

    println!("Number of increases: {}", increases);

    Ok(())
}
