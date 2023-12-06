use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let N: i32 = input.trim().parse().expect("Invalid input"); // Parse input as an integer

    let result: i32;

    if N <= 125 {
        result = 4;
    } else if N <= 211 {
        result = 6;
    } else {
        result = 8;
    }

    println!("{}", result);
}
