use std::io;

fn main() {
    let mut input = String::new();

    //println!("Enter an integer: ");
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let D: i32 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            eprintln!("Invalid input. Please enter an integer.");
            std::process::exit(1);
        }
    };

    let ans = D as f64 / 100.0;
    println!("{}", ans);
}
