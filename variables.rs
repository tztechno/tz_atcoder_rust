//abc170_a variables.rs
#####################################
#####################################
use proconio::input;

fn main() {
    input! {
        x: [i32; 5]
    }
    let sum: i32 = x.iter().sum();
    println!("{}", 15 - sum);
}
#####################################
fn main() {
    proconio::input! {
        x1: i32,
        x2: i32,
        x3: i32,
        x4: i32,
        x5: i32,
    }
    println!("{}", 15 - (x1 + x2 + x3 + x4 + x5));
}
#####################################
use proconio::input;

fn main() {
  input! {
    xs: [i32; 5],
  };
  
  let mut ans: i32 = 0;
  for (i, x) in xs.iter().enumerate() {
    if x == &0 {
      ans = (i + 1) as i32;
      break;
    }
  }
  
  println!("{}", ans);
#####################################
use proconio::input;

fn main() {
    input! {
        x1: u8,
        x2: u8,
        x3: u8,
        x4: u8,
        x5: u8,
    }
    println!("{}",
        if x1 == 0 { 1 }
        else if x2 == 0 { 2 }
        else if x3 == 0 { 3 }
        else if x4 == 0 { 4 }
        else { 5 }
    );
}
#####################################
use proconio::input;

fn main() {
    input! {
        x1: i32,
        x2: i32,
        x3: i32,
        x4: i32,
        x5: i32,
    }
    let ans: i32 = {
        if x1==0 {1}
        else if x2==0 {2}
        else if x3==0 {3}
        else if x4==0 {4}
        else {5}
    };
    println!("{}", ans);
}
#####################################
use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let X: Vec<i32> = input.split_whitespace().map(|s| s.parse().unwrap()).collect();

    for i in 0..5 {
        if X[i] == 0 {
            println!("{}", i + 1);
            break;
        }
    }
}
#####################################
