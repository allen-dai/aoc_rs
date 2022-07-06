use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() {
    let file = File::open("./input.txt").expect("Can not open file");
    let reader = BufReader::new(file);

    let mut split_vec: Vec<String> = Vec::new();
    let mut aim: i64 = 0;
    let mut depth: i64 = 0;
    let mut hor: i64 = 0;
    let mut x: i64 = 0;

    for l in reader.lines() {
        split_vec = l.unwrap().split(" ").map(|s| s.to_string()).collect();
        x = split_vec[1].parse::<i64>().unwrap(); 
        if split_vec[0] == "down"{
            aim += x;
        }
        else if split_vec[0] == "up"{
            aim -= x;
       }
        else{
            hor += x;
            depth += aim * split_vec[1].parse::<i64>().unwrap();
        }
        
    }
    println!("{}", hor * depth);
}
