use std::collections::HashMap;
use std::env;
use std::fs;
use std::str::Lines;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = parse_config(&args);

    let contents = fs::read_to_string(config.file_path).expect("should have read this file");

    let lines = contents.lines();

    let duplicates = get_duplicates(lines);
    let groups = get_groupitems(contents);

    //part 1
    let res_p1 = find_scores(duplicates);
    print!("part one");
    print!("{}", res_p1);

    // part 2
    let res_p2 = find_scores(groups);
    print!("part two");
    print!("{}", res_p2);
}

struct Config {
    file_path: String,
}

fn parse_config(args: &[String]) -> Config {
    let file_path = args[1].clone();

    Config { file_path }
}

fn get_groupitems(lines: String) -> Vec<char> {
    let lines: Vec<&str> = lines.lines().collect();
    let mut duplicates = Vec::new();
    let cmp: i32 = 3;

    for line in lines.chunks(3) {
        let mut map: HashMap<char, i32> = HashMap::new();
        for l in line {
            let mut line_map: HashMap<char, i32> = HashMap::new();
            for chr in l.chars() {
                line_map.insert(chr, 1);
            }
            for (k, _v) in &line_map {
                match map.get(&k) {
                    Some(count) => {
                        map.insert(*k, count + 1);
                    }
                    None => {
                        map.insert(*k, 1);
                    }
                }
            }
        }
        for (k, v) in &map {
            if v == &cmp {
                duplicates.push(*k)
            }
        }
    }
    return duplicates;
}

fn find_scores(slice: Vec<char>) -> u32 {
    let mut score: u32 = 0;
    const OFFSET_UPPERCASE: u32 = 'A' as u32;
    const OFFSET_LOWERCASE: u32 = 'a' as u32;

    for c in slice {
        if c.is_uppercase() {
            let mut ch_ord = c as u32;
            ch_ord = (ch_ord - OFFSET_UPPERCASE) + 27;
            score += ch_ord
        } else {
            let mut ch_ord = c as u32;
            ch_ord = (ch_ord - OFFSET_LOWERCASE) + 1;
            score += ch_ord
        }
    }

    return score;
}

fn get_duplicates(lines: Lines) -> Vec<char> {
    let mut duplicates = Vec::new();
    for line in lines {
        let mut map: HashMap<char, bool> = HashMap::new();
        let mut n = Vec::new();
        let sack_size = line.chars().count() / 2;
        let mut count = 0;

        for c in line.chars() {
            if count < sack_size {
                map.insert(c, true);
                count += 1
            } else {
                // second compartment
                if map.contains_key(&c) {
                    if !n.contains(&c) {
                        n.push(c)
                    }
                }
            }
        }
        duplicates.append(&mut n);
    }
    return duplicates;
}
