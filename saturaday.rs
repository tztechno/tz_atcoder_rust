use std::collections::HashMap;

fn main() {
    proconio::input! {
        S: String,
    }

    let name = vec!["Monday", "Tuesday", "Wednesday", "Thursday", "Friday"];
    let days = vec![5, 4, 3, 2, 1];

    let mapping: HashMap<_, _> = name.iter().zip(days.iter()).map(|(k, v)| (String::from(*k), *v)).collect();

    if let Some(&value) = mapping.get(&S) {
        println!("{}", value);
    } else {
        println!("Day not found");
    }
}
