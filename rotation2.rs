//abc077_a　rotation2.rs
###################################
###################################
###################################
fn main() {
    proconio::input! { c:[String;2] }
    println!(
        "{}",
        ["NO", "YES"][c[0].chars().eq(c[1].chars().rev()) as usize]
    );
}
###################################
use proconio::{input, marker::Chars};
fn main() {
    input! { c: [Chars; 2] }
    let mut r = c.clone();
    r[0].reverse();
    r[1].reverse();
    r.reverse();
    println!("{}", if c == r { "YES" } else { "NO" });
}
###################################
use proconio::*;
fn main() {
    input! {c: [marker::Chars; 2]}
    let mut d = c.clone();
    d.swap(0, 1);
    d.iter_mut().for_each(|d| d.reverse());
    if d == c {
        println!("YES")
    } else {
        println!("NO")
    }
}
###################################
use proconio::{input, marker::Chars};
fn main() {
    input! {
        c: [Chars; 2],
    }
    if c[0][0] == c[1][2] && c[0][1] == c[1][1] && c[0][2] == c[1][0] {
        println!("YES");
    } else {
        println!("NO");
    }
}
###################################
use itertools::Itertools;
use proconio::{input, marker::Chars};
fn main() {
    input! {
        c1: Chars,
        c2: Chars,
    }
    if c1
        .into_iter()
        .zip(c2.into_iter().rev())
        .all(|(c1i, c2i)| c1i == c2i)
    {
        println!("YES");
    } else {
        println!("NO");
    }
}
###################################
use proconio::input;
fn main() {
    input! {
        a: String,
        b: String,
    };
    // Check if the reversed string is equal to the original string
    if a.chars().rev().collect::<String>() == b {
        println!("YES");
    } else {
        println!("NO");
    }
}
###################################
