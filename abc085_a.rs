//abc085_a.rs
###################################
###################################
###################################
###################################
use proconio::{fastout, input};
#[fastout]
fn main() {
    input! {
        s: String,
    }
    println!("2018{}", &s[4..]);
}
###################################
use proconio::{input, marker::Chars};
fn main() {
    input! {
        mut s: Chars,
    }
    s[3] = '8';
    let string_from_s: String = s.iter().collect();
    println!("{}", string_from_s);
}
###################################
use std::io::stdin;
fn main(){
    let mut a = input();
    a.replace_range(3..4, "8");
    println!("{}", a)
}
fn input()->String{
    let mut a = String::new();
    stdin().read_line(&mut a).unwrap();
    return a.trim().parse().unwrap();
}
###################################
use std::io;
fn main() {
    let mut s = String::new();
    io::stdin().read_line(&mut s).expect("Failed to read line");
    let s = s.trim();
    println!("2018{}", &s[4..]);
}
###################################
