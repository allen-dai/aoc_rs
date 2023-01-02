use regex::Regex;
use std::collections::VecDeque;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, Read};

pub struct Monkey {
    ops: Vec<OP>,
    items: VecDeque<usize>,
    id: usize,
    d: usize,
}

impl Monkey {
    pub fn run(&mut self) -> Vec<(usize, usize)> {
        // println!("--- Monkey {}:", self.id);
        let mut ret = Vec::new();
        while !self.items.is_empty() {
            let n = self.items.pop_front().unwrap();
            let mut new = 0;
            for op in self.ops.iter() {
                match op {
                    OP::Inspect { operand, operator } => {
                        let mut r = n;
                        if operand != "old" {
                            r = operand.parse().unwrap();
                        }
                        match operator.as_str() {
                            "*" => {
                                // print!("{} * {}", n, r);
                                new = n * r;
                            }
                            "/" => {
                                // print!("{} / {}", n, r);
                                new = n / r;
                            }
                            "+" => {
                                // print!("{} + {}", n, r);
                                new = n + r;
                            }
                            _ => (),
                        }
                    }
                    OP::Throw { test } => {
                        let id = test(new);
                        // print!(" -> {id}\n");
                        ret.push((id, new));
                    }
                }
            }
        }
        ret
    }
}

pub enum OP {
    Inspect { operand: String, operator: String },
    Throw { test: Box<dyn Fn(usize) -> usize> },
}

fn main() {
    let mut path = "in".to_string();
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        path = args[1].to_string();
    }
    let mut f = File::open(&path).unwrap();
    let mut lines: Vec<String> = BufReader::new(f)
        .lines()
        .map(|r| r.unwrap().trim().to_string())
        .collect();
    let monkeys_s: Vec<Vec<String>> = lines.split(|c| c.is_empty()).map(|r| r.into()).collect();
    let mut monkeys = Vec::new();
    let num_re = Regex::new(r"\d+").unwrap();
    let op_re = Regex::new(r"(\S+) (\+|-|\*|/) (\S+)").unwrap();
    let mut lcm = 1;
    for monkey in monkeys_s.iter() {
        if monkey.len() <= 0 {
            continue;
        }

        let id = num_re
            .find(&monkey[0])
            .unwrap()
            .as_str()
            .parse::<usize>()
            .unwrap();

        let items: VecDeque<usize> = num_re
            .find_iter(&monkey[1])
            .map(|c| c.as_str().parse::<usize>().unwrap())
            .collect();

        let op_cap = op_re.captures_iter(&monkey[2]).next().unwrap();
        let (operator, operand) = (op_cap[2].to_string(), op_cap[3].to_string());

        let divisor = num_re
            .find(&monkey[3])
            .unwrap()
            .as_str()
            .parse::<usize>()
            .unwrap();

        let if_true = num_re
            .find(&monkey[4])
            .unwrap()
            .as_str()
            .parse::<usize>()
            .unwrap();

        let if_false = num_re
            .find(&monkey[5])
            .unwrap()
            .as_str()
            .parse::<usize>()
            .unwrap();

        let divisible = move |new: usize| -> usize {
            if new % divisor == 0 {
                if_true
            } else {
                if_false
            }
        };

        let inspect = OP::Inspect { operator, operand };
        let throw = OP::Throw {
            test: Box::new(divisible),
        };
        let ops = vec![inspect, throw];

        let monkey = Monkey {
            ops,
            id,
            items,
            d: divisor,
        };
        monkeys.push(monkey);
        lcm *= divisor
    }

    let mut ret = vec![0; monkeys.len()];

    let ROUND: usize = 10000;
    for _ in 0..ROUND {
        for i in 0..monkeys.len() {
            ret[i] += monkeys[i].items.len();
            let res = monkeys[i].run();
            for (id, val) in res {
                monkeys[id].items.push_back(val % lcm);
            }
        }
    }
    // println!();
    for _ in 0..ROUND {
        for monkey in monkeys.iter() {
            // println!("{:?}", monkey.items);
        }
    }
    ret.sort();
    println!(
        "{:?} {}",
        ret.clone(),
        ret.pop().unwrap() * ret.pop().unwrap()
    );
}
