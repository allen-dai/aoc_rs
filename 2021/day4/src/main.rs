use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() {
    let file = File::open("./src/input.txt").expect("Could not open file");
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();

    let board_input: Vec<String> = lines[0].clone().split(",").map(|s| s.to_string()).collect();

    let mut boards: Vec<Vec<Vec<&str>>> = Vec::new();
    let mut board: Vec<Vec<&str>> = Vec::new();
    let mut row: Vec<&str> = Vec::new();

    let mut winners = 0;

    for i in 2..lines.len() {
        // Push() trigger by empty line
        if lines[i].is_empty() {
            boards.push(board.clone());
            board.clear();
            row.clear();
        }
        // EOF - Last line push
        else if i == lines.len() - 1 {
            row = lines[i].split_whitespace().collect();
            board.push(row.clone());
            boards.push(board.clone());
        } else {
            row = lines[i].split_whitespace().collect();
            board.push(row.clone());
        }
    }

    let mut copy_boards = boards.clone();

    for board in 0..copy_boards.len() {
        for row in 0..copy_boards[board].len() {
            for n in 0..copy_boards[board][row].len() {
                copy_boards[board][row][n] = "";
            }
        }
    }

    let mut winner = 0;
    let mut winner_call = 0;

    let mut win_board_index = -1;

    'outer: for input in board_input.iter() {
        'inner: for board in 0..boards.len() {
            for row in 0..boards[board].len() {
                for n in 0..boards[board][row].len() {
                    if boards[board][row][n].eq(input) {
                        copy_boards[board][row][n] = input;

                        win_board_index = check_winboard_index(&copy_boards);

                        if win_board_index != -1 && boards.len() > 1 {
                            boards.remove(win_board_index as usize);
                            copy_boards.remove(win_board_index as usize);
                            break 'inner;
                        }
                    }
                }
            }
        }
    }

    for board in 0..copy_boards.len() {
        for row in 0..copy_boards[board].len() {
            for n in 0..copy_boards[board][row].len() {
                copy_boards[board][row][n] = "";
            }
        }
    }

    'outer:  for input in board_input.iter() {
        for board in 0..boards.len() {
            for row in 0..boards[board].len() {
                for n in 0..boards[board][row].len() {
                    if boards[board][row][n].eq(input) {
                        copy_boards[board][row][n] = input;

                        if check_board(&copy_boards) {
                            winner_call = input.to_string().parse::<usize>().unwrap();
                            break 'outer;
                        }
                    }
                }
            }
        }
    }


    let score = calc_score(&boards[0], &copy_boards[0]);

    println!("{} {:?}", score * winner_call, copy_boards.len());
}
fn check_board(boards: &Vec<Vec<Vec<&str>>>) -> bool {
    // Check row
    for board in boards {
        for row in board {
            if !row.contains(&"") {
                return true;
            }
        }
    }

    // Check col
    let mut temp: bool = true;
    for i in 0..boards[0].len() {
        for board in boards {
            // Checking columns, if contains empty string, temp = false
            for row in board {
                if row[i].len() == 0 {
                    temp = false
                }
            }

            // If temp remain true, means we have a winning board, then return true.
            if temp {
                return true;
            }

            // At this step, temp is false, make it true again.
            temp = true;
        }
    }
    return false;
}

fn check_winboard_index(boards: &Vec<Vec<Vec<&str>>>) -> i32 {
    // Check row
    for i in 0..boards.len() {
        for row in &boards[i] {
            if !row.contains(&"") {
                return i as i32;
            }
        }
    }

    // Check col
    let mut temp: bool = true;
    for i in 0..boards[0].len() {
        for j in 0..boards.len() {
            // Checking columns, if contains empty string, temp = false
            for row in &boards[j] {
                if row[i].len() == 0 {
                    temp = false
                }
            }

            // If temp remain true, means we have a winning board, then return true.
            if temp {
                return j as i32;
            }

            // At this step, temp is false, make it true again.
            temp = true;
        }
    }
    return -1;
}

fn calc_score(og: &Vec<Vec<&str>>, copy: &Vec<Vec<&str>>) -> usize {
    let mut unmarked = 0;
    for i in 0..og.len() {
        for j in 0..og[0].len() {
            if copy[i][j] == "" {
                unmarked += og[i][j].to_string().parse::<usize>().unwrap();
            }
        }
    }
    return unmarked;
}
