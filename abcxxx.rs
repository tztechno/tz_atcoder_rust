//abc068_a　abcxxx.rs
###################################
###################################
###################################
###################################
#![allow(unused_imports)]
use proconio::{input, marker::{Chars, Usize1}, source::line::LineSource};
use std::{collections::*, io::*, cmp::*};
use itertools::Itertools;
use superslice::Ext;
use num::*;

fn main() {
    let mut stdin = LineSource::new(BufReader::new(stdin()));
    macro_rules! input(($($tt:tt)*) => (proconio::input!(from &mut stdin, $($tt)*)));
    input!{
        n: usize,
    }
    println!("ABC{}", n);
}
###################################
use proconio::input;
#[allow(non_snake_case)]
#[allow(unused_variables)]
fn main() {
    input!{n:usize,};
    println!("ABC{}", n);
}

###################################
use proconio::input;
fn main() {
    input! {
        n: usize,
    }
    println!("ABC{}", n);
}
###################################
use proconio::*;
fn main() {
    input! { a: String }
    println!("ABC{}", a);
}
###################################
