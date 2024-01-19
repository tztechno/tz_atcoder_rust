//abc174_a conditioner.rust

####################################
fn main() {
    proconio::input! {
        x: i32
    }

    if x >= 30 {
        println!("Yes");
    } else {
        println!("No");
    }
}
####################################
use proconio::input;
use proconio::fastout;

fn main() {
    input! {
        X: isize,
    }
    if X >= 30 {
        println!("Yes");
    }
    else {
        println!("No");
    }
}
####################################
use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let x: i32 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            eprintln!("Invalid input. Please enter a number.");
            return;
        }
    };

    let judge = x >= 30;
    let ans = if judge { "Yes" } else { "No" };

    println!("{}", ans);
}
####################################
use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let x: i32 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            eprintln!("Invalid input. Please enter a number.");
            return;
        }
    };

    if x >= 30 {
        println!("Yes");
    } else {
        println!("No");
    }
}
####################################
