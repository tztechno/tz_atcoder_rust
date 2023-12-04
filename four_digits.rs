use std::io;

fn main() {
    let mut S = String::new();

    println!("Enter a string: ");
    io::stdin().read_line(&mut S).expect("Failed to read line");

    let S = S.trim();

    let n = S.len();

    if n < 4 {
        let padding = "0".repeat(4 - n);
        let padded_string = padding + S;
        println!("{}", padded_string);
    } else {
        println!("{}", S);
    }
}
