//abc067_a cookies.rs
###################################
###################################
use proconio::input;
fn main() {
    input! {
        (a, b): (usize, usize),
    }
    let ans = a % 3 == 0 || b % 3 == 0 || (a + b) % 3 == 0;
    println!("{}", if ans { "Possible" } else { "Impossible" });
}
###################################
use proconio::input;
fn main() {
    input! {
        a: usize,
        b: usize,
    }
    if a % 3 == 0 || b % 3 == 0 || (a + b) % 3 == 0 {
        println!("Possible");
    } else {
        println!("Impossible");
    }
}
###################################
use proconio::*;
fn main() {
    input! { a: i64, b: i64 }
    if (a%3 == 0 || b%3==0 || (a+b)%3==0) {
        println!("Possible")      
    }
    else {
        println!("Impossible");    
    }
}    
###################################
use proconio::input;
fn main() {
    input!{x: [u64; 2]}
    if (x[0]%3 == 0 || x[1]%3==0 || (x[0]+x[1])%3==0) {
        println!("Possible")      
    }
    else {
        println!("Impossible");    
    }
}
###################################
