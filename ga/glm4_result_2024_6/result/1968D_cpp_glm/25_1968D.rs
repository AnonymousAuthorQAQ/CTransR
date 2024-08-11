use std::io;
use std::io::prelude::*;
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
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut handle = stdout.lock();

    let mut t = 1;
    let mut line = String::new();
    stdin.read_line(&mut line).unwrap();
    t = line.trim().parse::<usize>().unwrap();

    while t > 0 {
        let mut line = String::new();
        stdin.read_line(&mut line).unwrap();
        let mut parts = line.trim().split_whitespace();
        let n = parts.next().unwrap().parse::<usize>().unwrap();
        let k = parts.next().unwrap().parse::<usize>().unwrap();
        let ps1 = parts.next().unwrap().parse::<usize>().unwrap();
        let ps2 = parts.next().unwrap().parse::<usize>().unwrap();

        let mut p = Vec::with_capacity(n);
        let mut a = Vec::with_capacity(n);

        for _ in 0..n {
            let mut line = String::new();
            stdin.read_line(&mut line).unwrap();
            p.push(line.trim().parse::<usize>().unwrap() - 1);
        }

        for _ in 0..n {
            let mut line = String::new();
            stdin.read_line(&mut line).unwrap();
            a.push(line.trim().parse::<usize>().unwrap());
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

        t -= 1;
    }
}