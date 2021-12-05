use std::io;
use std::fmt;
use std::collections::HashMap;


pub fn run(input: String) {
    day5_1(&input);
    day5_2(&input);
}

#[derive(PartialEq, Eq, Hash)]
#[derive(Debug, Copy, Clone)]
struct Coor {
    x: i32,
    y: i32
}

#[derive(Debug, Copy, Clone)]
struct Line {
    begin: Coor,
    end: Coor
}

fn get_coordinates_to_mark(line: &Line) -> Vec<Coor> {
    let mut coords = Vec::new();
    if line.begin.x == line.end.x {
        if line.begin.y < line.end.y {
            for i in line.begin.y..line.end.y+1 {
                coords.push(Coor {
                    x: line.begin.x,
                    y: i
                });
            }
        } else if line.begin.y > line.end.y {
            for i in line.end.y..line.begin.y+1 {
                coords.push(Coor {
                    x: line.begin.x,
                    y: i
                });
            }
        } else {
            coords.push(Coor {
                x: line.begin.x,
                y: line.begin.y
            });
        }
    } else if line.begin.y == line.end.y {
        if line.begin.x < line.end.x {
            for i in line.begin.x..line.end.x+1 {
                coords.push(Coor {
                    x: i,
                    y: line.begin.y
                });
            }
        } else if line.begin.x > line.end.x {
            for i in line.end.x..line.begin.x+1 {
                coords.push(Coor {
                    x: i,
                    y: line.begin.y
                });
            }
        } else {
            coords.push(Coor {
                x: line.begin.x,
                y: line.begin.y
            });
        }
    } else {
        println!("Panicing for line {:?}", &line);
        panic!();
    }
    // println!("Inserting for line {:?} coords {:?}", line, coords);
    return coords;
}

fn get_coordinates_to_mark2(line: &Line) -> Vec<Coor> {
    let mut coords = Vec::new();
    if line.begin.x == line.end.x {
        if line.begin.y < line.end.y {
            for i in line.begin.y..line.end.y+1 {
                coords.push(Coor {
                    x: line.begin.x,
                    y: i
                });
            }
        } else if line.begin.y > line.end.y {
            for i in line.end.y..line.begin.y+1 {
                coords.push(Coor {
                    x: line.begin.x,
                    y: i
                });
            }
        } else {
            coords.push(Coor {
                x: line.begin.x,
                y: line.begin.y
            });
        }
    } else if line.begin.y == line.end.y {
        if line.begin.x < line.end.x {
            for i in line.begin.x..line.end.x+1 {
                coords.push(Coor {
                    x: i,
                    y: line.begin.y
                });
            }
        } else if line.begin.x > line.end.x {
            for i in line.end.x..line.begin.x+1 {
                coords.push(Coor {
                    x: i,
                    y: line.begin.y
                });
            }
        } else {
            coords.push(Coor {
                x: line.begin.x,
                y: line.begin.y
            });
        }
    } else {

        let x_diff = line.begin.x - line.end.x;
        let y_diff = line.begin.y - line.end.y;

        assert_eq!(x_diff.abs(), y_diff.abs());

        let abs = x_diff.abs();

        if x_diff < 0 && y_diff < 0 { // Rechts oben
            for i in 0..abs+1{
                coords.push(Coor {
                    x: line.begin.x + i,
                    y: line.begin.y + i
                });
            }
        } else if x_diff > 0 && y_diff > 0 { // Links unten
            for i in 0..abs+1{
                coords.push(Coor {
                    x: line.begin.x - i,
                    y: line.begin.y - i
                });
            }
        } else if x_diff < 0 && y_diff > 0 { // Rechts unten
            for i in 0..abs+1{
                coords.push(Coor {
                    x: line.begin.x + i,
                    y: line.begin.y - i
                });
            }
        } else if x_diff > 0 && y_diff < 0 { // Links oben
            for i in 0..abs+1{
                coords.push(Coor {
                    x: line.begin.x - i,
                    y: line.begin.y + i
                });
            }
        } else {
            panic!();
        }
    }
    // println!("Inserting for line {:?} coords {:?}", line, coords);
    return coords;
}

fn day5_1(input: &String) {

    let inputs: Vec<&str> = input.trim().split("\n").collect();

    let mut lines = Vec::new();

    for line in inputs.iter() {
        let xy: Vec<&str> = line.split(" -> ").collect();
        let start = xy[0];
        let end = xy[1];

        let start_parts: Vec<i32> = start.split(',').map(|n| n.parse::<i32>().unwrap()).collect();
        let start_coord = Coor {
            x: start_parts[0],
            y: start_parts[1]
        };

        let end_parts: Vec<i32> = end.split(',').map(|n| n.parse::<i32>().unwrap()).collect();
        let end_coord = Coor {
            x: end_parts[0],
            y: end_parts[1]
        };

        if start_coord.x == end_coord.x || start_coord.y == end_coord.y {
            lines.push(Line {
                begin: start_coord,
                end: end_coord
            });
        }
    }

    let mut hit_coords: HashMap<Coor, i32> = HashMap::new();

    for line in lines {
        let coordinates_to_mark = get_coordinates_to_mark(&line);
        for coor in coordinates_to_mark.iter() {
            if hit_coords.contains_key(coor) {
                let hit_count = hit_coords.get_mut(coor).unwrap();
                *hit_count += 1;
            } else {
                hit_coords.insert(*coor, 1);
            }
        }
    }
    
    // println!("Hit coords: {:?}", hit_coords);

    let greater_two_hits: Vec<i32> = hit_coords.into_iter()
        .map(|(_k, v)| v)
        .filter(|v| v > &1).collect();

    println!("{} coordinates where hit more than once.", greater_two_hits.len());

}

fn day5_2(input: &String) {

    let inputs: Vec<&str> = input.trim().split("\n").collect();

    let mut lines = Vec::new();

    for line in inputs.iter() {
        let xy: Vec<&str> = line.split(" -> ").collect();
        let start = xy[0];
        let end = xy[1];

        let start_parts: Vec<i32> = start.split(',').map(|n| n.parse::<i32>().unwrap()).collect();
        let start_coord = Coor {
            x: start_parts[0],
            y: start_parts[1]
        };

        let end_parts: Vec<i32> = end.split(',').map(|n| n.parse::<i32>().unwrap()).collect();
        let end_coord = Coor {
            x: end_parts[0],
            y: end_parts[1]
        };

        if true {
            lines.push(Line {
                begin: start_coord,
                end: end_coord
            });
        }
    }

    let mut hit_coords: HashMap<Coor, i32> = HashMap::new();

    for line in lines {
        let coordinates_to_mark = get_coordinates_to_mark2(&line);
        for coor in coordinates_to_mark.iter() {
            if hit_coords.contains_key(coor) {
                let hit_count = hit_coords.get_mut(coor).unwrap();
                *hit_count += 1;
            } else {
                hit_coords.insert(*coor, 1);
            }
        }
    }
    
    // println!("Hit coords: {:?}", hit_coords);

    let greater_two_hits: Vec<i32> = hit_coords.into_iter()
        .map(|(_k, v)| v)
        .filter(|v| v > &1).collect();

    println!("{} coordinates where hit more than once.", greater_two_hits.len());


}
