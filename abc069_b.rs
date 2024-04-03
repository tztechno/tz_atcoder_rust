abc069_b.rs
#########################################
s.chars().nth(0).unwrap()
s.len()-2 
chars.first().unwrap(),
chars.len() - 2,
#########################################
#########################################
#########################################
#########################################
use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    }
    println!("{}{}{}", s[0], s.len()-2, s[s.len()-1]);
}
#########################################
fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    let chars: Vec<_> = input.trim().chars().collect();

    println!(
        "{}{}{}",
        chars.first().unwrap(),
        chars.len() - 2,
        chars.last().unwrap()
    );
}

#########################################
use proconio::input;
use std::collections::HashSet;
use std::cmp;

fn main() {
    input!{
        mut s: String,
    }
    println!("{}{}{}",s.chars().nth(0).unwrap(), s.len()-2 ,s.chars().nth(s.len()-1).unwrap());
}
#########################################
use proconio::input;
fn main() {
    input! {
        a: String,
    }
    let y = a.len(); // Use len() to get string length
    if y <= 2 {
        println!("{}", a); // Print original string if length is 2 or less
    } else {
        let first_char = a.chars().next().unwrap(); // Get first character
        let last_char = a.chars().rev().next().unwrap(); // Get last character
        let y_str = (y - 2).to_string(); // Convert length to String
        let ans = format!("{}{}{}", first_char, y_str, last_char); // Use format! for cleaner string building
        println!("{}", ans);
    }
}
#########################################
