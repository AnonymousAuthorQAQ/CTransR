use std::io::{self, BufRead};
use std::cmp::max;

fn calculate(p: &Vec<usize>, a: &Vec<usize>, pos: usize, k: usize) -> u64 {
    let mut mx = 0;
    let mut cur = 0;
    let mut vis = vec![false; p.len()];
    while !vis[pos] && k > 0 {
        vis[pos] = true;
        mx = max(mx, cur + (k as u64) * (a[pos] as u64));
        cur += a[pos];
        pos = p[pos];
        k -= 1;
    }
    mx
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let t: usize = lines.next().unwrap().parse().unwrap();
    for _ in 0..t {
        let mut line = lines.next().unwrap();
        let mut parts = line.split_whitespace();
        let n: usize = parts.next().unwrap().parse().unwrap();
        let k: usize = parts.next().unwrap().parse().unwrap();
        let ps1: usize = parts.next().unwrap().parse().unwrap();
        let ps2: usize = parts.next().unwrap().parse().unwrap();
        let mut p = Vec::with_capacity(n);
        let mut a = Vec::with_capacity(n);
        for _ in 0..n {
            p.push(parts.next().unwrap().parse::<usize>().unwrap() - 1);
        }
        for _ in 0..n {
            a.push(parts.next().unwrap().parse().unwrap());
        }
        let ans1 = calculate(&p, &a, ps1 - 1, k);
        let ans2 = calculate(&p, &a, ps2 - 1, k);
        if ans1 == ans2 {
            println!("Draw");
        } else if ans1 > ans2 {
            println!("Bodya");
        } else {
            println!("Sasha");
        }
    }
}