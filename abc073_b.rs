abc073_b.rs
#######################################
#######################################
#######################################
#######################################
#######################################
use proconio::input;

fn main() {
    input! { n: usize, lr: [(i64, i64); n] }
    println!("{}", lr.iter().fold(0, |sum, x| { sum + x.1 - x.0 + 1 }));
}

#######################################
use proconio::input;

fn main() {
    input! {
        n: usize,
        lr: [(usize, usize); n],
    }
    let mut ans = 0;
    for (l, r) in lr {
        ans += r - l + 1;
    }
    println!("{}", ans);
}
#######################################
use proconio::input;

fn main() {
    input! {
        n: usize,
        lr: [(i32, i32); n],
    }
    let mut t = 0; // to make `t` mutable
    for i in 0..n {
        let l = lr[i].0;
        let r = lr[i].1;
        t += r - l + 1;
    }
    println!("{}", t);
}
#######################################
