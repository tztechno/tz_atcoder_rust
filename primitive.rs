use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let mut iter = input.split_whitespace();

    let a: i32 = iter.next().unwrap().parse().expect("Failed to parse A");
    let b: i32 = iter.next().unwrap().parse().expect("Failed to parse B");
    let c: i32 = iter.next().unwrap().parse().expect("Failed to parse C");

    if c == 0 {
        if a <= b {
            println!("Aoki");
        } else {
            println!("Takahashi");
        }
    } else if c == 1 {
        if b <= a {
            println!("Takahashi");
        } else {
            println!("Aoki");
        }
    }
}
