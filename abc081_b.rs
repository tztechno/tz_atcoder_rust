abc081_b.rs
##########################################
N を i32 から usize に変更しました。
A の宣言で mut を使ってミュータブルにし、B を Vec<i32> として宣言し、A.clone() を使って元の配列をコピーしました。
print(i); を println!("{}", i); に修正して、改行を追加しました。
exit() を return; に変更しました。
##########################################
##########################################
##########################################
##########################################
##########################################
use proconio::input;

fn main() {
    input! {
        n: u8,
        mut a_lis: [u32;n],
    }
    let mut ans = 0;
    let mut j = true;

    while j {
        for i in 0..a_lis.len(){
            if a_lis[i] % 2 == 1 {
                j = false;
                break;
            }
            a_lis[i] = a_lis[i] / 2;
        }
        if j {
            ans += 1;    
        }
    }

    println!("{}",ans)
}
##########################################
N を i32 から usize に変更しました。
A の宣言で mut を使ってミュータブルにし、B を Vec<i32> として宣言し、A.clone() を使って元の配列をコピーしました。
print(i); を println!("{}", i); に修正して、改行を追加しました。
exit() を return; に変更しました。

[after fix = AC]
fn main() {
    proconio::input! {
        N: usize,
        mut A: [i32; N]
    }
    
    let mut B: Vec<i32> = A.clone();

    for i in 0..N * 10 {
        for j in 0..N {
            let bj = B[j];
            if bj % 2 == 0 {
                B[j] = bj / 2;
            } else {
                println!("{}", i);
                return;
            }
        }
    }
}

##########################################
[before fix]    
fn main() {
    proconio::input! {
        N: usize,
        A: [i32; N]
    }
    
    B: [i32; N] = A
    
    for i in 0..N*10 {
      for j in 0..N {
        bj=B[j]
        if (bj%2==0){
          B[j]=bj/2    
        } 
        else{
          print(i)
          exit()       
        }
      }
    }
}

##########################################
[python]
N=int(input())
A=list(map(int,input().split()))
B=A
for i in range(N*10):
  for j in range(N):
    bj=B[j]
    if bj%2==0:
        B[j]=bj//2
    else:
        print(i)
        exit()
##########################################
