use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() {
    let file = File::open("./input.txt").expect("Could not open input.txt");
    let reader = BufReader::new(file);
    let mut lines: Vec<String> = reader
        .lines()
        .map(|l| l.expect("Could not parse line"))
        .collect();

    let width = lines[0].len();

    let mut zeros: i32 = 0;
    let mut ones: i32 = 0;
    let mut ox_rate = String::new();
    let mut co2_rate = String::new();

    let copy_lines = lines.clone();

    for x in 0..2 {
        lines = copy_lines.clone();
        for i in 0..width {
            zeros = 0;
            ones = 0;
            for j in lines.iter() {
                let digit = j.chars().nth(i).unwrap();
                if digit == '0' {
                    zeros += 1;
                } else {
                    ones += 1;
                }
            }

            if lines.len() > 1 {
                if zeros < ones {
                    if x == 0 {
                        lines.retain(|line| line.chars().nth(i).unwrap() == '1');
                    } else {
                        lines.retain(|line| line.chars().nth(i).unwrap() == '0');
                    }
                } else if zeros > ones {
                    if x == 0 {
                        lines.retain(|line| line.chars().nth(i).unwrap() == '0');
                    } else {
                        lines.retain(|line| line.chars().nth(i).unwrap() == '1');
                    }
                } else {
                    if x == 0 {
                        lines.retain(|line| line.chars().nth(i).unwrap() == '1');
                    } else {
                        lines.retain(|line| line.chars().nth(i).unwrap() == '0');
                    }
                }
            }
        }

        if x == 0 {
            ox_rate = lines[0].clone();
        } else {
            co2_rate = lines[0].clone();
        }
    }

    println!("{} {}", ox_rate, co2_rate);

    println!(
        "{}",
        i32::from_str_radix(&ox_rate, 2).expect("err")
            * i32::from_str_radix(&co2_rate, 2).expect("err")
    );
}
