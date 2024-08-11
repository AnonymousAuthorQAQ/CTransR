use std::cmp::max;
use std::io::{self, BufRead};

fn calculate(p: &[usize], a: &[i64], pos: usize, k: usize) -> i64 {
    let mut mx = 0;
    let mut cur = 0;
    let n = p.len();
    let mut vis = vec![false; n];
    let mut current_pos = pos;
    let mut steps = k;

    while !vis[current_pos] && steps > 0 {
        vis[current_pos] = true;
        mx = max(mx, cur + (steps as i64) * a[current_pos]);
        cur += a[current_pos];
        current_pos = p[current_pos];
        steps -= 1;
    }
    mx
}

fn main() {
    let stdin = io::stdin();
    let lock = stdin.lock();
    let reader = lock.lines();

    let mut lines = reader.map(|line| line.unwrap());

    let t: usize = lines.next().unwrap().parse().unwrap();

    for _ in 0..t {
        let line = lines.next().unwrap();
        let mut iter = line.split_whitespace().map(|s| s.parse::<usize>().unwrap());
        let n = iter.next().unwrap();
        let k = iter.next().unwrap();
        let ps1 = iter.next().unwrap() - 1; // convert to 0-indexed
        let ps2 = iter.next().unwrap() - 1; // convert to 0-indexed

        let p: Vec<usize> = lines.next().unwrap()
            .split_whitespace()
            .map(|s| s.parse::<usize>().unwrap() - 1) // convert to 0-indexed
            .collect();

        let a: Vec<i64> = lines.next().unwrap()
            .split_whitespace()
            .map(|s| s.parse::<i64>().unwrap())
            .collect();

        let ans1 = calculate(&p, &a, ps1, k);
        let ans2 = calculate(&p, &a, ps2, k);

        if ans1 == ans2 {
            println!("Draw");
        } else if ans1 > ans2 {
            println!("Bodya");
        } else {
            println!("Sasha");
        }
    }
}