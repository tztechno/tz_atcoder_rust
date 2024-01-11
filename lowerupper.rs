//ABC192_B lowerupper

########################################################################
use proconio::{input, marker::Chars};

fn main(){
    input!{
        s: Chars,
    }
    for i in 0..s.len() {
        if (i%2 == 0) ^ s[i].is_lowercase() {
            println!("No");
            return;
        }
    }
    println!("Yes");
}

########################################################################

use proconio::input;
use proconio::marker::Bytes;

fn main() {
    input! {
        n: Bytes,
    }
    for i in 0..n.len() {
        if i % 2 == 0 && n[i].is_ascii_uppercase() {
            println!("{}", "No");
            return;
        } else if i % 2 == 1 && n[i].is_ascii_lowercase() {
            println!("{}", "No");
            return;
        }
    }
    println!("{}", "Yes");
}

########################################################################
  
use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let S: Vec<char> = input.trim().chars().collect();
    let mut U = Vec::new();
    let mut L = Vec::new();
    let mut U2 = Vec::new();
    let mut L2 = Vec::new();
    for (i, &c) in S.iter().enumerate() {
        if i % 2 == 0 {
            L.push(c);
            L2.push(c.to_lowercase().next().unwrap());
        } else {
            U.push(c);
            U2.push(c.to_uppercase().next().unwrap());
        }
    }
    if U.iter().collect::<String>() == U2.into_iter().collect::<String>() && 
       L.iter().collect::<String>() == L2.into_iter().collect::<String>() {
        println!("Yes");
    } else {
        println!("No");
    }
}

########################################################################
