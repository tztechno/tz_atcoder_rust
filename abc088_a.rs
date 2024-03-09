//abc088_a.rs
###################################
###################################
use proconio::input;

fn main() {
    input! {
        n: usize,
        a: usize,
    }
    let res = if n % 500 <= a { "Yes" } else { "No" };
    println!("{}", res);
}

###################################
fn main() {
    proconio::input! {
        n: i32,
        a: i32,
    }
    if n % 500 <= a {
        println!("Yes");
    } else {
        println!("No");
    }
}

###################################
use proconio::input;

fn main() {
    input! {
        n: usize,
        a: usize,
    }
    let yes = n % 500 <= a;
    println!("{}", if yes { "Yes" } else { "No" });
}

###################################
use proconio::input;
fn main() {
    input! {
        N: usize,
        A: usize,
    }
    if N%500<=A {
        println!("Yes")
    } else {
        println!("No")
    }
}
###################################
