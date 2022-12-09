use std::collections::HashSet;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = parse_config(&args);

    let contents = fs::read_to_string(config.file_path).expect("should have read this file");

    let knots = 10;
    let mut kpos = vec![(0, 0); knots];
    let dmap = [("R", (1, 0)), ("L", (-1, 0)), ("D", (0, -1)), ("U", (0, 1))];

    let mut p1 = HashSet::new();
    let mut p2 = HashSet::new();
    p1.insert((0, 0));
    p2.insert((0, 0));

    let mut hx = 0;
    let mut hy = 0;

    for row in contents.split("\n") {
        let d_split: Vec<&str> = row.split(" ").collect();
        let d = d_split[0];
        let n_split: Vec<&str> = row.split(" ").collect();
        let n = n_split[1].parse().unwrap();
        let (dx, dy) = dmap.iter().find(|(k, _)| *k == d).unwrap().1;
        for _ in 0..n {
            hx += dx;
            hy += dy;
            kpos[0] = (hx, hy);

            for k in 1..knots {
                kpos[k] = tmove(kpos[k - 1], kpos[k]);
                if kpos[k] == kpos[k - 1] {
                    break;
                }
                p1.insert(kpos[1]);
                p2.insert(kpos[kpos.len() - 1]);
            }
        }
    }
    println!("{}", p1.len());
    println!("{}", p2.len());
}

fn tmove(hpos: (i32, i32), tpos: (i32, i32)) -> (i32, i32) {
    let (hx, hy) = hpos;
    let (tx, ty) = tpos;
    if (hx - tx).abs() <= 1 && (hy - ty).abs() <= 1 {
        return tpos;
    }

    let moves = [
        (0, 1),
        (0, -1),
        (1, 0),
        (-1, 0),
        (1, 1),
        (1, -1),
        (-1, 1),
        (-1, -1),
    ];
    let mut best_move = None;
    let mut best_dist = std::i32::MAX;
    for (dx, dy) in moves {
        let tx_ = tx + dx;
        let ty_ = ty + dy;
        let dist = (hx - tx_).abs() + (hy - ty_).abs();
        if dist < best_dist {
            best_dist = dist + (dx.abs() + dy.abs());
            best_move = Some((dx, dy));
        }
    }
    let (dx, dy) = best_move.unwrap();
    (tx + dx, ty + dy)
}

struct Config {
    file_path: String,
}

fn parse_config(args: &[String]) -> Config {
    let file_path = args[1].clone();

    Config { file_path }
}
