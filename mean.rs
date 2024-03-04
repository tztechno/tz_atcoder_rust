//abc082_a　mean.rs
###################################
###################################
###################################
use std:: io::stdin;
fn main(){
    let  a = input();
    let cal1 = (a[0] + a[1]) % 2;
    let mut cal2 = (a[0] + a[1]) / 2;
    if cal1 != 0{
        cal2 += 1;
    }
    println!("{}", cal2);
}

fn input()->Vec<usize>{
    let mut a = String::new();
    stdin().read_line(&mut a).unwrap();
    return a.trim().split_whitespace().map(|e| e.parse().ok().unwrap()).collect();
}
###################################
use proconio::input;
fn main() {
    input! {
        a: usize,
        b: usize,
    }
    println!("{}", ((a as f64 + b as f64) / 2.).ceil());
}
###################################
use proconio::input;
fn main() {
    input! {
        a: f64,
        b: f64
    }
    let val = (a + b) / 2.0;
    println!("{}", val.ceil());
}
###################################
use proconio::*;
fn main() {
    input! { a: i64, b: i64 }
    println!("{}", (a+b+1)/2);
}
###################################
