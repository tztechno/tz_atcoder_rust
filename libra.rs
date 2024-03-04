//abc083_a　libra.rs
###################################
#![allow(unreachable_code)]
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        a: u32,
        b: u32,
        c: u32,
        d: u32
    }
    println!(
        "{}",
        if a + b > c + d {
            "Left"
        } else if a + b == c + d {
            "Balanced"
        } else {
            "Right"
        }
    )
}
###################################
use proconio::input;
#[allow(non_snake_case)]
#[allow(unused_variables)]
fn main() {
    input!{ A: u64, B: u64, C: u64, D: u64, };
    println!("{}", if A+B > C+D {"Left"} else if A+B < C+D {"Right"} else {"Balanced"});
}
###################################
use proconio::input;
fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
        d: usize,
    }
    println!(
        "{}",
        if a + b > c + d {
            "Left"
        } else if a + b == c + d {
            "Balanced"
        } else {
            "Right"
        }
    );
}
###################################
use proconio::*;
fn main() {
    input! { a:i64, b:i64, c:i64, d:i64 }
    let D: i64 = a + b - c - d;
    if D > 0 {
        println!("Left");
    } else if D < 0 {
        println!("Right");
    } else {
        println!("Balanced");
    }
}    
###################################
use std::io;
fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let A: Vec<i32> = input
        .split_whitespace()
        .map(|s| s.trim().parse().expect("Invalid input"))
        .collect();
    let D: i32 = A[0] + A[1] - A[2] - A[3];
    if D > 0 {
        println!("Left");
    } else if D < 0 {
        println!("Right");
    } else {
        println!("Balanced");
    }
}
###################################
