//ABC177_A late

#######################################
use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        (D, T, S): (usize, usize, usize),
    }
    if (D + S - 1) / S <= T {
        println!("Yes");
    }
    else {
        println!("No");
    }
}
#######################################
use proconio::input;

fn main() {
    input! {
        D: i32,
        T: i32,
        S: i32,
    }
    if ((T * S)>= D ) {
        println!("Yes");
    } else {
        println!("No");
    }
}

#######################################
### my ans
  
use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let mut iter = input.split_whitespace();

    let D: f64 = iter.next().unwrap().parse().expect("Invalid input");
    let T: f64 = iter.next().unwrap().parse().expect("Invalid input");
    let S: f64 = iter.next().unwrap().parse().expect("Invalid input");

    if D / S <= T {
        println!("Yes");
    } else {
        println!("No");
    }
}

#######################################
use proconio::input;

fn main() {
    input! {
        d: f32,
        t: f32,
        s: f32,
    }
    println!("{}", if t >= d / s { "Yes" } else { "No" });
}

#######################################
