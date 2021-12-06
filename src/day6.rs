use std::io;
use std::fmt;
use std::collections::HashMap;


pub fn run(input: String) {
    day6_1(&input);
    day6_2(&input);
}

fn day6_1(input: &String) {
    let mut nums: Vec<i64> = input.trim().split(',').map(|s| s.parse::<i64>().unwrap()).collect();

    println!("{:?}", nums);

    let mut map: HashMap<i64, i64> = HashMap::new();

    for num in nums.iter() {
        let v = map.entry(*num).or_insert_with(|| 0);
        *v += 1;
    }

    let mut map2: HashMap<i64, i64> = map.clone();
    for i in 0..9 {
        if !map2.contains_key(&i) {
            map2.insert(i, 0);
        }
    }
    println!("Nums: {:?}", map2);

    for day in 1..257 {
        println!("Day {}", day);
        let mut new_map: HashMap<i64, i64> = HashMap::new();
        for k in 0..9 {
            let v = map2.get(&k).unwrap_or(&0);
            if k == 7 {
                let prev = new_map.get(&6).unwrap_or(&0);
                new_map.insert(k - 1, *prev + *v);
                new_map.insert(k, 0);
            } else if k != 0 {
                if k != 6 && k!= 8 {
                    new_map.insert(k, 0);
                }
                new_map.insert(k - 1, *v);

            } else if k == 0 {
                new_map.insert(6, *v);
                new_map.insert(8, *v);
                new_map.insert(0, 0);
            }
        }
        map2 = new_map;
        // println!("Fish after day {:?}: {:?}", day, map2);

        // let mut print_vec = String::new();
        // for i in 0..9 {
        //     let amount = map2.get(&i).unwrap_or(&0);
        //     for _ in 0..*amount {
        //         print_vec.push_str(&i.to_string());
        //     }
        //     print_vec.push_str(" ");
        // }
        // println!("Vec {:?}", print_vec);
    }

    let mut count: i64 = 0;
    for (_, v) in map2.iter() {
        count += v;
    }
    println!("Total fish after day {:?}: {:?}", 80, count);
    println!("");

}

fn day6_2(input: &String) {

}