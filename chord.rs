use std::io;

fn main() {
    let mut s = String::new();
    println!("Enter a string: ");
    io::stdin().read_line(&mut s).expect("Failed to read line");
    let s = s.trim();

    let name = vec!["ACE", "BDF", "CEG", "DFA", "EGB", "FAC", "GBD"];

    if name.contains(&s) {
        println!("Yes");
    } else {
        println!("No");
    }
}
