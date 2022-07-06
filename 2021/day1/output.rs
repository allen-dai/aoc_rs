use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::path::Path;

fn main(){
    let file = File::open("input.txt").expect("Can not open file");
    let reader = BufReader::new(file);
    let mut line = String::new();
    let nums: Vec<u32> = reader.lines().map(|line| line.unwrap().parse::<u32>().unwrap()).collect();
    let mut sums: Vec<u32> = Vec::new();

    for i in 0..nums.len()-2{
        sums.push(nums[i]+nums[i+1]+nums[i+2]);
    }

    let mut ans: u32 = 0;
    for i in 0..sums.len()-1{
        if sums[i+1] > sums[i]{
            ans+=1;
        }
    }

    println!("{}", ans);
}
