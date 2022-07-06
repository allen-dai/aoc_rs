use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() {
    let file = File::open("./src/input.txt").expect("");
    let reader = BufReader::new(file);

    let mut crabs: Vec<i64> = Vec::new();
    for l in reader.lines() {
        crabs = l
            .unwrap()
            .split(",")
            .map(|p| p.to_string().parse::<i64>().unwrap())
            .collect();
    }

    crabs.sort();

    let median: i64 = (crabs.len() / 2) as i64;

    /* if crabs.len() % 2 != 0 {
        let medium: i64 = (crabs[crabs.len() / 2] + crabs[crabs.len() / 2 + 1]) / 2;
    }
    else{
        let medium: i64 = crabs[crabs.len() / 2];
    } */

    let mut ans: Vec<Vec<i64>> = Vec::new();

    /* for crab in &crabs {
        ans += (crab - median).abs();
    } */

    let mut total_fuel = 0;

    for i in 0..(&crabs)[(&crabs).len() - 1] {
        for n in &crabs {
            let num_int = (n - i).abs();
            total_fuel += num_int * (num_int + 1) / 2;
        }
        ans.push(vec![total_fuel, i]);
        total_fuel = 0;
    }

    ans.sort();
    println!("{:?}", ans[0]);
}
