//abc073_a　september.rs
###################################
###################################
use proconio::{input, marker::Chars};

#[allow(non_snake_case)]
#[allow(unused_variables)]
fn main() {
    input! {N: Chars,};
    println!("{}", if N[0] == '9' || N[1] == '9' {"Yes"} else {"No"});
}

###################################
use proconio::*;

fn main()
{
	input!{n: usize}
	let mut f = [false; 10];
	f[n / 10] = true;
	f[n % 10] = true;
	println!("{}", if f[9] {"Yes"} else {"No"});
}
###################################
use std::io;

fn main() {
  let mut s: String = String::new();
  std::io::stdin().read_line(&mut s).ok();
  let v:Vec<u8> = s.trim().split_whitespace().map(|e| e.parse().ok().unwrap()).collect();
  let x = v[0];
  if (x % 10 == 9 || x / 10 == 9) {
    println!("Yes");
  }
  else {
    println!("No");
  }
}

###################################
use proconio::input;
fn main() {
    input! {
        n: String
    }
    if n.contains("9") {
        println!("Yes");
    } else {
        println!("No");
    }
}
###################################
use proconio::*;
fn main() {
    input! { x: String }
    if x.contains('9') {
        println!("Yes");
    } else {
        println!("No");
    }
}
###################################
