//abc092_a.rs
################################
################################
################################
################################
################################
use proconio::input;
use std::cmp;

fn main() {
    input! {
        a: i32,
        b: i32,
        c: i32,
        d: i32,
    }

    let train_normal = a;
    let train_pass = b;
    let bus_normal = c;
    let bus_pass = d;

    let train_min = cmp::min(train_normal, train_pass);
    let bus_min = cmp::min(bus_normal, bus_pass);

    let total_min = train_min + bus_min;

    println!("{}", total_min);
}
################################
use proconio::input;
fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
        d: usize
    }
    println!("{}", a.min(b) + c.min(d));
}

################################
use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
        d: usize,
    }
    let bus_cost = a.min(b);
    let train_cost = c.min(d);
    println!("{}", bus_cost + train_cost);
}

################################
use std::cmp::min;
use proconio::input;

fn main() {
    input! {
        (a,b,c,d): (usize, usize, usize, usize),
    }
    let ans = min(a,b)+min(c,d);
    println!("{}", ans);
}

################################
use std::cmp::min;
use proconio::input;

fn main(){
  input!{
    a:i32,
    b:i32,
    c:i32,
    d:i32,
  }
    println!("{}",min(a,b)+min(c,d));
}
################################
