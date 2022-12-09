use std::collections::HashSet;
use std::env;
use std::fs;


fn main() {
    let args: Vec<String> = env::args().collect();
    let config = parse_config(&args);

    let contents = fs::read_to_string(config.file_path).expect("should have read this file");

    // Store the commands in a vector.
    let mut commands = Vec::new();
    for line in contents.lines() {
        commands.push(line);
    }
    // Use a stack to keep track of the current directory and its total size.
    let mut stack = Vec::new();
    stack.push(("/", 0));

    // Use a set to store the directories with a total size of at most 100000.
    let mut set = HashSet::new();

    // Iterate through the commands.
    for command in commands {
        let tokens: Vec<&str> = command.split_whitespace().collect();
        let op = tokens[0];

        match op {
            "cd" => {
                let dir = tokens[1];
                if dir == ".." {
                    stack.pop();
                } else {
                    let (cur_dir, cur_size) = stack.last().unwrap();
                    stack.push((dir, *cur_size));
                }
            }
            "ls" => {
                let (_, mut cur_size) = stack.last().unwrap();
                let size = tokens[1].parse::<u32>().unwrap();
                let new_size = cur_size + size;
                if new_size <= 100000 {
                    let (mut cur_dir, _) = stack.last().unwrap();
                    set.insert(cur_dir);
                    let (_cur_dir, _) = stack.last_mut().unwrap();
                    cur_size = new_size;
                }
            }
            _ => {}
        }
    }

    // Sum up the total sizes of the directories in the set.
    let mut sum = 0;
    for dir in set {
        let (_, size) = stack.iter().find(|(d, _)| d == dir).unwrap();
        sum += size;
    }

    println!("The sum of the total sizes of the directories is {}", sum);
}

struct Config {
    file_path: String,
}

fn parse_config(args: &[String]) -> Config {
    let file_path = args[1].clone();

    Config { file_path }
}
