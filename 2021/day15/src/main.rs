use std::collections::HashMap;
use std::error;
use std::io::{stdin, Read};
use std::result;

type Result<T> = result::Result<T, Box<dyn error::Error>>;

fn main() -> Result<()> {
    let mut s = String::new();
    stdin().read_to_string(&mut s)?;
    let mut graph: Vec<Vec<u8>> = s
        .lines()
        .map(|line| line.as_bytes().iter().map(|b| b - 48).collect())
        .collect();

    let dr = vec![1, -1, 0, 0];
    let dc = vec![0, 0, -1, 1];

    println!("{:?}", graph);
    Ok(())
}
