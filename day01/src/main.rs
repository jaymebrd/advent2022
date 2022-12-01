use std::collections::BinaryHeap;
use std::env;
use std::fs;
use std::str::Lines;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = parse_config(&args);

    let contents = fs::read_to_string(config.file_path).expect("should have read this file");

    let lines = contents.lines();
    let mut heapify_calories = max_calories(lines);

    // Part 1
    println!("{}", heapify_calories.peek().unwrap());

    // Part 2, top three
    let mut total = 0;
    for _n in 1..4 {
        total += heapify_calories.pop().unwrap();
    }
    println!("{}", total);
}

struct Config {
    file_path: String,
}

fn parse_config(args: &[String]) -> Config {
    let file_path = args[1].clone();

    Config { file_path }
}

fn max_calories(lines: Lines) -> BinaryHeap<i32> {
    let mut max_heap = BinaryHeap::new();
    let mut tmp = 0;

    for val in lines {
        if val == "" {
            max_heap.push(tmp);
            tmp = 0
        } else {
            let val: i32 = val
                .trim()
                .parse()
                .expect("please give me the correct string number!");
            tmp += val
        }
    }
    return max_heap;
}
