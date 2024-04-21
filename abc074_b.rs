abc074_b.rs
#######################################
i32などの統一
let mut S: i32 = 0;新規変数定義
リストの受け方xs: [i32; n]
#######################################
#######################################
#######################################
#######################################
#######################################
#######################################
#######################################
#######################################
fn main() {
    proconio::input! {
        n: usize,
        k: i32,
        xs: [i32; n]
    }

    let ans = xs.iter().map(|&x| x.min(k - x) * 2).sum::<i32>();

    println!("{}", ans);
}
#######################################
fn main(){
  proconio::input!{
    n:usize,
    k:usize,
    x:[usize; n],
  }
  let mut ans = 0;
  for i in x{
    if i < k-i{
      ans += i;
    }else{
      ans += k-i;
    }
  }
  ans *= 2;
  println!("{}", ans);
}
#######################################
use proconio::input;

fn main() {
    input! {
        n: usize,
        k: i64,
        x: [i64; n],
    }
    let mut ans = 0i64;

    for xi in x.iter() {
        if (k - xi).abs() > xi.abs() {
            ans += xi.abs()
        } else {
            ans += (k - xi).abs()
        }
    }
    println!("{}", ans * 2);
}

#######################################
use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        x: [usize; n],
    }

    let mut total_length = 0;

    for i in x {
        if i < (k as isize - i as isize).abs() as usize {
            total_length += 2 * i;
        } else {
            total_length += 2 * (k as isize - i as isize).abs() as usize;
        }
    }

    println!("{}", total_length);
}

#######################################
#![allow(unused_imports)]
#![allow(unused_variables)]

use itertools::{iproduct, izip, sorted, Itertools};
use num_bigint::BigUint;
use proconio::{input, marker::{Usize1, Chars}};
use std::{cmp::*, collections::{vec_deque, BTreeSet, BinaryHeap, HashMap}, hash::Hash, ops::Index, vec};
use std::usize;
use superslice::Ext;
use std::collections::{BTreeMap, VecDeque};
use petgraph::graph::{self, UnGraph};


fn main() {
    input! {
        n: usize,
        k: usize,
        x: [usize; n]
    }

    let mut ans = 0;

    for num in x {
        ans += min(num * 2, num.abs_diff(k) * 2);
    }

    println!("{}", ans);
    
}
#######################################
use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let N: i32 = input.trim().parse().expect("Invalid input");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let K: i32 = input.trim().parse().expect("Invalid input");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let A: Vec<i32> = input
        .split_whitespace()
        .map(|s| s.parse().expect("Invalid input"))
        .collect();

    let mut S: i32 = 0;

    for &x in &A {
        S += std::cmp::min(x, K - x) * 2;
    }
    println!("{}", S);
}
#######################################
