//abc074_a　bichrome.rs
###################################
###################################
use itertools::Itertools;
use proconio::{input, marker::*};
use std::cmp::*;
fn main() {
    input! {
        n: i64,
        a: i64,
    };
    let ans = n.pow(2) - a;
    println!("{}", ans);
}
###################################
fn main(){
  proconio::input!{
    a:usize,
    b:usize,
  }
  println!("{}",a*a-b)
}
###################################
use proconio::input;
fn main() {
    input! {
        n: usize,
        a: usize,
    }
    println!("{}", n.pow(2) - a);
}
###################################
use proconio::input;
fn main() {
    input!{
        n: u64,
        a: u64,
        }
    println!("{}", n*n-a);
}
###################################
