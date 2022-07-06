#![allow(dead_code, unused)]

use std::collections::HashMap;
use std::error::Error;
use std::io::{stdin, stdout, Read, Write};

type Result<T> = std::result::Result<T, Box<dyn Error>>;

fn main() -> Result<()> {
    let mut s = String::new();
    stdin().read_to_string(&mut s)?;
    let mut input = String::new();
    let mut map = HashMap::new();
    input = s.lines().take(1).next().unwrap().to_string();

    for line in s.lines().skip(2) {
        if line.is_empty() {
            continue;
        }
        let sp = line.split(" -> ").collect::<Vec<&str>>();
        let key = sp[0];
        let val = sp[1];
        map.entry(key).or_insert(val);
    }
    for _ in 0..map.len() {
        let mut temp = String::new();
        for i in 0..input.len() - 1 {
            let s = map
                .get(&*String::from_utf8(input.as_bytes()[i..i + 1].to_vec()).unwrap())
                .unwrap();
            temp += s;
        }
        input += &temp;
    }
    println!("{:?}", input);

    Ok(())
}

fn ans(s: &str) -> Result<()> {
    Ok(())
}
