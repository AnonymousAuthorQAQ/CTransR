use std::io::{self, Read};
use std::cmp::max;

fn calculate(p: &Vec<usize>, a: &Vec<usize>, pos: usize, k: usize) -> i64 {
    let n = p.len();
    let mut mx = 0;
    let mut cur = 0;
    let mut vis = vec![false; n];
    while !vis[pos] && k > 0 {
        vis[pos] = true;
        mx = max(mx, cur + (k as i64) * (a[pos] as i64));
        cur += a[pos];
        pos = p[pos];
        k -= 1;
    }
    mx
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut lines = input.lines();
    let t: usize = lines.next().unwrap().parse().unwrap();
    for _ in 0..t {
        let mut line = lines.next().unwrap().split_whitespace();
        let n: usize = line.next().unwrap().parse().unwrap();
        let k: usize = line.next().unwrap().parse().unwrap();
        let ps1: usize = line.next().unwrap().parse().unwrap();
        let ps2: usize = line.next().unwrap().parse().unwrap();
        let mut p = Vec::with_capacity(n);
        let mut a = Vec::with_capacity(n);
        for _ in 0..n {
            p.push(line.next().unwrap().parse().unwrap() - 1);
        }
        for _ in 0..n {
            a.push(line.next().unwrap().parse().unwrap());
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