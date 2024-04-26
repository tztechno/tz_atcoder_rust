abc076_b.rs
##############################################
##############################################
##############################################
##############################################
use proconio::input;

fn main() {
    input! {
        n:usize,
        k:usize,
    }
    let mut tmp = 1;
    for _ in 0..n {
        tmp = (tmp * 2).min(tmp + k);
    }
    println!("{}", tmp);
}
##############################################
use proconio::input;
fn main() {
    input! {
        n: usize,
        k: usize,
    }
    let mut ans = 1;
    for _ in 0..n {
        ans = (ans * 2).min(ans + k)
    }
    println!("{}", ans);
}

##############################################
use std::cmp::min;
use proconio::input;

fn main() {
    input! {
        mut n: usize,
        mut k: usize,        
    }
    let mut ans = 1;
    for _ in 0..n {
        ans = min(ans * 2, ans + k);
    }
    println!("{}", ans);
}
##############################################
