###############################################################
###############################################################
###############################################################
###############################################################
###############################################################
use itertools::Itertools;
use proconio::{fastout, input, marker::Usize1};
use std::fmt;

const N: usize = 200;
const M: usize = 10;

struct Output {
    vi: Vec<(usize, usize)>,
}

impl fmt::Display for Output {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for vi in self.vi.iter() {
            writeln!(f, "{} {}", vi.0 + 1, (vi.1 + 1) % (M + 1))?
        }
        Ok(())
    }
}

struct State {
    output: Output,
    b: Vec<Vec<usize>>,
}

impl State {
    fn find(&self, v: usize) -> (usize, usize) {
        let s = (0..M).find(|&i| self.b[i].contains(&v)).unwrap();
        let t = (0..self.b[s].len()).find(|&i| self.b[s][i] == v).unwrap();

        (s, t)
    }

    fn ope1(&mut self, v: usize, i: usize) -> bool {
        let s = (0..M).find(|&i| self.b[i].contains(&v)).unwrap();
        let t = (0..self.b[s].len()).find(|&i| self.b[s][i] == v).unwrap();
        if i == s {
            false
        } else {
            let mut c = vec![];
            for j in t..self.b[s].len() {
                c.push(self.b[s].pop().unwrap());
            }
            while let Some(x) = c.pop() {
                self.b[i].push(x);
            }
            self.output.vi.push((v, i));
            true
        }
    }

    fn ope2(&mut self) -> bool {
        let next = N - self.b.iter().flatten().count();
        for i in 0..M {
            if self.b[i].iter().last() == Some(&next) {
                self.output.vi.push((self.b[i].pop().unwrap(), M));
                return true;
            }
        }

        false
    }
}

#[fastout]
fn main() {
    input! {
        n: usize, m: usize,
        b: [[Usize1; n / m]; m],
    }

    let output = Output { vi: vec![] };
    let mut state = State { output, b };

    for i in 0..n {
        if state.ope2() {
            continue;
        }

        let (s, t) = state.find(i);

        let mut v = vec![t];
        let mut now = t;
        while now + 1 < state.b[s].len() {
            let next = (now + 1..state.b[s].len())
                .min_by_key(|&i| state.b[s][i])
                .unwrap();
            v.push(next);
            now = next;
        }
        if *v.last().unwrap() == state.b[s].len() - 1 {
            v.pop();
        }
        for x in v.into_iter().rev() {
            if let Some(j) = (0..m)
                .filter(|&j| j != s)
                .filter(|&j| {
                    state.b[j].len() + state.b[s].len() - (x + 1)
                        <= state.b.iter().flatten().count() / M * 3 / 2
                })
                .sorted_by_key(|&j| state.b[j].iter().min().unwrap_or(&N))
                .last()
            {
                state.ope1(state.b[s][x + 1], j);
            } else {
                let j = (0..m)
                    .filter(|&j| j != s)
                    .sorted_by_key(|&j| state.b[j].iter().min().unwrap_or(&N))
                    .last()
                    .unwrap();
                state.ope1(state.b[s][x + 1], j);
            }
        }

        state.ope2();
    }

    println!("{}", state.output);
}

###############################################################
// -*- coding:utf-8-unix -*-

use proconio::input;
use std::collections::{BinaryHeap, VecDeque};

fn main() {
    input! {
        n: usize,
        m: usize,
        mut b: [[usize; n/m]; m],
    }

    for i in 0..n {
        // i+1を外に取り出す

        let mut change = false;
        let mut ans_j = 0;
        let mut ans_k = 0;
        for j in 0..m {
            for k in 0..b[j].len() {
                if b[j][k] == i + 1 {
                    ans_j = j;
                    if k != b[j].len() - 1 {
                        println!("{} {}", b[j][k + 1], (j + 1) % m + 1);
                        ans_k = k;
                        change = true;
                        break;
                    }
                }
            }
        }
        if change {
            let mut po: Vec<_> = b[ans_j].drain(ans_k + 1..).collect();
            b[(ans_j + 1) % m].append(&mut po);
        }
        b[ans_j].pop();

        println!("{} {}", i + 1, 0);
    }
}
###############################################################
