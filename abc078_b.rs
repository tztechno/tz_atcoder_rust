abc078_b.rs
#################################
#################################
#################################
use proconio::input;
fn main() {
    input! {
        (x, y, z): (usize, usize, usize),
    }
    println!("{}", (x - z) / (y + z));
}
#################################
use proconio::input;
fn main() {
    input! {
        x: usize,
        y: usize,
        z: usize,
    }
    println!("{}", (x - z) / (y + z));
}
#################################
use proconio::*;
fn main() {
  input! { X: i32, Y: i32, Z: i32 };
  if X%(Y+Z)>=Z {
    println!("{}",X/(Y+Z));    
  }
  else {
    println!("{}",X/(Y+Z)-1);     
  }
}#############################
[python]
X,Y,Z=map(int,input().split())
if X%(Y+Z)>=Z:
  print(X//(Y+Z))
else:
  print(X//(Y+Z)-1)
#################################
