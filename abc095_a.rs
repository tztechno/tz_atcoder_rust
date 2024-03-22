//abc095_a.rs
#########################################
for c in s.chars() {
let mut count = 0;
#########################################
#########################################
#########################################
#########################################
#########################################
#########################################
    use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    }
    let sum = s.into_iter().filter(|&e| e == 'o').count();
    println!("{}", 700 + sum * 100);
}
#########################################
use proconio::input;

fn main() {
    input! {
        s: String,
    }

    let mut count = 0;

    for c in s.chars() {
        if c == 'o' {
            count += 1;
        }
    }

    println!("{}", 700 + 100 * count);
}
#########################################
use proconio::{fastout, input};
#[fastout]
fn main() {
    input! {
        s: String,
    }
    let mut t = 0;
    for i in 0..3 {
        if let Some(c) = s.chars().nth(i) {
            if c == 'o' {
                t += 1;
            }
        }
    }
    println!("{}", 700 + 100 * t);
}
#########################################
