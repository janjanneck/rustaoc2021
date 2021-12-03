use std::io;

pub fn run(input: String) {
    day3_1(&input);
    day3_2(&input);
}

fn day3_1(input: &String) {

    let inputs: Vec<&str> = input.split("\n").filter(|s| s != &"").collect();


    let mut epsilon_rate = Vec::new();
    for column_idx in 0..inputs[0].len() {
        let mut ones = 0;
        let mut zeros = 0;
        for line_idx in 0..inputs.len() {
            let character = inputs[line_idx].as_bytes()[column_idx] as char;
            match character {
                '1' => ones += 1,
                '0' => zeros += 1,
                _ => panic!()
            }
        }
        if ones > zeros {
            epsilon_rate.push(1);
        } else if ones < zeros {
            epsilon_rate.push(0);
        } else {
            panic!()
        }
    }


    //println!(": {:?}", epsilon_rate);
}

fn get_epsilon(inputs: &Vec<&str>) -> Vec<char>{

    let mut epsilon_rate = Vec::new();
    // let mut gamma_rate = Vec::new();

    for column_idx in 0..inputs[0].len() {
        let mut ones = 0;
        let mut zeros = 0;
        for line_idx in 0..inputs.len() {
            let character = inputs[line_idx].as_bytes()[column_idx] as char;
            match character {
                '1' => ones += 1,
                '0' => zeros += 1,
                _ => panic!()
            }
        }
        if ones > zeros {
            epsilon_rate.push('1');
        } else if ones < zeros {
            epsilon_rate.push('0');
        } else {
            epsilon_rate.push('1');
        }
    }
    return epsilon_rate;
}

fn get_gamma(inputs: &Vec<&str>) -> Vec<char>{
    let eps = get_epsilon(inputs);

    let mut result = Vec::new();

    for line in eps.iter() {
        match line {
            '0' => result.push('1'),
            '1' => result.push('0'),
            _ => panic!()
        }
    }
    return result;
}

fn day3_2(input: &String) {

    let inputs: Vec<&str> = input.split("\n").filter(|s| s != &"").collect();

    let mut remainder: Vec<&str> = inputs;

    for column_idx in 0..remainder[0].len() {
        if remainder.len() == 1 {
            println!("Here!");
            break;
        }
        let eps = get_gamma(&remainder);
        let required_char: char = eps[column_idx];
        let mut rem_loc = Vec::new();
        for line_idx in 0..remainder.len() {
            if remainder[line_idx].as_bytes()[column_idx] as char == required_char {
                rem_loc.push(remainder[line_idx]);
            }
        }
        remainder = rem_loc;
    }
    println!("{:?}", remainder);
}
