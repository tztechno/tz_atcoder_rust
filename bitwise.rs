use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let mut iter = input.split_whitespace();

    let A: i32 = iter.next().unwrap().parse().expect("Invalid input");
    let B: i32 = iter.next().unwrap().parse().expect("Invalid input");

    for i in 0..256 {
        if (A ^ i) == B {
            println!("{}", i);
            break;
        }
    }
}
