//practiceA.rs
##################################
1
2 3
test
##################################
##################################
##################################
##################################
##################################
##################################
##################################
##################################
##################################
[favorite]
fn main() {
    proconio::input! {
        a: i32,
        b: [i32;2],
        c: String
    }
    println!("{} {}", a + b[0]+b[1], c);
}
##################################
use proconio::input;

fn main() {
    input!(a: i32, b: i32, c: i32, s: String);

    println!("{} {s}", a + b + c);
}

##################################
use proconio::input;
fn main() {
    input! {
        a: i32,
        b: i32, c: i32,
        s: String
    }
    println!("{} {}", a+b+c, s);
}
##################################
use proconio::input;

pub fn main() {
    input! {
        a: i32,
        bc: [i32; 2],
        s: String
    }

    println!("{} {}", a + bc[0] + bc[1], s);
}
##################################
use text_io::read;
use text_io::scan;

fn main() {
    let (a, b, c): (i32, i32, i32);
    scan!("{}", a);
    scan!("{} {}", b, c);

    let input_text: String = read!();

    let sum = a + b + c;
    println!("{} {}", sum, input_text);
}

##################################
