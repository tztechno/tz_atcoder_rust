//abc170_a variables.rs
#####################################
#####################################
use proconio::input;

fn main() {
    input!{
        x: [u64; 5],
    }
    for i in 0..5 {
        if x[i] == 0 {
            println!("{}", i+1);
        }
    }
}

#####################################
use proconio::*;

fn main() {
    input! { a: [u8; 5] }
    for i in 0..5 {
        if a[i] == 0 {
            println!("{}", i + 1);
        }
    }
}
#####################################
use proconio::input;

fn main() {
    input! {
        x:[i32;5],
    };
    let index = x.iter().position(|&x| x == 0).unwrap() + 1;
    println!("{index}");
}
#####################################
use proconio::input;

fn main(){
  for i in 0..5 {
    input!{
      x:i32
    }
    if x == 0 {
      println! ("{}", i+1);
      return;
    }
  }
}
#####################################
use proconio::input;

fn main() {
    for i in 1..6 {
        input! {x:i32,}
        if x == 0 {
            println!("{}", i);
        }
    }
}
#####################################
use proconio::input;

fn main() {
    input! {
        a: i32,
        b: i32,
        c: i32,
        d: i32,
        e: i32,
    }
    
    let result;
    result = if a == 0 { 1 } 
      else if b == 0 { 2 }
      else if c == 0 { 3 }
      else if d == 0 { 4 }
      else { 5 };
    println!("{}", result);
}
#####################################
use proconio::input;

fn main() {
  input! {
    x1: i64,
    x2: i64,
    x3: i64,
    x4: i64,
    x5: i64
  }
  let ans = if x1 == 0 {
    1
  } else if x2 == 0 {
    2
  } else if x3 == 0 {
    3
  } else if x4 == 0 {
    4
  } else {
    5
  };
  println!("{}", ans);
}
#####################################
use proconio::input;
use itertools::Itertools;
fn main() {
    input! {n: [usize; 5]}
    for i in 0..5 {
        if n[i] == 0 {
            return println!("{}", i + 1);
        }
    }
}

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
