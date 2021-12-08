use std::io;
use std::fmt;
use std::collections::HashMap;


pub fn run(input: String) {
    day7_1(&input);
    day7_2(&input);
}

fn day7_1(input: &String) {
    let mut nums: Vec<i64> = input.trim().split(',').map(|s| s.parse::<i64>().unwrap()). collect();

    let max = *nums.iter().max().unwrap();
    println!("{}", max);

    let mut best = (0, 2000000);
    let mut sum_abs = 0;
    for pos in 0..max + 1 {
        for i in nums.iter() {
            sum_abs += (pos - i).abs();
            // println!("{}", sum_abs);
        }
        println!("Sum of abs for position {}: {}", pos, sum_abs);
        if sum_abs < best.1 {
            best = (pos, sum_abs)
        }
        sum_abs = 0;
    }

    println!("{:?}", best);
}

fn day7_2(input: &String) {
    let mut nums: Vec<i64> = input.trim().split(',').map(|s| s.parse::<i64>().unwrap()). collect();

    let max = *nums.iter().max().unwrap();
    println!("{}", max);

    let mut best = (0, 997233916);
    let mut fuel_sum = 0;
    for pos in 0..max + 1 {
        for i in nums.iter() {
            let diff = (pos - i).abs();
            // println!("{}", sum_abs);
            let sum = (diff * (diff + 1)) / 2;
            fuel_sum += sum;
        }
        println!("Fuel sum for position {}: {}", pos, fuel_sum);
        if fuel_sum < best.1 {
            best = (pos, fuel_sum)
        }
        fuel_sum = 0;
    }

    println!("{:?}", best);
}