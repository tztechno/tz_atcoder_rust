//abc079_a　good_integer.rs
###################################
use itertools::Itertools;
use proconio::{input, marker::*};
use std::cmp::*;
fn main() {
    input! {
        n: Chars
    };
    let ans = if n.windows(3).any(|x| x[0] == x[1] && x[0] == x[2]) {
        "Yes"
    } else {
        "No"
    };
    println!("{}", ans);
}
###################################
use proconio::{input, marker::Chars};
fn main() {
    input! {
        s: Chars,
    };
    let mut is_ok = true;
    for i in 1..3 {
        if s[0] != s[i] {
            is_ok = false;
        }
    }
    for i in 2..4 {
        if s[1] != s[i] && !is_ok {
            println!("No");
            return;
        }
    }
    println!("Yes");
}
###################################
use proconio::fastout;
use proconio::input;
use proconio::marker::Chars;
#[fastout]
fn main() {
    input! {
        n: Chars,
    }
    let is_good = n[0] == n[1] && n[1] == n[2] || n[1] == n[2] && n[2] == n[3];
    println!("{}", if is_good { "Yes" } else { "No" })
}
###################################
use proconio::{input, marker::Chars};
#[proconio::fastout]
fn main() {
    input! {
        n:Chars
    }
    println!(
        "{}",
        if (n[1] == n[2]) && (n[0] == n[1] || n[1] == n[3]) {
            "Yes"
        } else {
            "No"
        }
    );
}
###################################
use proconio::*;
fn main() {
    input! { s: String }
    let chars: Vec<char> = s.chars().collect(); 
    if ((chars[0]==chars[1] && chars[1]==chars[2]) || (chars[1]==chars[2] && chars[2]==chars[3])) 
    {
     println!("Yes");       
    } else {
    println!("No");        
    }
}
###################################
