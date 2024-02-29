//abc078_a　hex.rs
###################################
fn main(){
  proconio::input!{
    a:char,
    b:char,
  }
  if (a==b){
    println!("=")
  }else if (a>b){
    println!(">");
  }else{
    println!("<");
  }
}
###################################
use proconio::{input, marker::Chars};

pub fn main() {
    input !{
        x: char,
        y: char
    };
    println!("{}", if x < y { "<" } else if x == y { "=" } else { ">" });
}
###################################
use proconio::input;
#[proconio::fastout]
fn main() {
    input! {
        a:char,b:char
    }
    println!(
        "{}",
        if a > b {
            ">"
        } else if a == b {
            "="
        } else {
            "<"
        }
    );
}
###################################
#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
    input! {x: char, y: char}

    if x == y {
        println!("=");
    } else if x < y {
        println!("<");
    } else {
        println!(">");
    }
}
###################################
use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        X: String,
        Y: String,
    }
    if X < Y {
        println!("<");
    }
    else if X > Y {
        println!(">");
    }
    else {
        println!("=");
    }
}

###################################
fn input() -> String {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    buf.trim().to_string()
}
fn main() {
    let s = input();
    let v: Vec<_> = s.split_whitespace().collect();
    let (x, y) = (v[0], v[1]);
    if x > y {
        println!(">");
    } else if x < y {
        println!("<");
    } else {
        println!("=")
    }
}

###################################
use num::integer::{gcd, lcm};
use proconio::input;
use proconio::marker::{Bytes, Chars};
use std::char::from_digit;
use std::cmp::{max, min, Ordering, Reverse};
use std::collections::BinaryHeap;
use std::mem::swap;

fn main() {
    input! {
        x:char,
        y:char
    }
    println!(
        "{}",
        match x.cmp(&y) {
            Ordering::Less => '<',
            Ordering::Equal => '=',
            Ordering::Greater => '>',
        }
    )
}
###################################
use proconio::input;
fn main() {
    input! {
        x: String,
        y: String,
    }
    
    let s = vec!['A', 'B', 'C', 'D', 'E', 'F'];
    let index_x = s.iter().position(|&c| c == x.chars().next().unwrap()).unwrap();
    let index_y = s.iter().position(|&c| c == y.chars().next().unwrap()).unwrap();
    
    if index_x < index_y {
        println!("<");      
    } else if index_x > index_y {
        println!(">");        
    } else {
        println!("=");    
    }       
}
###################################
