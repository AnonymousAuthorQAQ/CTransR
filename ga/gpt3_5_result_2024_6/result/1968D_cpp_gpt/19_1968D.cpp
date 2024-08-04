use std::io::{self, Read};
use std::collections::HashMap;

fn calculate(p: &Vec<usize>, a: &Vec<usize>, mut pos: usize, k: i64) -> i64 {
    let n = p.len();
    let mut mx = 0;
    let mut cur = 0;
    let mut vis = vec![false; n];
    let mut k = k;
    while !vis[pos] && k > 0 {
        vis[pos] = true;
        mx = mx.max(cur + (k * a[pos] as i64));
        cur += a[pos] as i64;
        pos = p[pos];
        k -= 1;
    }
    mx
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut lines = input.lines();

    let t: usize = lines.next().unwrap().trim().parse().unwrap();
    for _ in 0..t {
        let line: Vec<usize> = lines.next().unwrap()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        let n = line[0];
        let k = line[1];
        let ps1 = line[2];
        let ps2 = line[3];

        let mut p = vec![0; n];
        let mut a = vec![0; n];

        let mut line: Vec<usize> = lines.next().unwrap()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        for i in 0..n {
            p[i] = line[i] - 1;
        }

        let line: Vec<usize> = lines.next().unwrap()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        for i in 0..n {
            a[i] = line[i];
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