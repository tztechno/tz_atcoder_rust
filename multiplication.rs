//abc169_a multiplication.rs
################################
################################
################################
################################
use proconio::*;
fn main() {
    input! { a: i64, b: i64 }
    println!("{}", a * b);
}
################################
fn main() {
    proconio::input! {
        a: i32,
        b: i32,
    }
    println!("{}", a * b);
}
################################
use proconio::input;
fn main() {
    input! {
        (a, b): (usize, usize),
    }
    println!("{}", a * b);
}
################################
use proconio::input;
fn main() {
    input! {
        a: usize,
        b: usize
    }
    println!("{}", a * b);
}
################################
use proconio::input;
use proconio::fastout;
#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        (A, B): (usize, usize),
    }
    println!("{}", A * B);
}
################################
fn main() {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    let mut arr = s.split_whitespace();
    let a = arr.next().unwrap().parse::<i32>().unwrap();
    let b = arr.next().unwrap().parse::<i32>().unwrap();
    println!("{}", a * b);
}
################################
use std::io;
fn main() {
    let mut input = String::new();
    if let Err(err) = io::stdin().read_line(&mut input) {
        eprintln!("Error reading input: {}", err);
        return;
    }
    let mut iter = input.split_whitespace();
    let A: Result<i32, _> = iter.next().unwrap().parse();
    let B: Result<i32, _> = iter.next().unwrap().parse();
    match (A, B) {
        (Ok(a), Ok(b)) => {
            println!("{}", a * b);
        }
        (Err(err), _) | (_, Err(err)) => {
            eprintln!("Error parsing input: {}", err);
        }
    }
}
################################
use std::io;
fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let mut iter = input.split_whitespace();
    let A: i32 = iter.next().unwrap().parse().expect("Invalid input");
    let B: i32 = iter.next().unwrap().parse().expect("Invalid input");
    println!("{}", A*B);
    }
################################
