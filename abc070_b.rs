abc070_b.rs
##############################
let mut t: i32 = 0; // Added mut keyword to make t mutable
##############################
##############################
##############################
##############################
use proconio::input;

fn main() {
    input! {
        (a, b, c, d): (usize, usize, usize, usize),
    }

    let mut alice = vec![false; 100];
    for i in a..b {
        alice[i] = true;
    }

    let ans = (c..d).filter(|&i| alice[i]).count();
    println!("{}", ans);
}

##############################
use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
        d: usize,
    }

    if a.max(c) <= b.min(d) {
        println!("{}", b.min(d) - a.max(c));
    } else {
        println!("0");
    }
}

##############################
use itertools::Itertools;
use proconio::input;
fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
        d: usize
    }
    let e = a.max(c);
    let f = b.min(d);

    println!("{}", f.saturating_sub(e));
}

##############################
use proconio::input;

fn main() {
    input! { a: i32, b: i32, c: i32, d: i32  };    
    let mut t: i32 = 0; // Added mut keyword to make t mutable
    for i in a..b {
        if c <= i && i < d { // Added missing curly braces and fixed condition
            t = t + 1; // Added semicolon at the end of the statement
        }
    }
    println!("{}", t);
}
##############################
