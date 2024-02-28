//abc076_a　goal.rs
###################################
###################################
###################################
use proconio::*;
fn main() {
    input! { r: i32, g: i32 }
    println!("{}", g * 2 - r);
}
###################################
use itertools::Itertools;
use proconio::{input, marker::*};
use std::cmp::*;
fn main() {
    input! {
        r: i64,
        g: i64,
    };
    let ans = g * 2 - r;
    println!("{}", ans);
}
###################################
use proconio::input;
fn main() {
    input! {
        a: i32,
        b: i32,
    }
    println!("{}", 2 * b - a);
}
###################################
use itertools::Itertools;
use proconio::{input, marker::*};
use std::cmp::*;
fn main() {
    input! {
        r: i64,
        g: i64,
    };
    let ans = 2*g-r;
    println!("{}", ans);
}
###################################
