use std::io;
use std::fmt;

#[derive(PartialEq)]
#[derive(Debug)]
struct Cell {
    number: i32,
    checked: bool
}

impl fmt::Display for Cell {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        if self.checked {
            fmt.write_str("X")?;
        } else {
            fmt.write_str("_")?;
        }
        fmt.write_str(&self.number.to_string());
        Ok(())
    }
}

#[derive(PartialEq)]
#[derive(Debug)]
struct Board {
    rows: Vec<Vec<Cell>>
}

pub fn run(input: String) {
    day4_1(&input);
    day4_2(&input);
}

fn sum_unmarked(board: &Board) -> i32 {
    let mut sum = 0;
    for row in board.rows.iter() {
        for cell in row.iter() {
            if cell.checked == false {
                sum += cell.number;
            }
        }
    }
    return sum;
}

fn check_for_win(board: &Board) -> bool {
    let wincount = board.rows[0].len();
    for row in board.rows.iter() {
        let mut matches = 0;
        for cell in row.iter() {
            if cell.checked {
                matches += 1;
            }
        }
        if matches == wincount {
            println!("Winner: {:?}", board);
            return true;
        }
    }
    for col_idx in 0..board.rows.len() {
        let mut matches = 0;
        for row in board.rows.iter() {
            if row[col_idx].checked {
                matches += 1;
            }
        }
        if matches == wincount {
            println!("Winner: {:?}", board);
            return true;
        }
    }
    return false;
}

fn check_for_match1(boards: &Vec<Board>, draw: &i32) -> bool {
      // Check for match
      for board in boards.iter() {
        if check_for_win(&board) {
            // println!("Current draw {:?}", draw);
            let sum = sum_unmarked(&board);
            // println!("Sum unmarked {:?}", sum);
            // println!("Result 1 {:?}", sum * draw);
            return true;
        }
    }
    return false;
}

fn check_for_match2(boards: &Vec<Board>) -> Vec<&Board> {
    return boards.iter().filter(|b| check_for_win(b)).collect();
}

fn day4_1(input: &String) {

    let mut inputs: Vec<&str> = input.split("\n").collect();

    let mut boards: Vec<Board> = Vec::new();
    let mut current_board: Board = Board {
        rows: Vec::new()
    };

    let drawn_numbers: Vec<i32> = inputs.remove(0).split(',').map(|i| i.parse::<i32>().unwrap()).collect();
    inputs.remove(0); // Remove empty line
    
    for line in inputs {
        // println!("Current line: {}", line);
        if line == "" {
            boards.push(current_board);
            current_board = Board {
                rows: Vec::new()
            };
        } else {
            let row: Vec<Cell> = line.split_whitespace()
                .map(|s| s.parse::<i32>().unwrap())
                .map(|n| Cell {
                    number: n,
                    checked: false
                })
                .collect();
            current_board.rows.push(row);
        }
    }
    
    // Part 1
    for draw in drawn_numbers.iter() {

        // Mark all boards
        for board in boards.iter_mut() {
            for row in board.rows.iter_mut() {
                for cell in row.iter_mut() {
                    if cell.number == *draw {
                        cell.checked = true
                    }
                }
            }
        }

        if check_for_match1(&boards, draw) {
            break;
        }
    }


    // Part 2 new
    for draw in drawn_numbers.iter() {
        let mut is_to_remove = Vec::new();

        // Mark all boards
        for board in boards.iter_mut() {
            for row in board.rows.iter_mut() {
                for cell in row.iter_mut() {
                    if cell.number == *draw {
                        cell.checked = true
                    }
                }
            }
        }

        let winners = check_for_match2(&boards);
        println!("Winners for draw {}", draw);

        for winner in winners {
            println!("{:?}", winner);
            let index = boards.iter().position(|x| *x == *winner).unwrap();
            is_to_remove.push(index);
        }

        println!("is_to_remove {:?}", is_to_remove);
        is_to_remove.reverse();

        if boards.len() == 1 && is_to_remove.len() == 1 {
            println!("Last winning board {:?}", boards);
            let sum = sum_unmarked(&boards[0]);
            println!("Resukt {:?}", sum * draw);
            break;
        }

        println!("Boards len before {}", boards.len());
        for i in is_to_remove.iter() {
            boards.remove(*i);
        }
        println!("Boards len after  {}", boards.len());

        println!("=======================");

            
    }

}

fn day4_2(input: &String) {

}
