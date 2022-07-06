use regex::Regex;
use std::collections::{HashMap, HashSet, VecDeque};
use std::io::{self, Read, Write};
use std::{error, result};

type Result<T> = result::Result<T, Box<dyn error::Error>>;

fn main() -> Result<()> {
    let mut s = String::new();
    io::stdin().read_to_string(&mut s)?;
    s = s.trim().to_string();

    let mut map = HashMap::new();

    let re = Regex::new(
        r"(?x)
                        (?P<start>\S+)
                        -
                        (?P<end>\S+)
                        ",
    )
    .unwrap();

    for line in s.lines() {
        if let Some(cap) = re.captures(line) {
            let start = cap["start"].to_string();
            let end = cap["end"].to_string();
            map.entry(start.clone()).or_insert(vec![]).push(end.clone());
            map.entry(end.clone()).or_insert(vec![]).push(start.clone());
        } else {
            return Err("unexpected capture".into());
        }
    }
    //println!("{:?}", map);
    part_1(&map)?;
    part_2(&map)?;
    Ok(())
}

fn part_1(map: &HashMap<String, Vec<String>>) -> Result<()> {
    let mut q: VecDeque<(String, HashSet<String>)> = VecDeque::new();
    let mut ans = 0;
    q.push_back(("start".to_string(), HashSet::from(["start".to_string()])));
    while !q.is_empty() {
        let (pos, small) = q.pop_front().unwrap();
        if pos == "end" {
            ans += 1;
            continue;
        }
        for n in map[&pos.to_string()].iter() {
            if n.eq("start") || pos.eq(n){
                continue;
            }
            if !small.contains(n) {
                let mut new_small = small.clone();
                if n.to_lowercase().eq(n) {
                    new_small.insert(n.to_string());
                }
                q.push_back((n.to_string(), new_small));
            }
        }
    }
    println!("{}", ans);

    Ok(())
}


fn part_2(map: &HashMap<String, Vec<String>>) -> Result<()> {
    let mut q: VecDeque<(String, Vec<String>, bool)> = VecDeque::new();
    let mut ans = 0;
    q.push_back(("start".to_string(), Vec::from(["start".to_string()]), false));
    while !q.is_empty() {
        let (pos, small, twice) = q.pop_front().unwrap();
        if pos == "end" {
            ans += 1;
            continue;
        }
        for n in map[&pos.to_string()].iter() {
            if n.eq("start") || pos.eq(n){
                continue;
            }
            if !small.contains(n) {
                let mut new_small = small.clone();
                if n.to_lowercase().eq(n) {
                    new_small.push(n.to_string());
                }
                q.push_back((n.to_string(), new_small, twice));
            }
            else if !twice{
                q.push_back((n.to_string(), small.clone(), true));
            }
        }
    }
    println!("{}", ans);

    Ok(())
}
