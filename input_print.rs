#####################################################################

use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let N: usize = input.trim().parse().expect("Invalid input");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let A: Vec<i32> = input
        .split_whitespace()
        .map(|s| s.parse().expect("Invalid input"))
        .collect();

    println!("{}", N);
    for &num in &A {
        print!("{} ", num);
    }
    println!();
}

#####################################################################

use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let N: usize = input.trim().parse().expect("Invalid input");
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let K: usize = input.trim().parse().expect("Invalid input");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let A: Vec<i32> = input
        .split_whitespace()
        .map(|s| s.parse().expect("Invalid input"))
        .collect();

    println!("{}", N);
    println!("{}", K);
    for &num in &A {
        print!("{} ", num);
    }
    println!();
}

#####################################################################
