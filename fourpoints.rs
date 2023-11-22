use std::collections::HashMap;

fn main() {
    let mut x = [0; 3];
    let mut y = [0; 3];

    for i in 0..3 {
        let input: Vec<i32> = read_line()
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        x[i] = input[0];
        y[i] = input[1];
    }

    let mut count_x = HashMap::new();
    let mut count_y = HashMap::new();

    for i in 0..3 {
        *count_x.entry(x[i]).or_insert(0) += 1;
        *count_y.entry(y[i]).or_insert(0) += 1;
    }

    let mut x0 = 0;
    let mut y0 = 0;

    for i in 0..3 {
        if count_x[&x[i]] == 1 {
            x0 = x[i];
        }
        if count_y[&y[i]] == 1 {
            y0 = y[i];
        }
    }

    println!("{} {}", x0, y0);
}

fn read_line() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().to_string()
}
