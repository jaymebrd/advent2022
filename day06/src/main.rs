use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = parse_config(&args);

    let contents = fs::read_to_string(config.file_path).expect("should have read this file");

    // let res1 = find_marker(contents, 4);
    let res2 = find_marker(contents, 14);

    // part 1
    // println!("{}", res1);

    println!("{}", res2);
}

fn find_marker(s: String, n:usize) -> usize {
    let mut recent_chars = Vec::new();

    for (i, c) in s.chars().enumerate() {
        recent_chars.push(c);

        if recent_chars.len() > n {
            recent_chars.remove(0);
        }

        if recent_chars.len() == n {
            if recent_chars.iter().all(|&c| recent_chars.iter().filter(|&&d| d == c).count() == 1) {
                return i + 1;
            }
        }
    }
    return 0
}


struct Config {
    file_path: String,
}

fn parse_config(args: &[String]) -> Config {
    let file_path = args[1].clone();

    Config { file_path }
}

