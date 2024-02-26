//abc072_a　sandglass.rs
###################################
use proconio::input;
use proconio::fastout;
use std::cmp::max;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        X: isize,
        t: isize,
    }
    println!("{}", max(0, X - t));
}
###################################
use proconio::*;

fn main() {
    input! { x: i32, t: i32 }
    println!("{}", 0.max(x - t));
}
###################################
use itertools::Itertools;
use proconio::{input, marker::*};
use std::cmp::*;

fn main() {
    input! {
        x: i64,
        t: i64,
    };
    let ans = (x - t).max(0);
    println!("{}", ans);
}

###################################
use proconio::*;
use proconio::{fastout, marker::Chars};
use std::cmp::{max, min};
#[fastout]
fn main() {
    input! {
        x:i64,
        t:i64,
    }
    println!("{}", max(0, x - t));
}
###################################
use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let mut iter = input.split_whitespace();

    let x: i32 = iter.next().unwrap().parse().expect("Failed to parse x");
    let t: i32 = iter.next().unwrap().parse().expect("Failed to parse t");

    if x-t>=0{
        println!("{}", x-t);
    } else {
        println!("0");
    }
}

###################################
