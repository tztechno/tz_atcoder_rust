//abc075_a　three.rs
###################################
###################################
###################################
###################################
use proconio::{input, marker::Chars};

pub fn main() {
    input !{
        a: i32,
        b: i32,
        c: i32
    };
    println!("{}", a ^ b ^ c);
}
###################################
use proconio::*;
fn main()
{
	input!{mut a: [i8; 3]}
	a.sort();
	println!("{}", if a[0] == a[1] {a[2]} else {a[0]});
}
###################################
use proconio::input;
fn main() {
    input! {
        a: isize,
        b: isize,
        c: isize,
    }
    println!(
        "{}",
        if a == b {
            c
        } else if b == c {
            a
        } else {
            b
        }
    );
}
###################################
use std::collections::HashSet;
fn main() {
    proconio::input! {
        a: isize,
        b: isize,
        c: isize
    }
    let s0: isize = a + b + c;
    let s: isize = [a, b, c].iter().cloned().collect::<HashSet<_>>().iter().sum();
    println!("{}", s * 2 - s0);
}
###################################
