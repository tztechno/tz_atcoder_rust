use std::io;

fn main() {
    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("Failed to read line");
    let X: f64 = input.trim().parse().expect("Invalid input. Please enter a valid floating-point number.");

    let a = (X * 10.0) as i32;

    match a % 10 {
        v if v >= 5 => println!("{}", X.floor() as i32 + 1),
        v if v <= 4 => println!("{}", X.floor() as i32),
        _ => {}
    }
}
