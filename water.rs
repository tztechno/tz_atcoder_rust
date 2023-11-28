use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let N: i32 = input.trim().split_whitespace().next().expect("No value entered").parse().expect("Invalid number");

    let m = N / 5;
    let diff = N % 5;

    if diff <= 2 {
        println!("{}", m * 5);
    } else {
        println!("{}", (m + 1) * 5);
    }
}
