abc080_b.rs
##########################################
##########################################
##########################################
##########################################
##########################################
##########################################
[after fix AC]
use std::io;
fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let X: &str = input.trim();
    let mut M = 0;
    for x in X.chars() {
        M += x.to_digit(10).unwrap();
    }
    let M_i32 = M as i32; // Cast M to i32
    if X.parse::<i32>().unwrap() % M_i32 == 0 {
        println!("Yes");
    } else {
        println!("No");
    }
}
##########################################
[before fix WA]
use std::io;
fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let X: &str = input.trim();
    let mut M=0
    for x in list(X) {
      M+=int(x)    
    }
    if (int(X)%M==0){
        println!("Yes");        
    }
    else {
         println!("No");       
    }
}
##########################################
[python]
X=str(input())
M=0
for x in list(X):
  M+=int(x)
if int(X)%M==0:
  print('Yes')
else:
  print('No')
##########################################
