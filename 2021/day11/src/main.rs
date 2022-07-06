use std::io::{stdin, stdout, Read, Write};
use std::{error, result};

type Result<T> = result::Result<T, Box<dyn error::Error>>;

fn main() -> Result<()> {
    let mut s = String::new();
    stdin().read_to_string(&mut s)?;
    let mut oct = Vec::new();
    for line in s.lines() {
        let mut temp = Vec::new();
        if line.is_empty() {
            continue;
        }
        for c in line.chars() {
            temp.push(c.to_digit(10).unwrap() as i32);
        }
        oct.push(temp.clone());
    }
    part_1(&mut oct, 100)?;
    part_2(&mut oct)?;
    println!("{:?}", oct);
    Ok(())
}

fn flash(ans: &mut i32, oct: &mut Vec<Vec<i32>>, r: i32, c: i32) {
    *ans += 1;
    oct[r as usize][c as usize] = -1;
    for dr in [-1, 0, 1] {
        for dc in [-1, 0, 1] {
            let rr = r + dr;
            let cc = c + dc;
            if 0 <= rr
                && rr < oct.len() as i32
                && 0 <= cc
                && cc < oct[0].len() as i32
                && oct[rr as usize][cc as usize] != -1
            {
                let rr = rr as usize;
                let cc = cc as usize;
                oct[rr][cc] += 1;
                if oct[rr][cc] >= 10 {
                    flash(ans, oct, rr as i32, cc as i32);
                }
            }
        }
    }
}

fn all_zeros(oct: &Vec<Vec<i32>>) -> bool {
    for v in oct {
        for n in v {
            if *n != 0 {
                return false;
            }
        }
    }
    true
}

fn part_1(oct: &mut Vec<Vec<i32>>, step: usize) -> Result<()> {
    let row_len = oct.len();
    let col_len = oct[0].len();
    let ans = &mut 0;
    for _ in 0..step {
        for r in 0..row_len {
            for c in 0..col_len {
                oct[r][c] += 1;
            }
        }

        for r in 0..row_len {
            for c in 0..col_len {
                if oct[r][c] == 10 {
                    flash(ans, oct, r as i32, c as i32);
                }
            }
        }

        for r in 0..row_len {
            for c in 0..col_len {
                if oct[r][c] == -1 {
                    oct[r][c] = 0;
                }
            }
        }
    }
    println!("{}", ans);
    Ok(())
}

fn part_2(oct: &mut Vec<Vec<i32>>) -> Result<()> {
    let row_len = oct.len();
    let col_len = oct[0].len();
    let ans = &mut 0;
    let mut step = 0;
    while !all_zeros(&oct){
        step += 1;
        for r in 0..row_len {
            for c in 0..col_len {
                oct[r][c] += 1;
            }
        }

        for r in 0..row_len {
            for c in 0..col_len {
                if oct[r][c] == 10 {
                    flash(ans, oct, r as i32, c as i32);
                }
            }
        }

        for r in 0..row_len {
            for c in 0..col_len {
                if oct[r][c] == -1 {
                    oct[r][c] = 0;
                }
            }
        }
    }
    println!("{}", step);
    Ok(())
}
