use std::io;

fn main() {
    let mut input = String::new();

    //println!("Enter a value for S: ");
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let input = input.trim(); 

    if input.ends_with("er") {
        println!("er");
    } else {
        println!("ist");
    }
}
