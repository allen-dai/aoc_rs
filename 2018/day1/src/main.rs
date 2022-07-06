use std::io::{stdin, stdout, Read, Write};

type MyResult<T> = Result<T, Box<dyn std::error::Error>>;

fn main() -> MyResult<()> {
    let mut s = String::new();
    stdin().read_to_string(&mut s)?;
    part_1(&s)?;
    part_2(&s)?;
    Ok(())
}

fn part_1(s: &str) -> MyResult<()> {
    let mut res = 0;
    for line in s.lines() {
        res += line.parse::<i32>()?;
    }
    writeln!(stdout(), "{}", res)?;
    Ok(())
}

fn part_2(s: &str) -> MyResult<()> {
    let mut seen = std::collections::HashSet::new();
    let mut curr = 0;
    seen.insert(curr);

    loop {
        for line in s.lines() {
            curr += line.parse::<i32>()?;
            if seen.contains(&curr) {
                writeln!(stdout(), "{}", curr)?;
                return Ok(());
            }
            seen.insert(curr);
        }
    }
}
