//abc071_a　delivery.rs
###################################
###################################
###################################
use proconio::*;

fn main() {
    input! { x: i32, a: i32, b: i32 }
    println!(
        "{}",
        if (a - x).abs() > (b - x).abs() {
            "B"
        } else {
            "A"
        }
    )
}
###################################
use proconio::input;

fn main() {
    input! {
        x: isize,
        a: isize,
        b: isize,
    }
    println!(
        "{}",
        if (x - a).abs() < (x - b).abs() {
            "A"
        } else {
            "B"
        }
    );
}
###################################
use proconio::input;

fn main() {
    input!{
        X: [i64; 3],
    }
    let y = X[0];
    let a = X[1];
    let b = X[2];
    if (y-a).abs() < (y-b).abs() {
        println!("A");
    } else {
        println!("B");
    }
}
###################################
use proconio::input;

fn main() {
    input!{
        X: [i64; 3],
    }
    let y = X[0];
    let a = X[1];
    let b = X[2];
    if (y as i64 - a as i64).abs() < (y as i64 - b as i64).abs() {
        println!("A");
    } else {
        println!("B");
    }
}
###################################
