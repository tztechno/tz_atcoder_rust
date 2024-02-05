use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");

    let A: &str = input.trim();

    if A == "Hello,World!" {
        println!("AC");
    } else {
        println!("WA");
    }
}
