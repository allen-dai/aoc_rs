use std::collections::{HashMap, HashSet};
use std::error;
use std::io::{stdin, stdout, Read, Write};
use std::result;

type Result<T> = result::Result<T, Box<dyn error::Error>>;

struct Bracket {}

impl Bracket {
    fn is_pair(left: &char, right: &char) -> Result<bool> {
        let pairs = HashMap::from([('(', ')'), ('[', ']'), ('{', '}'), ('<', '>')]);
        match pairs.get(left) {
            Some(v) => Ok(v == right),
            None => Err("Invalid input".into()),
        }
    }

    fn is_left(c: &char) -> bool {
        let pairs = HashSet::from(['(', '[', '{', '<']);
        pairs.contains(c)
    }
}

fn main() -> Result<()> {
    let mut s = String::new();
    stdin().read_to_string(&mut s)?;

    let mut stack = Vec::new();

    let mut incorrect = Vec::new();
    let mut incomplete = Vec::new();
    for line in s.lines() {
        if line.is_empty() {
            continue;
        }
        for c in line.chars() {
            if Bracket::is_left(&c) {
                stack.push(c);
            } else {
                if let Some(left) = stack.last() {
                    match Bracket::is_pair(&left, &c) {
                        Ok(b) => {
                            if b {
                                stack.pop();
                            } else {
                                incorrect.push(c);
                                break;
                            }
                        }
                        Err(e) => {
                            return Err(e);
                        }
                    }
                }
            }
        }
        stack.reverse();
        incomplete.push(stack.clone());
        stack.clear();
    }

    part_1(&incorrect)?;
    part_2(&incomplete)?;

    Ok(())
}

fn part_1(s: &Vec<char>) -> Result<()> {
    let part_1_socre = HashMap::from([(')', 3), (']', 57), ('}', 1197), ('>', 25137)]);
    let mut res = 0;
    for c in s {
        if let Some(v) = part_1_socre.get(&c) {
            res += v;
        } else {
            return Err("Unmatchable character.".into());
        }
    }
    writeln!(stdout(), "{}", res)?;
    Ok(())
}

fn part_2(s: &Vec<Vec<char>>) -> Result<()> {
    let part_2_socre = HashMap::from([('(', 1), ('[', 2), ('{', 3), ('<', 4)]);
    let mut res = Vec::new();
    let mut temp: i128;
    for v in s {
        temp = 0;
        for c in v {
            if let Some(v) = part_2_socre.get(&c) {
                temp = temp * 5 + v;
            } else {
                return Err("Unmatchable character.".into());
            }
        }
        res.push(temp);
    }
    res.sort();
    writeln!(stdout(), "{:?}", res[res.len() / 2])?;
    Ok(())
}
