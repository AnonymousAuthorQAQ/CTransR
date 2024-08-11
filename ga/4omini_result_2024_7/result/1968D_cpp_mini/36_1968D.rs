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
    let mut lines = reader.lines().filter_map(Result::ok);

    let t: usize = lines.next().unwrap().trim().parse().unwrap();
    for _ in 0..t {
        let line = lines.next().unwrap();
        let mut tokens = line.split_whitespace();
        let n: usize = tokens.next().unwrap().parse().unwrap();
        let k: usize = tokens.next().unwrap().parse().unwrap();
        let ps1: usize = tokens.next().unwrap().parse::<usize>().unwrap() - 1;
        let ps2: usize = tokens.next().unwrap().parse::<usize>().unwrap() - 1;

        let p: Vec<usize> = lines.next().unwrap().split_whitespace()
            .map(|x| x.parse::<usize>().unwrap() - 1)
            .collect();
        let a: Vec<i64> = lines.next().unwrap().split_whitespace()
            .map(|x| x.parse::<i64>().unwrap())
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