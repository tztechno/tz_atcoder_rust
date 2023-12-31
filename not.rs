//ABC178_A 
//Not

use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let mut iter = input.split_whitespace();
    let N: i32 = iter.next().unwrap().parse().expect("Invalid input");

    if N==1 {
        println!("{}", 0);
    } else {
        println!("{}", 1);        
    }
}


