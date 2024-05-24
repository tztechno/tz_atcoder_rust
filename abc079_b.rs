abc079_b.rs
##########################################
##########################################
##########################################
##########################################
fn main(){
  proconio::input!{n:usize}
  let mut l = vec![2_u64, 1_u64];
  for i in 0..(n-1){
    l.push(l[i]+l[i+1]);
  }
  println!("{}", l[n])
}
##########################################
use proconio::input;
fn main() {
    input! {
        n: usize,
    }
    let lucas_numbers = (2..=n).fold(vec![2_u64, 1_u64], |mut acc, _| {
        let len = acc.len();
        acc.push(acc[len - 1] + acc[len - 2]);
        acc
    });
    let result = lucas_numbers.last().unwrap();
    println!("{}", result);
}
##########################################
use itertools::Itertools;
use proconio::input;
fn main() {
    input! {
        n: usize
    }
    let mut v = vec![0usize; n + 1];
    v[0] = 2;
    v[1] = 1;
    for i in 2..n + 1 {
        v[i] = v[i - 1] + v[i - 2];
    }
    println!("{}", v[n]);
}
##########################################
##########################################
[AC:予めリスト要素を準備しておく]
use std::io;
fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let n: usize = input.trim().parse().expect("Invalid input");
    let mut v = vec![0usize; n+1];
    v[0] = 2;
    v[1] = 1;
    for i in 2..n+1 {
        v[i] = v[i-1] + v[i-2];
    }
    println!("{}", v[n]); 
}
##########################################
[AC:pushで要素を加えていく][i32はエラー、usizeはAC]
use std::io;
fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let n: usize = input.trim().parse().expect("Invalid input");
    let mut v: Vec<usize> = vec![2, 1];
    for i in 2..n+1 {
        v.push(v[i-1] + v[i-2]);
        }
    println!("{}", v[n]); 
}
##########################################
[AC:pushで要素を加えていく][i32はエラー、i64はAC]
use std::io;
fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let n: usize = input.trim().parse().expect("Invalid input");
    let mut v: Vec<i64> = vec![2,1];
    for i in 2..n+1 {
        v.push(v[i-1]+v[i-2]);
        }
    println!("{}", v[n]); 
}
##########################################
[python]
N=int(input())
L=[2,1]
for i in range(N-1):
    L+=[L[-2]+L[-1]]
print(L[-1])
##########################################
