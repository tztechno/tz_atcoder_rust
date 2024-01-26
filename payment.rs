//abc173_a payment.rs

##############################
use proconio::input;
use proconio::fastout;

fn main() {
    input! {
        N: usize,
    }
    if N % 1000 == 0 {
        println!("0");
    }
    else {
        println!("{}", 1000 - N % 1000);
    }
}
##############################
use proconio::input;

fn main() {
    input! {
        n: usize,
    }
    if n % 1000 == 0 {
        println!("0");
    } else {
        println!("{}", 1000 - n % 1000);
    }
}
##############################
use proconio::input;

fn main() {
    proconio::input! {
        N: i32
    }
    let A = (1000 - N % 1000) % 1000;
    println!("{}", A);
}
##############################
