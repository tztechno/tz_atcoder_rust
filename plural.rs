//ABC179_A
//plural

use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let S = input.trim(); // Remove leading and trailing whitespaces

    if S.chars().last().unwrap() == 's' {
        println!("{}{}", S, "es");
    } else {
        println!("{}{}", S, "s");
    }
}
