use std::io;
use std::convert::TryInto;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let numbers: Vec<i32> = input
        .split_whitespace()
        .map(|s| s.parse().expect("Invalid input"))
        .collect();

    let a = numbers[0];
    let b = numbers[1];

    println!("{}", 32_i32.pow((a - b).try_into().unwrap()));
}
