use proconio::input;

fn main() {
  input! {
    xs: [i32; 5],
  };
  
  let mut ans: i32 = 0;
  for (i, x) in xs.iter().enumerate() {
    if x == &0 {
      ans = (i + 1) as i32;
      break;
    }
  }
  
  println!("{}", ans);
