//abc091_a.rs
################################
use proconio::{input, marker::*};
let ans;
################################
################################
################################
################################
use proconio::*;

fn main() {
    input! {a: usize, b: usize, c: usize}

    if a + b >= c {
        println!("Yes")
    } else {
        println!("No")
    }
}
################################
use itertools::Itertools;
use proconio::{input, marker::*};
use std::cmp::*;

fn main() {
    input! {
        a: i64,
        b: i64,
        c: i64,
    };
    let ans = if a + b >= c { "Yes" } else { "No" };
    println!("{}", ans);
}

################################
use proconio::*;

fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
    }
    
    if a+b >= c {
        println!("Yes");
    } else {
        println!("No");
    }
}
################################
use proconio::input;

fn main() {
    input! {
        (a, b, c): (usize, usize, usize),
    }

    let ans = a + b >= c;
    println!("{}", if ans { "Yes" } else { "No" });
}

################################
use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
    }

    if a + b >= c {
        println!("Yes");
    } else {
        println!("No");
    }
}
################################
#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(unused_variables)]
use proconio::{fastout, input, marker::Chars};
use std::cmp::{max, min, Reverse};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};
use std::io::{stdin, stdout, BufReader, Write};
#[fastout]
fn main() {
    input!{
        a: usize,
        b: usize,
        c: usize,
    }
    if a+b >= c{
        println!("Yes");
    } else{
        println!("No");
    }
}

################################
use proconio::{input};
fn main() {
    input! {c:[i32;3]}
    if (c[0]+c[1]>=c[2])
        {println!("Yes");}
    else
        {println!("No");}
}
################################
use proconio::input;
fn main() {
    input! {
        c: [i32; 3]
    }
    let ans;
    if c[0] + c[1] >= c[2] {
        ans = "Yes";
    } else {
        ans = "No";
    }
    println!("{}", ans);
}
################################

################################
