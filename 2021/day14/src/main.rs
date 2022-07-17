use std::collections::HashMap;
use std::error::Error;
use std::io::{stdin, Read};

type Result<T> = std::result::Result<T, Box<dyn Error>>;

fn main() -> Result<()> {
    let mut poly = parse_input()?;
    poly.mutate(10)?;
    println!("{:?}", poly.val);
    println!("{:?}", poly.score());

    Ok(())
}

fn parse_in() -> Result<(String, HashMap<(usize, usize), usize>)> {
    let mut s = String::new();
    stdin().read_to_string(&mut s)?;
    let mut map = HashMap::new();
    let input = s.lines().next().unwrap().to_string();

    for line in s.lines().skip(1) {
        if line.is_empty() {
            continue;
        }
        let sp = line.split(" -> ").collect::<Vec<&str>>();
        let key = sp[0].as_bytes();
        let val = sp[1].as_bytes();
        map.entry((key[0] as usize, key[1] as usize))
            .or_insert(val[0] as usize);
    }

    Ok((input, map))
}

// -------------------- Well, have no problem with the example, dk why wont work. -----------------------
fn parse_input() -> Result<Poly> {
    let mut s = String::new();
    stdin().read_to_string(&mut s)?;
    let mut map = HashMap::new();
    let input = s.lines().next().unwrap().to_string();

    for line in s.lines().skip(1) {
        if line.is_empty() {
            continue;
        }
        let sp = line.split(" -> ").collect::<Vec<&str>>();
        let mut key = sp[0].chars().into_iter();
        let val = sp[1].chars().next().unwrap();
        map.entry((key.next().unwrap(), key.next().unwrap()))
            .or_insert(val);
    }

    Ok(Poly { val: input, map })
}

#[derive(Clone)]
struct Poly {
    val: String,
    map: HashMap<(char, char), char>,
}

impl Poly {
    pub fn mutate(&mut self, step: usize) -> Result<()> {
        let mut last = String::new();
        for _ in 0..step {
            self.val = self
                .val
                .chars()
                .into_iter()
                .zip(self.val.chars().into_iter().skip(1))
                .map(|(a, b)| {
                    let mut result = String::new();
                    result.push(a);
                    result.push(*self.map.get(&(a, b)).unwrap());
                    result.push(b);
                    let temp = result.clone();
                    if !last.is_empty() {
                        if result.chars().rev().last() == last.chars().last() {
                            result = result.chars().skip(1).collect();
                        }
                    }
                    last = temp;
                    result
                })
                .collect();
        }
        Ok(())
    }

    pub fn score(&self) -> Result<usize> {
        let mut freq = HashMap::new();

        for c in self.val.chars() {
            *freq.entry(c).or_insert(0) += 1;
        }

        let most = freq.values().max().unwrap();
        let least = freq.values().min().unwrap();

        Ok(most - least)
    }
}

#[test]
fn poly_test() -> Result<()> {
    let poly = parse_input()?;

    let mut step1 = poly.clone();
    let mut step2 = poly.clone();
    let mut step3 = poly.clone();
    let mut step4 = poly.clone();

    step1.mutate(1)?;
    step2.mutate(2)?;
    step3.mutate(3)?;
    step4.mutate(4)?;

    assert_eq!(step1.val, "NCNBCHB");
    assert_eq!(step2.val, "NBCCNBBBCBHCB");
    assert_eq!(step3.val, "NBBBCNCCNBBNBNBBCHBHHBCHB");
    assert_eq!(
        step4.val,
        "NBBNBNBBCCNBCNCCNBBNBBNBBBNBBNBBCBHCBHHNHCBBCBHCB"
    );

    Ok(())
}
