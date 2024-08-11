use std::io;
use std::io::prelude::*;
use std::collections::VecDeque;

fn calculate(p: &Vec<usize>, a: &Vec<i32>, pos: usize, k: usize) -> i64 {
    let mut vis = vec![false; p.len()];
    let mut mx = 0;
    let mut cur = 0;
    let mut pos = pos;
    let mut k = k;
    while !vis[pos] && k > 0 {
        vis[pos] = true;
        mx = mx.max(cur + (k as i64) * (a[pos] as i64));
        cur += a[pos];
        pos = p[pos];
        k -= 1;
    }
    mx
}

fn main() {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut handle = stdout.lock();

    let mut t = String::new();
    stdin.read_line(&mut t).unwrap();
    let t: usize = t.trim().parse().unwrap();

    for _ in 0..t {
        let mut n_k = String::new();
        stdin.read_line(&mut n_k).unwrap();
        let mut n_k = n_k.trim().split_whitespace();
        let n: usize = n_k.next().unwrap().parse().unwrap();
        let k: usize = n_k.next().unwrap().parse().unwrap();
        let ps1: usize = n_k.next().unwrap().parse().unwrap();
        let ps2: usize = n_k.next().unwrap().parse().unwrap();

        let mut p = Vec::with_capacity(n);
        let mut a = Vec::with_capacity(n);
        for _ in 0..n {
            p.push(0);
            a.push(0);
        }

        let mut line = String::new();
        stdin.read_line(&mut line).unwrap();
        let mut line = line.trim().split_whitespace();
        for i in 0..n {
            p[i] = line.next().unwrap().parse().unwrap() - 1;
        }

        let mut line = String::new();
        stdin.read_line(&mut line).unwrap();
        let mut line = line.trim().split_whitespace();
        for i in 0..n {
            a[i] = line.next().unwrap().parse().unwrap();
        }

        let ans1 = calculate(&p, &a, ps1 - 1, k);
        let ans2 = calculate(&p, &a, ps2 - 1, k);

        if ans1 == ans2 {
            handle.write_all(b"Draw\n").unwrap();
        } else if ans1 > ans2 {
            handle.write_all(b"Bodya\n").unwrap();
        } else {
            handle.write_all(b"Sasha\n").unwrap();
        }
    }
}