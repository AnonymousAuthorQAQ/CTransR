use std::io;
use std::io::{BufRead, BufReader};

fn calculate(p: &Vec<i32>, a: &Vec<i32>, pos: i32, k: i32) -> i64 {
    let n = p.len() as i32;
    let mut mx = 0;
    let mut cur = 0;
    let mut vis = vec![false; n as usize];

    while !vis[pos as usize] && k > 0 {
        vis[pos as usize] = true;
        mx = mx.max(cur + (k as i64) * (a[pos as usize] as i64));
        cur += a[pos as usize];
        pos = p[pos as usize];
        k -= 1;
    }

    mx
}

fn main() {
    let stdin = io::stdin();
    let mut lines = BufReader::new(stdin).lines();

    let t: i32 = lines.next().unwrap().unwrap().parse().unwrap();

    for _ in 0..t {
        let mut line = lines.next().unwrap().unwrap();
        let mut parts = line.split_whitespace();
        let n: i32 = parts.next().unwrap().parse().unwrap();
        let k: i32 = parts.next().unwrap().parse().unwrap();
        let ps1: i32 = parts.next().unwrap().parse().unwrap();
        let ps2: i32 = parts.next().unwrap().parse().unwrap();

        let mut p = Vec::with_capacity(n as usize);
        let mut a = Vec::with_capacity(n as usize);

        for _ in 0..n {
            p.push(parts.next().unwrap().parse().unwrap() - 1);
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