use std::collections::{HashMap, HashSet};
use std::io::{stdin, stdout, Read, Write};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
type Grid = HashMap<(usize, usize), i32>;
type Claim = Vec<(usize, HashSet<(usize, usize)>)>;

fn main() -> Result<()> {
    let mut s = String::new();
    stdin().read_to_string(&mut s)?;
    let (grid, claims) = parse_input(&mut s)?;
    part_1(&grid)?;
    part_2(&grid, &claims)?;
    Ok(())
}

fn parse_input(s: &mut String) -> Result<(Grid, Claim)> {
    let mut grid = Grid::new();
    let mut claims = Vec::new();
    for line in s.lines() {
        if line.is_empty() {
            continue;
        }
        let mut s = line.split(" ").collect::<Vec<&str>>();
        let id: usize = (s[0])[1..].parse::<usize>()?;
        s = s[2..].to_vec();
        s[0] = &(s[0])[..s[0].len() - 1];
        let loc_temp = s[0].split(",").collect::<Vec<&str>>();
        let size_temp = s[1].split("x").collect::<Vec<&str>>();
        let location = (loc_temp[0].parse::<usize>()?, loc_temp[1].parse::<usize>()?);
        let width = size_temp[0].parse::<usize>()?;
        let height = size_temp[1].parse::<usize>()?;

        let mut claim = HashSet::new();

        for r in 0..width {
            for c in 0..height {
                let curr_loc = (location.0 + r, location.1 + c);
                *grid.entry(curr_loc).or_insert(0) += 1;
                claim.insert(curr_loc);
            }
        }
        claims.push((id, claim));
    }
    Ok((grid, claims))
}

fn part_1(grid: &Grid) -> Result<()> {
    let count = grid.values().filter(|&&count| count > 1).count();
    writeln!(stdout(), "{}", count)?;
    Ok(())
}

fn part_2(grid: &Grid, claims: &Claim) -> Result<()> {
    for (id, claim) in claims {
        let mut overlap = false;
        for c in claim {
            if grid.get(&c).unwrap() > &1 {
                overlap = true;
                break;
            }
        }
        if !overlap {
            writeln!(stdout(), "{}", id)?;
        }
    }

    Ok(())
}
