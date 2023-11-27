use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let K: i32 = input.trim().parse().expect("Failed to parse input");

    let h = 21 + K / 60;
    let m = K % 60;

    let ans = format!("{:02}:{:02}", h, m);

    println!("{}", ans);
}
