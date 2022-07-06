use std::collections::HashMap;
use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() {
    let file = File::open("./src/in.2").expect("");
    let reader = BufReader::new(file);

    let mut inputs: Vec<String> = Vec::new();
    let mut map: HashMap<String, i64> = HashMap::new();
    let mut reverse_map: HashMap<i64, String> = HashMap::new();

    let mut ans = 0;
    for l in reader.lines() {
        let line: Vec<String> = l.unwrap().split(" | ").map(|s| s.to_string()).collect();

        let mut signal_pattern: Vec<String> = line[0]
            .split(" ")
            .map(|s| sort_word(&s.to_string()))
            .collect();

        let mut output_pattern: Vec<String> = line[1]
            .split(" ")
            .map(|s| sort_word(&s.to_string()))
            .collect();

        let mut seg5: Vec<String> = Vec::new();
        let mut seg6: Vec<String> = Vec::new();

        for o in signal_pattern.iter() {
            // 1
            if o.len() == 2 {
                map.insert(o.to_string(), 1);
                reverse_map.insert(1, o.to_string());
            }
            // 7
            else if o.len() == 3 {
                map.insert(o.to_string(), 7);
                reverse_map.insert(7, o.to_string());
            }
            // 4
            else if o.len() == 4 {
                map.insert(o.to_string(), 4);
                reverse_map.insert(4, o.to_string());
            }
            // Number that has 5 segments -- Num 6, 9, 0
            else if o.len() == 5 {
                seg5.push(o.to_string());
            }
            // 8
            else if o.len() == 7 {
                map.insert(o.to_string(), 8);
                reverse_map.insert(8, o.to_string());
            }
            // Number that has 6 segments -- Num 5, 2, 3
            else {
                seg6.push(o.to_string());
            }
        }

        //println!("{:?} \n {:?}", seg5, seg6);

        // Num 6
        for (i, x) in seg6.clone().iter().enumerate() {
            let string_one = reverse_map.get(&1).unwrap();
            if !contains(x, string_one) {
                map.insert(x.to_string(), 6);
                reverse_map.insert(6, x.to_string());
                seg6.remove(i);
                break;
            }
        }

        //Num 9
        for (i, x) in seg6.clone().iter().enumerate() {
            let string_four = reverse_map.get(&4).unwrap();
            if contains(x, string_four) {
                map.insert(x.to_string(), 9);
                reverse_map.insert(9, x.to_string());
                seg6.remove(i);
                break;
            }
        }

        //Num 0
        map.insert(seg6[0].to_string(), 0);
        reverse_map.insert(0, seg6[0].to_string());

        //Num 3
        for (i, x) in seg5.clone().iter().enumerate() {
            let string_one = reverse_map.get(&1).unwrap();
            if contains(x, string_one) {
                map.insert(x.to_string(), 3);
                reverse_map.insert(3, x.to_string());
                seg5.remove(i);
                break;
            }
        }

        //Num 5
        for (i, x) in seg5.clone().iter().enumerate() {
            let string_one = reverse_map.get(&1).unwrap();
            let string_nine = reverse_map.get(&9).unwrap();
            if fine_nine_check(string_one, x, string_nine) {
                map.insert(x.to_string(), 5);
                reverse_map.insert(5, x.to_string());
                seg5.remove(i);
                break;
            }
        }

        //Num 2
        map.insert(seg5[0].to_string(), 2);
        reverse_map.insert(2, seg5[0].to_string());

        println!("{:?}", map);

        let mut ans_string: String = String::new();

        for (i, o) in output_pattern.iter().enumerate() {
            let n = *map.get(o).unwrap() as i64;
            ans_string += &n.to_string();
        }

        println!("{}", ans_string);

        ans += ans_string.parse::<i64>().unwrap();

        map.clear();
        reverse_map.clear();
    }

    println!("{:?}", ans);
}

fn get_digit(s: &str) -> i64 {
    match s {
        "abcefg" => return 0,
        "cf" => return 1,
        "acdeg" => return 2,
        "acdfg" => return 3,
        "bcdf" => return 4,
        "abdfg" => return 5,
        "abdefg" => return 6,
        "acf" => return 7,
        "abcdefg" => return 8,
        "abcdfg" => return 9,
        _ => return -1,
    }
}

fn sort_word(word: &str) -> String {
    let mut cvec = word.chars().collect::<Vec<_>>();
    cvec.sort_unstable();
    cvec.iter().collect()
}

fn contains(string: &str, substring: &str) -> bool {
    let mut s1 = string.chars().collect::<Vec<_>>();
    let mut s2 = substring.chars().collect::<Vec<_>>();

    for s in s2 {
        if !s1.contains(&s) {
            return false;
        }
    }

    return true;
}

fn fine_nine_check(string_1: &str, string_5: &str, string_9: &str) -> bool {
    let mut s1 = string_1.chars().collect::<Vec<_>>();
    let mut s5 = string_5.chars().collect::<Vec<_>>();
    let mut s9 = string_9.chars().collect::<Vec<_>>();

    s5.extend(s1);

    for s in s9 {
        if !s5.contains(&s) {
            return false;
        }
    }
    return true;
}
