abc068_b.rs
#########################################
#########################################
#########################################
#########################################
#########################################
#########################################
[exellent!]
fn main() {
    proconio::input! {
        n: u32
    }
    let mut i = 1;
    while i <= n {
        i *= 2;
    }
    i /= 2;
    println!("{}", i);
}
#########################################
[exellent!]
use num::pow;
use proconio::input;
fn main() {
    input! {
        mut n: usize,
    }
    let mut ans = 1;
    loop {
        if n >> ans == 0 {
            println!("{}", pow(2, ans-1));
            return;
        }
        ans += 1;
    }
}
#########################################
fn main() {
    let mut input = String::new();

    std::io::stdin().read_line(&mut input).unwrap();
    let n: u32 = input.trim().parse().unwrap();

    let mut count: u32 = 1;
    let mut num: u32 = 1;

    for i in (2..=n).step_by(2) {
        let mut j = i;
        let mut k = 0;

        while j % 2 == 0 {
            j /= 2;
            k += 1;
        }

        if k >= count {
            count = k;
            num = i;
        }
    }

    println!("{}", num);
}
#########################################
#########################################

-new crateの使用
use num::pow;
use std::cmp;
        
-ansの初期設定
let mut ans: usize = 0;
    
-pow,maxの使い方        
let t=pow(2,i);
ans = cmp::max(ans, t); 

#########################################
[my ans]
use proconio::input;
use num::pow;
use std::cmp;

fn main() {
    input! {
        n: usize,
    }
    let mut ans: usize = 0; 
    for i in 0..n+1 {
      let t=pow(2,i);
      if t<=n {
        ans = cmp::max(ans, t);      
      }
      else {
        break;      
      } 
    }
    println!("{}", ans);
}
#########################################
