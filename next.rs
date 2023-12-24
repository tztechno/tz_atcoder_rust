#ABC329_B
#next

use std::io;
use std::collections::HashSet; // Import HashSet for creating the set B

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let N: usize = input.trim().parse().expect("Invalid input");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let A: Vec<i32> = input
        .split_whitespace()
        .map(|s| s.parse().expect("Invalid input"))
        .collect();

    // Create a set B from the vector A
    // set action in python
    let B: HashSet<i32> = A.into_iter().collect();

    // Collect the set elements into a vector to allow sorting
    let mut C: Vec<i32> = B.into_iter().collect();

    // Sort the vector C in ascending order
    C.sort();

    // Print the second largest element (index -2)
    print!("{} ", C[C.len() - 2]); // Access the element using len()

    println!(); // Add a newline
}
