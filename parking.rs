//abc080_a　parking.rs
###################################
use proconio::*;
fn main() {
    input! { n: i32, a: i32, b: i32 }
    println!("{}", b.min(n * a));
}
###################################
use proconio::input;
fn main() {
	input! {mut a: [i32; 3]}
	print!("{}", i32::min(a[0] * a[1], a[2]));
}
###################################
use itertools::Itertools;
use proconio::{input, marker::*};
use std::cmp::*;
fn main() {
    input! {
        n: i64,
        a: i64,
        b: i64,
    };
    let ans = min(a * n, b);
    println!("{}", ans);
}
###################################
use proconio::{fastout, input};
#[fastout]
fn main() {
    input! {
         n: i32,
         a: i32,
         b: i32,
    }
    println!("{}", std::cmp::min(n * a, b));
}
###################################
use proconio::input;

fn main() {
    input! {
        n: usize,
        a: usize,
        b: usize,
    }
    let a_cost = a * n;
    let b_cost = b;
    println!("{}", a_cost.min(b_cost));
}

###################################
use proconio::input;
fn main() {
    input! {
        n: i32,
        a: i32,
        b: i32,
    };
    let ans = std::cmp::min(a * n, b);
    println!("{}", ans);
}
###################################
