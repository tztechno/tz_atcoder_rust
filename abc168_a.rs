//abc168_a.rs
################################
################################
################################
use proconio::input;

fn main(){
  input!{
    a:i32,
  }
  match a%10{
    0|1|6|8=>println!("pon"),
    3=>println!("bon"),
    _=>println!("hon"),
  }
}
################################
fn main()
{
	proconio::input!
	{
		n: i32
	}


	println!("{}",match n % 10
	{
		2 | 4 | 5 | 7 | 9 => "hon",
		0 | 1 | 6 | 8 => "pon",
		3 => "bon",
		_ => unreachable!()
	});
}
################################
use proconio::input;

fn main(){
    input! {
        n: i32, 
    }

    match n%10 {
        2 |  4 |  5 |  7 |  9 => println!("hon"),
        0 | 1 | 6 | 8 => println!("pon"),
        _ => println!("bon"),
    }

}
################################
use proconio::input;

fn main() {
    input! {
        n: u32,
    }

    match n % 10 {
        2 | 4 | 5 | 7 | 9 => println!("hon"),
        0 | 1 | 6 | 8 => println!("pon"),
        3 => println!("bon"),
        _ => unreachable!(),
    }
}
################################
use proconio::input;
fn main() {
    input! {
        s: i32,
    }
    let a = s % 10;
    if [2, 4, 5, 7, 9].contains(&a) {
        println!("hon");
    } else if [0, 1, 6, 8].contains(&a) {
        println!("pon");
    } else {
        println!("bon");
    }
}
################################
