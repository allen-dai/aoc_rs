#![feature(toowned_clone_into)]
use regex::Regex;
use std::collections::HashSet;
use std::io::{self, Read};
use std::{error, result};

type Result<T> = result::Result<T, Box<dyn error::Error>>;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Point {
    x: usize,
    y: usize,
}

#[derive(Debug)]
enum Axis {
    YAxis,
    XAxis,
}

#[derive(Debug)]
struct Fold {
    axis: Axis,
    p: usize,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
struct Graph {
    graph: Vec<Vec<usize>>,
}

impl Graph {
    fn new(row: usize, col: usize) -> Self {
        Graph {
            graph: vec![vec![0; col + 1]; row + 1],
        }
    }

    fn filp_y(&mut self, y: usize) {
        let mut up = Vec::new();
        let mut down = Vec::new();
        let mut was_up_short = false;
        self.graph[..y].clone_into(&mut up);
        self.graph[y..].clone_into(&mut down);
        let (mut tallest, mut shortest);
        if up.len() >= down.len() {
            tallest = up;
            shortest = down;
        } else {
            tallest = down;
            shortest = up;
            was_up_short = true;
        }
        shortest.reverse();
        let mut rr = 0;
        for r in (tallest.len() - shortest.len())..tallest.len() {
            for c in 0..tallest[r].len() {
                tallest[r][c] = std::cmp::max(tallest[r][c], shortest[rr][c]);
            }
            rr += 1;
        }
        self.graph = tallest;
        if was_up_short {
            self.graph.reverse();
        }
    }

    fn flip_x(&mut self, x: usize) {
        let mut left = Vec::new();
        let mut right = Vec::new();
        for row in self.graph.iter() {
            let mut temp = Vec::new();
            for c in 0..x {
                temp.push(row[c]);
            }
            left.push(temp);
        }
        for row in self.graph.iter() {
            let mut temp = Vec::new();
            for c in x + 1..self.graph[0].len() {
                temp.push(row[c]);
            }
            right.push(temp);
        }
        let mut was_left_short = false;
        let (mut longest, mut shortest);
        if x + 1 > self.graph[0].len() / 2 {
            longest = left;
            shortest = right;
        } else {
            longest = right;
            shortest = left;
            was_left_short = true;
        }

        for r in 0..shortest.len() {
            shortest[r].reverse();
            let mut cc = 0;
            for c in longest[0].len() - shortest[0].len()..longest[0].len() {
                longest[r][c] = std::cmp::max(longest[r][c], shortest[r][cc]);
                cc += 1;
            }
            if was_left_short {
                longest[r].reverse();
            }
        }
        self.graph = longest;
    }

    fn dots(&self) -> i32 {
        let mut res = 0;
        for r in self.graph.iter() {
            for n in r {
                if *n == 1 {
                    res += 1;
                }
            }
        }
        res
    }
}

fn main() -> Result<()> {
    let s = input()?;
    let re = Regex::new(
        r"(?x)
        ((?P<x>\d+),(?P<y>\d+))
        |
        ((?P<fold>[xy]{1})=(?P<val>\d+))
        ",
    )
    .unwrap();

    let mut points = Vec::new();
    let mut actions = Vec::new();
    for line in s.lines() {
        if line.is_empty() {
            continue;
        }
        let caps = match re.captures(line) {
            Some(caps) => caps,
            None => {
                return Err("Unrecognized pattern".into());
            }
        };

        match caps.name("fold") {
            Some(fold) => {
                let axis: Axis;
                if fold.as_str() == "x" {
                    axis = Axis::XAxis;
                } else {
                    axis = Axis::YAxis;
                }
                actions.push(Fold {
                    axis,
                    p: caps["val"].parse()?,
                });
            }
            None => {
                points.push(Point {
                    x: caps["x"].parse()?,
                    y: caps["y"].parse()?,
                });
            }
        }
    }

    let mut max_row = 0;
    let mut max_col = 0;
    for p in points.iter() {
        if p.x > max_col {
            max_col = p.x;
        }
        if p.y > max_row {
            max_row = p.y
        }
    }
    let mut graph = Graph::new(max_row, max_col);
    for p in points.iter() {
        graph.graph[p.y][p.x] = 1;
    }
    //println!("{:?}", actions);
    part_1(&mut graph.clone(), &actions)?;
    part_2(&mut points, &actions)?;
    Ok(())
}

fn input() -> Result<String> {
    let mut s = String::new();
    io::stdin().read_to_string(&mut s)?;
    Ok(s.trim().to_string())
}

fn part_1(graph: &mut Graph, actions: &Vec<Fold>) -> Result<()> {
    let action = &actions[0];
    match action.axis {
        Axis::XAxis => {
            graph.flip_x(action.p);
        }
        Axis::YAxis => {
            graph.filp_y(action.p);
        }
    }

    println!("{}", graph.dots());
    Ok(())
}

fn part_2(points: &mut Vec<Point>, actions: &Vec<Fold>) -> Result<()> {
    let mut g = HashSet::new();
    for p in points {
        g.insert((p.x as i32, p.y as i32));
    }
    for action in actions {
        let mut new_g = HashSet::new();
        match action.axis {
            Axis::XAxis => {
                let v = action.p as i32;
                for (x, y) in g.clone() {
                    if x < v {
                        new_g.insert((x, y));
                    } else {
                        new_g.insert((v - (x - v), y));
                    }
                }
            }
            Axis::YAxis => {
                let v = action.p as i32;
                for (x, y) in g.clone() {
                    if y < v {
                        new_g.insert((x, y));
                    } else {
                        new_g.insert((x, v - (y - v)));
                    }
                }
            }
        }
        g = new_g;
    }
    let max_x: i32 = g.iter().map(|v| v.0).max().unwrap();
    let max_y: i32 = g.iter().map(|v| v.1).max().unwrap();
    for y in 0..max_y + 1 {
        let mut r = "".to_string();
        for x in 0..max_x + 1 {
            if g.contains(&(x, y)) {
                r += "XX";
            } else {
                r += "  ";
            }
        }
        println!("{}", r);
    }

    Ok(())
}
