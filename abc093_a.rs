//abc093_a.rs
####################################
let s_chars: Vec<char> = s.chars().collect();
####################################
####################################
####################################
####################################
####################################
####################################
use proconio::*;

fn main() {
    input! {mut s: marker::Chars}
    s.sort();

    if s == vec!['a', 'b', 'c'] {
        println!("Yes")
    } else {
        println!("No")
    }
}
####################################
use itertools::Itertools;
#[allow(unused)]
use proconio::{
    *,
    marker::Usize1,
};

fn main() {
    input! {
        mut s: marker::Chars,
    }
    s.sort();
    let ans = if s.iter().collect::<String>() == "abc" { "Yes" } else { "No" };
    println!("{ans}");
}
####################################
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        s: String,
    }
    let s_chars: Vec<char> = s.chars().collect();
    if s_chars.contains(&'a') && s_chars.contains(&'b') && s_chars.contains(&'c') {
        println!("Yes");
    } else {
        println!("No");
    }
}
####################################
