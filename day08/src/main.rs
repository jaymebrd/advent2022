use std::collections::HashSet;
use std::env;
use std::fs;


fn main() {
    let args: Vec<String> = env::args().collect();
    let config = parse_config(&args);

    let contents = fs::read_to_string(config.file_path).expect("should have read this file");


}

struct Config {
    file_path: String,
}

fn parse_config(args: &[String]) -> Config {
    let file_path = args[1].clone();

    Config { file_path }
}
