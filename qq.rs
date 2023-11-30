use std::io;

fn main() {
    let mut input = String::new();
    
    io::stdin().read_line(&mut input)
        .expect("Failed to read line");
    
    let a: i32 = input.chars().nth(0).unwrap().to_digit(10).unwrap() as i32;
    let b: i32 = input.chars().nth(2).unwrap().to_digit(10).unwrap() as i32;

    let ans = a * b;
    println!("{}", ans);
}
