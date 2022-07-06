use std::collections::HashMap;
use std::io::{stdin, stdout, Read, Write};

type MyResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> MyResult<()> {
    let mut s = String::new();
    stdin().read_to_string(&mut s)?;
    part_1(&s)?;
    part_2(&s)?;
    Ok(())
}

fn part_1(s: &str) -> MyResult<()> {
    let (mut twos, mut threes) = (0, 0);
    for line in s.lines() {
        let mut line_map = HashMap::new();
        for c in line.chars() {
            *line_map.entry(c).or_insert(0) += 1;
        }

        let mut two_added = false;
        let mut three_added = false;

        for (_, rep) in line_map {
            match rep {
                2 => {
                    if !two_added {
                        twos += 1;
                        two_added = true;
                    }
                }
                3 => {
                    if !three_added {
                        threes += 1;
                        three_added = true;
                    }
                }
                _ => continue,
            }
        }
    }

    writeln!(stdout(), "{}", twos * threes)?;
    Ok(())
}

fn part_2(s: &str) -> MyResult<()> {
    let s_v: Vec<&str> = s.lines().collect();
    let line_length = s_v[0].len();
    let mut res: Vec<(String, usize)> = Vec::new();
    for i in 0..s_v.len() {
        let curr_line = s_v[i].as_bytes();
        for j in i + 1..s_v.len() {
            let mut diff = 0;
            let mut index = 0;
            let next_line = s_v[j].as_bytes();
            for c in 0..line_length {
                if curr_line[c] != next_line[c] {
                    diff += 1;
                    index = c;
                }
            }
            if diff == 1 {
                let l: String = curr_line
                    .iter()
                    .enumerate()
                    .filter(|(i, _)| i != &index)
                    .map(|(_, v)| *v as char)
                    .collect();
                res.push((l, index));
            }
        }
    }

    println!("{:?}", res);
    Ok(())
}
