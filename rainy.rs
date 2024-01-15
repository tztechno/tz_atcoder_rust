//abc175_a rainy.rs

use std::io;

fn main() {
    let mut A = String::new();
    io::stdin().read_line(&mut A).expect("Failed to read line");
    let A = A.trim();

    let mut DP = vec![0, 0, 0, 0];

    for i in 0..3 {
        if A.chars().nth(i).unwrap() == 'R' {
            DP[i + 1] = DP[i] + 1;
        }
    }

    let max_value = *DP.iter().max().unwrap();
    println!("{}", max_value);
}
