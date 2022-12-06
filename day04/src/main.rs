use std::env;
use std::fs;
use std::str::Lines;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = parse_config(&args);

    let contents = fs::read_to_string(config.file_path).expect("should have read this file");

    let lines = contents.lines();

    let (res_p1, res_p2) = find_overlaps(lines);

    // part 1
    println!("{}", res_p1);

    println!("-------");

    // part 2
    println!("{}", res_p2);
}

fn find_overlaps(lines: Lines) -> (i32, i32) {
    let mut full_count = 0;
    let mut any_count = 0;
    let mut interval_ranges: Vec<Vec<i32>> = Vec::new();

    // construct the input
    for line in lines {
        let intervals = line.split(",");

        for interval in intervals {
            let vec: Vec<i32> = interval
                .split("-") // Split the string by the "-" character
                .map(|s| s.parse().unwrap()) // Convert each string to an i32
                .collect(); // Collect the results into a vector
            interval_ranges.push(vec);
        }
    }

    for chunk in interval_ranges.chunks(2) {
        if chunk[0][0] >= chunk[1][0] && chunk[0][1] <= chunk[1][1]
            || chunk[1][0] >= chunk[0][0] && chunk[1][1] <= chunk[0][1]
        {
            full_count += 1
        }

        if (chunk[0][0] >= chunk[1][0] && chunk[0][0] <= chunk[1][1]) || (chunk[0][1] >= chunk[1][1] && chunk[1][0] <= chunk[1][1])
        {
            any_count +=1
        }
    }
    return (full_count, any_count);
}

struct Config {
    file_path: String,
}

fn parse_config(args: &[String]) -> Config {
    let file_path = args[1].clone();

    Config { file_path }
}
