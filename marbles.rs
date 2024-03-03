//abc081_a　marbles.rs
###################################
###################################
use proconio::{input, marker::Chars};

#[allow(non_snake_case)]
#[allow(unused_variables)]
fn main() {
    input! {s:Chars,};
    let mut res = 0;
    for &c in &s { if c == '1' { res += 1; }}
    println!("{}", res);
}
###################################
use proconio::input;
fn main() {
    input! {
        s: usize,
    }
    println!("{}", s / 100 + s % 100 / 10 + s % 10);
}
###################################
use proconio::input;
fn main() {
  input!{
    target: String,
  }  
  let filterd: Vec<char> = target.chars().filter(|&x| x == '1').collect();  
  println!("{}", filterd.len())
}
###################################
use proconio::{input, marker::Chars};
fn main() {
    input! {
        s: Chars,
    };
    let t = s.iter().filter(|&c| *c == '1').count();
    println!("{}", t);
}
###################################
use proconio::{input, marker::Chars};
fn main() {
    input! {
        s: Chars,
    };
    let mut t = 0;  // Make t mutable
    for i in 0..3 {  // Fix the loop range
        if s[i] == '1' {
            t = t + 1;
        }
    }
    println!("{}", t);
}
###################################
