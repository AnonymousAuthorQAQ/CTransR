use std::io::{self, BufRead};

fn calculate(p: &Vec<usize>, a: &Vec<i64>, mut pos: usize, mut k: usize) -> i64 {
    let n = p.len();
    let mut mx = 0;
    let mut cur = 0;
    let mut vis = vec![false; n];

    while !vis[pos] && k > 0 {
        vis[pos] = true;
        mx = mx.max(cur + (k as i64) * a[pos]);
        cur += a[pos];
        pos = p[pos];
        k -= 1;
    }

    mx
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let lines: Vec<String> = reader.lines().map(|line| line.unwrap()).collect();

    let mut idx = 0;
    let t: usize = lines[idx].parse().unwrap();
    idx += 1;

    for _ in 0..t {
        let data: Vec<usize> = lines[idx]
            .split_whitespace()
            .map(|x| x.parse::<usize>().unwrap())
            .collect();
        let n = data[0];
        let k = data[1];
        let ps1 = data[2] - 1; // Convert to 0-based
        let ps2 = data[3] - 1; // Convert to 0-based
        idx += 1;

        let p: Vec<usize> = lines[idx]
            .split_whitespace()
            .map(|x| x.parse::<usize>().unwrap() - 1) // Convert to 0-based
            .collect();
        idx += 1;

        let a: Vec<i64> = lines[idx]
            .split_whitespace()
            .map(|x| x.parse::<i64>().unwrap())
            .collect();
        idx += 1;

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