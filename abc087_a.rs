//abc087_a.rs
###################################
###################################
###################################
use proconio::*;
fn main() {
    input! { x: i32, a: i32, b: i32 };
    println!("{}", (x-a)%b);
}
###################################
use proconio::input;
fn main() {
    input! {
        x: usize,
        a: usize,
        b: usize,
    }
    println!("{}", (x - a) % b);
}
###################################
use proconio::input;
use proconio::marker::Bytes;
use proconio::marker::Chars;
fn main() {
    input! {
        mut x: i64,
        a: i64,
        b: i64,
    }
    x -= a;
    while x >= b {
        x -= b;
    }
    println!("{}",x);
}

###################################
use std::io::stdin;
fn main(){
    let a = input();
    let b = input();
    let c = input();

    let buy_after_a = a - b;
    let how_many = buy_after_a / c;
    let balance = buy_after_a - (how_many*c);
    println!("{}", balance);
}

fn input()->isize{
    let mut a = String::new();
    stdin().read_line(&mut a).unwrap();
    return a.trim().parse().unwrap();
}
###################################
use proconio::*;
fn main() {
    input! { x: i32 };
    input! { a: i32 };
    input! { b: i32 };
    let ans:i32=(x-a)%b;
    println!("{}", ans);
}
###################################
use std::io::{self, Read};

fn main() {
  let mut x_str = String::new();
  let mut a_str = String::new();
  let mut b_str = String::new();

  io::stdin().read_line(&mut x_str).expect("Failed to read input");
  io::stdin().read_line(&mut a_str).expect("Failed to read input");
  io::stdin().read_line(&mut b_str).expect("Failed to read input");

  let x: i32 = x_str.trim().parse().expect("Invalid input");
  let a: i32 = a_str.trim().parse().expect("Invalid input");
  let b: i32 = b_str.trim().parse().expect("Invalid input");

  let ans: i32 = (x - a) % b;

  println!("{}", ans);
}
###################################
