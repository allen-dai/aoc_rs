use std::collections::HashMap;
use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() {
    let file = File::open("./src/input.txt").expect("Can not open this file");
    let reader = BufReader::new(file);

    let mut points: HashMap<Vec<i64>, i64> = HashMap::new();

    for l in reader.lines() {
        let line_vec: Vec<Vec<i64>> = l
            .unwrap()
            .split(" -> ")
            .map(|s| {
                s.split(",")
                    .map(|x| x.to_string().parse::<i64>().unwrap())
                    .collect()
            })
            .collect();

        let x1 = line_vec[0][0];
        let y1 = line_vec[0][1];
        let x2 = line_vec[1][0];
        let y2 = line_vec[1][1];

        /* let x1 = std::cmp::min(_x1, _x2);
                let x2 = std::cmp::max(_x1, _x2);
                let y1 = std::cmp::min(_y1, _y2);
                let y2 = std::cmp::max(_y1, _y2);

                println!("{} {} | {} {}", x1, y1, x2, y2);
        */
        let dx = x2 - x1;
        let dy = y2 - y1;
        for i in 0..(1 + std::cmp::max(dx.abs(), dy.abs())) {
            let mut x: i64;
            let mut y: i64;

            if dx > 0 {
                x = i;
            } else if dx < 0 {
                x = i * -1;
            } else {
                x = 0;
            }

            if dy > 0 {
                y = i;
            } else if dy < 0 {
                y = i * -1;
            } else {
                y = 0;
            }

            x += x1;
            y += y1;

            let point = vec![x, y];
            if !points.contains_key(&point) {
                points.insert(point.clone(), 0);
            }
            *points.get_mut(&point).unwrap() += 1;
        }
    }
    let mut ans: i64 = 0;

    for (k, v) in points.iter() {
        println!("{:?} {}", k, v);
        if *v > 1 {
            ans += 1;
        }
    }

    println!("{}", ans);
}
