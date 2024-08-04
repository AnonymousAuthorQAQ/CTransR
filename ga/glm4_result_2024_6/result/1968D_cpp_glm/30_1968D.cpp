use std::io;
use std::io::prelude::*;

fn calculate(p: &[i32], a: &[i32], pos: i32, k: i32) -> i64 {
    let mut mx = 0;
    let mut cur = 0;
    let n = p.len();
    let mut vis = vec![false; n];
    let mut pos = pos as usize;
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
    let mut writer = stdout.lock();

    let mut line = String::new();
    stdin.read_line(&mut line).unwrap();
    let t = line.trim().parse::<i32>().unwrap();

    for _ in 0..t {
        line.clear();
        stdin.read_line(&mut line).unwrap();
        let mut iter = line.trim().split_whitespace();
        let n = iter.next().unwrap().parse::<usize>().unwrap();
        let k = iter.next().unwrap().parse::<i32>().unwrap();
        let ps1 = iter.next().unwrap().parse::<i32>().unwrap();
        let ps2 = iter.next().unwrap().parse::<i32>().unwrap();

        let mut p = Vec::with_capacity(n);
        let mut a = Vec::with_capacity(n);

        for _ in 0..n {
            line.clear();
            stdin.read_line(&mut line).unwrap();
            let mut iter = line.trim().split_whitespace();
            p.push(iter.next().unwrap().parse::<i32>().unwrap() - 1);
            a.push(iter.next().unwrap().parse::<i32>().unwrap());
        }

        let ans1 = calculate(&p, &a, ps1 - 1, k);
        let ans2 = calculate(&p, &a, ps2 - 1, k);

        if ans1 == ans2 {
            writeln!(writer, "Draw").unwrap();
        } else if ans1 > ans2 {
            writeln!(writer, "Bodya").unwrap();
        } else {
            writeln!(writer, "Sasha").unwrap();
        }
    }
}