//abc086_a.rs
###################################
###################################

use proconio::input;
use num_traits::{Num, FromPrimitive};
fn floor<T>(x: T, m: T)->T where T: Copy+Num,{let r = (x%m+m)%m;(x-r)/m}
fn ceil<T>(x: T, m: T)->T where T:Copy+Num+FromPrimitive{floor(x+m-T::from_i8(1).unwrap(), m)}
fn chmax<T: PartialOrd+Copy>(a: &mut T, b: &T)->bool{if *a<*b {*a=b.clone(); true}else {false}}
fn chmin<T: PartialOrd+Copy>(a: &mut T, b: &T)->bool{if *a>*b {*a=b.clone(); true}else {false}}

fn main() {
    input! {
        a: i32,
        b: i32
    }
    let res = if a*b%2==0 {"Even"} else {"Odd"};
    println!("{}", res);
        
}
###################################
use proconio::input;
fn main() {
    input! {
        a: usize,
        b: usize,
    }
    println!("{}", if a * b % 2 == 0 { "Even" } else { "Odd" })
}
###################################
use proconio::input;
pub fn main() {
    input! {
        a: usize,
        b: usize
    }
    if a * b % 2 == 0 {
        println!("Even");
    } else {
        println!("Odd");
    }
}
###################################
use proconio::*;
fn main() {
    input! { a: i32, b: i32 }
    if (a*b)%2==0 {
        println!("Even");
    } else {
        println!("Odd");       
    }
}
###################################
