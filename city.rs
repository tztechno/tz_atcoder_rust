//abc069_a　city.rs
###################################
###################################
i32,i64,usize
use proconio::*;        
use proconio::input;       
use proconio::{input, marker::*};        
###################################
use itertools::Itertools;
use proconio::{input, marker::*};
use std::cmp::*;
fn main() {
    input! {
        n: i64,
        m: i64,
    };
    let ans = (n - 1) * (m - 1);
    println!("{}", ans);
}
###################################
use proconio::input;
fn main() {
    input! {
        n: usize,
        m: usize,
    }
    println!("{}", (n - 1) * (m - 1));
}
###################################
fn main() {
    proconio::input! {
        n: i32,
        m: i32,
    }
    println!("{}", (n-1)*(m-1));
}
###################################
use proconio::*;
fn main() {
    input! { n: i64, m: i64 }
    println!("{}", (n-1)*(m-1));
}
###################################
