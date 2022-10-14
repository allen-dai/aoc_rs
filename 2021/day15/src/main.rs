#![allow(non_snake_case)]

use std::{
    collections::{BinaryHeap, HashMap, HashSet, VecDeque},
    fs,
};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let file = fs::read_to_string("src/in.1")?;
    let content: Vec<&str> = file.split("\n").collect();
    let mut graph: Vec<Vec<isize>> = content
        .iter()
        .map(|line| {
            line.as_bytes()
                .iter()
                .map(|c| -((c - b'0') as isize))
                .collect()
        })
        .collect();
    graph.pop();
    let R = graph.len();
    let C = graph[0].len();
    let mut new_graph = vec![vec![0; C * 5]; R * 5];
    let mut D = vec![vec![None; C * 5]; R * 5];
    for r in 0..graph.len() * 5 {
        for c in 0..graph[0].len() * 5 {
            let mut val = graph[r % R][c % C] - (r / R + c / C) as isize;
            while val < -9 {
                val += 9;
            }
            new_graph[r][c] = val;
        }
    }
    println!("{:?}", new_graph);
    let mut Q = BinaryHeap::new();
    Q.push((0, 0, 0));

    let rd: Vec<isize> = vec![0, 0, 1, -1];
    let cd: Vec<isize> = vec![1, -1, 0, 0];

    while !Q.is_empty() {
        let (dist, r, c) = Q.pop().unwrap();
        if r < 0 || c < 0 || r >= (R * 5) as isize || c >= (C * 5) as isize {
            continue;
        }
        let mut curr_dist = dist + new_graph[r as usize][c as usize];
        if D[r as usize][c as usize].is_none() || D[r as usize][c as usize].unwrap() > dist {
            D[r as usize][c as usize] = Some(curr_dist);
        } else {
            continue;
        }
        if (r as usize, c as usize) == (R * 5 - 1, C * 5 - 1) {
            break;
        }
        for i in 0..4 {
            let rr = r as isize + rd[i];
            let cc = c as isize + cd[i];
            Q.push((curr_dist, rr, cc));
        }
    }
    println!(
        "{}",
        D[R * 5 - 1][C * 5 - 1].unwrap() as isize - new_graph[0][0]
    );
    Ok(())
}
