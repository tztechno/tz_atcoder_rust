#ABC181_A
#rotation
#input= 5


========================================================================

use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let mut iter = input.split_whitespace();
    let N: i32 = iter.next().unwrap().parse().expect("Invalid input");

    if N%2 == 0 {
        println!("White");
        }
    else {
        println!("Black");
        }
    }

========================================================================

use proconio::input;

fn main() {
    input!{
        n: i32,

    }
    let ans: &str;
    if n % 2 == 0{
        ans = "White";
    }
    else{
        ans = "Black";
    }

    println!("{}", ans);

}

========================================================================

use proconio::input;

fn main() {
  input! {
  	a: i32
  }
  let ans = if a % 2 == 0 { "White" } else  { "Black" };
  println!("{}", ans);
}


========================================================================
