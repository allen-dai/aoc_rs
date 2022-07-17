use std::collections::HashMap;
use std::error::Error;
use std::io::{stdin, Read};

type Result<T> = std::result::Result<T, Box<dyn Error>>;

fn main() -> Result<()> {
    let (mut map, rules, last_c) = parse_in()?;
    // range: 10 = part 1, 40 = part 2
    for _ in 0..40 {
        let mut new_map = HashMap::new();
        for (k, v) in map.keys().zip(map.values()) {
            let new = rules.get(k).unwrap();
            *new_map.entry((k.0, *new)).or_insert(0) += v;
            *new_map.entry((*new, k.1)).or_insert(0) += v;
        }
        map = new_map;
    }
    let mut score = HashMap::new();
    for (k, v) in map.clone().keys().zip(map.clone().values()) {
        *score.entry(k.0).or_insert(0) += v;
    }
    *score.entry(last_c).or_insert(0) += 1;
    let max = score.values().max().unwrap();
    let min = score.values().min().unwrap();
    println!("{:?}", map);
    println!("{:?}", max - min);

    Ok(())
}

fn parse_in() -> Result<(HashMap<(u8, u8), usize>, HashMap<(u8, u8), u8>, u8)> {
    let mut s = String::new();
    stdin().read_to_string(&mut s)?;
    let mut rules = HashMap::new();
    let input = s.lines().next().unwrap().to_string();
    let last_c = input.as_bytes().last().unwrap();
    let mut map = HashMap::new();
    for (a, b) in input.as_bytes().iter().zip(input.as_bytes().iter().skip(1)) {
        *map.entry((a.clone(), b.clone())).or_insert(0) += 1;
    }

    for line in s.lines().skip(1) {
        if line.is_empty() {
            continue;
        }
        let sp = line.split(" -> ").collect::<Vec<&str>>();
        let key = sp[0].as_bytes();
        let val = sp[1].as_bytes();
        rules.entry((key[0], key[1])).or_insert(val[0]);
    }

    Ok((map, rules, *last_c))
}
