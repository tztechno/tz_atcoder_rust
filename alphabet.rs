use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let x = input.trim();

    if x.chars().count() == 1 && x.chars().all(char::is_alphabetic) {
        if x.chars().next().unwrap().is_lowercase() {
            println!("a");
        } else {
            println!("A");
        }
    } else {
        println!("Invalid input: Please enter a single alphabet character.");
    }
}
