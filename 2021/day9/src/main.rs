use std::collections::VecDeque;
use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() {
    let file = File::open("./src/in.2").expect("");
    let reader = BufReader::new(file);

    let mut input: Vec<Vec<i64>> = Vec::new();

    for l in reader.lines() {
        let line: Vec<i64> = l
            .unwrap()
            .chars()
            .map(|c| c.to_digit(10).unwrap() as i64)
            .collect();
        input.push(line);
    }

    let R = input.len();
    let C = input[0].len();

    let mut seen: Vec<(i64, i64)> = Vec::new();

    let mut cluster: Vec<i64> = Vec::new();

    let DR = vec![-1, 0, 1, 0];
    let DC = vec![0, 1, 0, -1];

    for r in 0..R {
        for c in 0..C {
            let point = (r as i64, c as i64);

            if input[r][c] != 9 && !seen.contains(&point) {
                let mut size = 0;

                let mut queue: VecDeque<(i64, i64)> = VecDeque::new();

                queue.push_back(point);

                while !queue.is_empty() {
                    let point = queue.pop_front().unwrap();
                    //println!("{:?}", queue);
                    if seen.contains(&point) {
                        continue;
                    }
                    seen.push(point.clone());

                    size += 1;
                    let pr = point.0;
                    let pc = point.1;
                    for d in 0..4 {
                        let rr = pr + DR[d];
                        let cc = pc + DC[d];

                        if rr >= 0
                            && rr < R as i64
                            && cc >= 0
                            && cc < C as i64
                            && input[rr as usize][cc as usize] != 9
                        {
                            queue.push_back((rr, cc));
                        }
                    }
                }
                cluster.push(size);
            }
        }
    }

    cluster.sort();
    let l = cluster.len() - 1;

    println!("{:?}", cluster[l] * cluster[l - 1] * cluster[l - 2]);
}
