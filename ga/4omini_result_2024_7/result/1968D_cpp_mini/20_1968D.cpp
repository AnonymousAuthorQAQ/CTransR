use std::io::{self, BufRead};
use std::cmp;

fn calculate(p: &[usize], a: &[i64], pos: usize, k: usize) -> i64 {
    let n = p.len();
    let mut mx = 0;
    let mut cur = 0;
    let mut vis = vec![false; n];
    
    let mut steps_remaining = k;
    let mut current_pos = pos;

    while !vis[current_pos] && steps_remaining > 0 {
        vis[current_pos] = true;
        mx = cmp::max(mx, cur + (steps_remaining as i64) * a[current_pos]);
        cur += a[current_pos];
        current_pos = p[current_pos];
        steps_remaining -= 1;
    }
    mx
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    let t: usize = lines.next().unwrap().unwrap().parse().unwrap();
    for _ in 0..t {
        let first_line: Vec<usize> = lines.next().unwrap().unwrap()
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();

        let (n, k, ps1, ps2) = (first_line[0], first_line[1], first_line[2], first_line[3]);

        let p: Vec<usize> = lines.next().unwrap().unwrap()
            .split_whitespace()
            .map(|s| s.parse::<usize>().unwrap() - 1) // Convert to zero-index
            .collect();

        let a: Vec<i64> = lines.next().unwrap().unwrap()
            .split_whitespace()
            .map(|s| s.parse::<i64>().unwrap())
            .collect();

        let ans1 = calculate(&p, &a, ps1 - 1, k); // ps1 - 1 for zero-indexing
        let ans2 = calculate(&p, &a, ps2 - 1, k); // ps2 - 1 for zero-indexing

        if ans1 == ans2 {
            println!("Draw");
        } else if ans1 > ans2 {
            println!("Bodya");
        } else {
            println!("Sasha");
        }
    }
}