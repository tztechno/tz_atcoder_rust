abc072_b.rs
######################################
######################################
######################################
######################################
######################################
######################################
use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    }
    println!("{}", s.iter().step_by(2).join(""));
}

######################################
use proconio::input;

fn main() {
    input! {
        s: String,
    }

    for (i, c) in s.chars().enumerate() {
        if i % 2 == 0 {
            print!("{}", c);
        }
    }

    println!();
}
######################################
use proconio::input;

fn main() {
    input! {
        s:String,
    }

    for i in 0..s.len() {
        if i % 2 == 0 {
            let tmp_s = s.chars().nth(i).unwrap();
            print!("{}",tmp_s);
        }
    }

    println!("");
}
######################################
use std::io;

fn main() {
    let mut s = String::new();
    io::stdin().read_line(&mut s).expect("Failed to read line");
    s = s.trim().to_string(); // Remove newline character

    let mut a = String::new();

    for i in (0..s.len()).step_by(2) {
        a.push(s.as_bytes()[i] as char);
    }

    println!("{}", a);
}
######################################
use std::io::{self, BufRead};

fn main() {
    let mut s = String::new();
    io::stdin().lock().read_line(&mut s).expect("Failed to read line");
    let s = s.trim();

    let mut a = String::new();
    for (i, c) in s.chars().enumerate() {
        if i % 2 == 0 {
            a.push(c);
        }
    }
    println!("{}", a);
}

######################################
