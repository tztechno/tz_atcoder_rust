use std::collections::HashMap;

fn fibonacci(n: u64, memo: &mut HashMap<u64, u64>) -> u64 {
    if n <= 1 {
        return n;
    }

    if let Some(&result) = memo.get(&n) {
        return result;
    }

    let result = fibonacci(n - 1, memo) + fibonacci(n - 2, memo);
    memo.insert(n, result);
    result
}

fn main() {
    let n = 35;
    let mut memo: HashMap<u64, u64> = HashMap::new();
    println!("{}", fibonacci(n, &mut memo));
}
