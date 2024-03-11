//abc090_a.rs
###################################
###################################
###################################
###################################
###################################
use proconio::{input, marker::Chars};
fn main() {
    input! {
        c: [Chars; 3],
    }
    println!("{}{}{}", c[0][0], c[1][1], c[2][2]);
}

###################################
use itertools::Itertools;
use proconio::{input, marker::*};
use std::cmp::*;

fn main() {
    input! {
        c: [Chars; 3],
    };
    let ans = format!("{}{}{}", c[0][0], c[1][1], c[2][2]);
    println!("{}", ans);
}
###################################
// https://atcoder.jp/contests/abc090/tasks/abc090_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    for i in 0..3 {
        input! {
            c: String,
        }
        print!("{}", &c[i..i+1]);
    }
    println!();
}
###################################
use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        a: String,
        b: String,
        c: String,
    }
    print!("{}", &a[0..1]);
    print!("{}", &b[1..2]);
    print!("{}", &c[2..3]);
    println!();
}
###################################
use proconio::input;

fn main() {
    input! {
        a: String,
        b: String,
        c: String,
    }

    let mut result = String::new();
    result.push(a.chars().next().unwrap());
    result.push(b.chars().nth(1).unwrap());
    result.push(c.chars().nth(2).unwrap());

    println!("{}", result);
}
###################################
