use std::collections::HashMap;
use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() {
    let file = File::open("./src/input.txt").expect("Can not open file");
    let reader = BufReader::new(file);

    let mut fishes: Vec<i128> = Vec::new();
    for l in reader.lines() {
        fishes = l
            .unwrap()
            .split(",")
            .map(|s| s.to_string().parse::<i128>().unwrap())
            .collect();
    }

    let mut X: HashMap<i128, i128> = HashMap::new();

    for fish in fishes {
        *X.entry(fish).or_insert(0) += 1;
    }

    for _ in 0..256 {
        let mut Y: HashMap<i128, i128> = HashMap::new();
        for (k, v) in X.iter() {
            println!("{}, {}", *k, *v);
            if *k == 0 {
                *Y.entry(6).or_insert(0) += *v;
                *Y.entry(8).or_insert(0) += *v;
            } else {
                *Y.entry(k - 1).or_insert(0) += *v;
            }
        }

        X = Y;
    }

    let mut ans: i128 = 0;

    for (_, v) in X.iter() {
        ans += v;
    }

    println!("{}", ans);
}
