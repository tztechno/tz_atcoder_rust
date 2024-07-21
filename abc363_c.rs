abc363_c.rs
#################################################
[TLE]

use itertools::Itertools; // Import Itertools for permutations
use std::collections::HashSet;

fn is_non_palindromic(s: &str, k: usize) -> bool {
    for i in 0..=s.len() - k {
        let substring = &s[i..i + k];
        if is_palindrome(substring) {
            return false;
        }
    }
    true
}

fn is_palindrome(s: &str) -> bool {
    let chars: Vec<_> = s.chars().collect();
    let len = chars.len();
    for i in 0..len / 2 {
        if chars[i] != chars[len - 1 - i] {
            return false;
        }
    }
    true
}

fn permute(s: &str) -> HashSet<String> {
    let mut results = HashSet::new();
    let chars: Vec<char> = s.chars().collect();
    for perm in chars.iter().permutations(chars.len()) {
        let perm_str: String = perm.into_iter().map(|&c| c).collect();
        results.insert(perm_str);
    }
    results
}

fn main() {
    use std::io::{self, Read};

    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut lines = input.lines();

    let first_line = lines.next().unwrap();
    let mut parts = first_line.split_whitespace();
    let n: usize = parts.next().unwrap().parse().unwrap();
    let k: usize = parts.next().unwrap().parse().unwrap();

    let s = lines.next().unwrap();

    let perms = permute(s);
    let mut count = 0;
    
    for perm in perms {
        if is_non_palindromic(&perm, k) {
            count += 1;
        }
    }
    
    println!("{}", count);
}
