###################################################
###################################################
###################################################
    for i in 0..s.len() {
        if (i%2 == 0) ^ s[i].is_lowercase() {
            println!("No");
            return;
        }
    }
###################################################
  for i in 0..3 {
        if A.chars().nth(i).unwrap() == 'R' {
            DP[i + 1] = DP[i] + 1;
        }
###################################################

    for i in 0..256 {
        if (A ^ i) == B {
            println!("{}", i);
            break;
        }
    }
###################################################
