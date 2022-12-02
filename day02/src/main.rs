use std::collections::HashMap;
use std::env;
use std::fs;
use std::str::Lines;

macro_rules! collection {
    // map-like
    ($($k:expr => $v:expr),* $(,)?) => {{
        core::convert::From::from([$(($k, $v),)*])
    }};
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = parse_config(&args);

    let contents = fs::read_to_string(config.file_path).expect("should have read this file");

    let lines = contents.lines();
    let (total_score_p1, total_score_p2) = play_rps(lines);

    // part one
    println!("{}", total_score_p1);

    // // part two
    println!("{}", total_score_p2);
}

struct Config {
    file_path: String,
}

fn parse_config(args: &[String]) -> Config {
    let file_path = args[1].clone();

    Config { file_path }
}

fn play_rps(lines: Lines) -> (i32, i32) {
    let mut score_p1 = 0;
    let mut score_p2 = 0;

    // part one
    let game_score: HashMap<_, _> = collection! {"B X" => 0, "B Y" => 3, "B Z" => 6, "A X" => 3, "A Y" => 6, "A Z" => 0, "C X" => 6, "C Y" => 0, "C Z"=> 3};
    let rps_score: HashMap<char, _> = collection! { 'X' => 1, 'Y' => 2, 'Z' => 3 };

    //part two
    let rsp_score: HashMap<_, _> = collection! {"B X" => 1, "B Y" => 2, "B Z" => 3 , "A X" => 3, "A Y" => 1, "A Z" => 2 , "C X" => 2, "C Y" => 3, "C Z"=> 1};
    let wld_score: HashMap<char, _> = collection! { 'X' => 0, 'Y' => 3, 'Z' => 6 };

    for line in lines {
        score_p1 += game_score[line];
        score_p2 += rsp_score[line];

        let rps = line.chars().nth(2).unwrap();
        score_p1 += rps_score[&rps];
        score_p2 += wld_score[&rps]
    }
    return (score_p1, score_p2);
}
