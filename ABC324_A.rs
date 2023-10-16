use std::collections::HashSet;
use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let n: usize = input.trim().parse().expect("Invalid input");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let a: Vec<i32> = input
        .split_whitespace()
        .map(|x| x.parse().expect("Invalid input"))
        .collect();

    let unique_elements: HashSet<i32> = a.iter().cloned().collect();

    if unique_elements.len() == 1 {
        println!("Yes");
    } else {
        println!("No");
    }
}
