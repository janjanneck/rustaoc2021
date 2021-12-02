use std::io;

pub fn run(input: String) {
    day2_1(&input);
    day2_2(&input);
}

fn day2_1(input: &String) {

    let inputs: Vec<&str> = input.split("\n").filter(|s| s != &"").collect();

    let commands: Vec<(&str, i32)> = inputs.iter()
    .map(|s| s.split_whitespace().collect())
    .map(|s: Vec<&str>| (s[0], s[1].parse::<i32>().unwrap()))
    .collect();

    let mut horizontal = 0;
    let mut depth = 0;

    for command in commands {
        let direction = command.0;
        let amount = command.1;

        match direction {
            "forward" => horizontal += amount,
            "down" => depth += amount,
            "up" => depth -= amount,
            _ => println!("something else")
        }
    }

    println!("Horizontal: {}, Depth: {}, Product: {}", horizontal, depth, horizontal * depth);
}

fn day2_2(input: &String) {

    let inputs: Vec<&str> = input.split("\n").filter(|s| s != &"").collect();

    let commands: Vec<(&str, i32)> = inputs.iter()
    .map(|s| s.split_whitespace().collect())
    .map(|s: Vec<&str>| (s[0], s[1].parse::<i32>().unwrap()))
    .collect();

    let mut horizontal = 0;
    let mut depth = 0;
    let mut aim = 0;

    for command in commands {
        let direction = command.0;
        let amount = command.1;

        match direction {
            "forward" => {
                horizontal += amount;
                depth += aim * amount;
            },
            "down" => aim += amount,
            "up" => aim -= amount,
            _ => println!("something else")
        }
    }

    println!("Horizontal: {}, Depth: {}, Product: {}", horizontal, depth, horizontal * depth);
}
