//abc070_a　palindromic.rs
###################################
###################################
###################################
###################################
use proconio::input;
fn main() {
	input! {mut a: i32}
	if a / 100 == a % 10 {
		print!("Yes\n");
	} else {
		print!("No\n");
	}
}
###################################
use proconio::{input, marker::Chars};
fn main() {
    input! {
        n:Chars,
    }
    println!("{}", if n[0] == n[2] { "Yes" } else { "No" });
}
###################################
use std::io;

fn main() {
    let mut s = String::new();
    io::stdin().read_line(&mut s).expect("Failed to read line");
    let s = s.trim();    
    if s.chars().next() == s.chars().last() {
        println!("Yes");
    } else {
        println!("No");
    }
}
###################################
use itertools::Itertools;
use proconio::{input, marker::*};
use std::cmp::*;

fn main() {
    input! {
        n: i64,
    };
    let ans = if n / 100 == n % 10 { "Yes" } else { "No" };
    println!("{}", ans);
}

###################################
use proconio::{input, marker::Chars};

fn main() {
    input! {n: Chars}
    let r = n.clone().into_iter().rev();

    if r.zip(n).filter(|(a, b)| a != b).count() == 0 {
        println!("Yes");
    } else {
        println!("No");
    }
}
###################################
use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: Chars,
    }

    let mut rev_n = n.clone();
    rev_n.reverse();

    if n == rev_n {
        println!("Yes");
    } else {
        println!("No");
    }
}

###################################
use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let n: Vec<char> = input.trim().chars().collect();

    let len = n.len();
    let mid = len / 2;
    
    if &n[..mid] == &n[len - mid..].iter().rev().cloned().collect::<Vec<_>>() {
        println!("Yes");
    } else {
        println!("No");
    }
}
###################################
