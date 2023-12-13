use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let file_path = if cfg!(target_os = "linux") {
        "/dev/stdin"
    } else {
        "./input.txt"
    };

    let file = File::open(file_path).expect("Failed to open file");
    let reader = io::BufReader::new(file);
    let input = reader.lines().next().expect("Failed to read line").unwrap();

    let values: Vec<i32> = input.split_whitespace().map(|s| s.parse().expect("Failed to parse")).collect();
    let (V, T, S, D) = (values[0], values[1], values[2], values[3]);

    if V * T <= D && D <= V * S {
        println!("No");
    } else {
        println!("Yes");
    }
}
