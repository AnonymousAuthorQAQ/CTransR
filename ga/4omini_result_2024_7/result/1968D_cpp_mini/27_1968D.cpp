use std::io::{self, BufRead};

fn calculate(p: &Vec<usize>, a: &Vec<i64>, mut pos: usize, mut k: i64) -> i64 {
    let n = p.len();
    let mut mx = 0;
    let mut cur = 0;
    let mut vis = vec![false; n];

    while !vis[pos] && k > 0 {
        vis[pos] = true;
        mx = mx.max(cur + k * a[pos]);
        cur += a[pos];
        pos = p[pos];
        k -= 1;
    }
    
    mx
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    let t: usize = lines.next().unwrap().unwrap().parse().unwrap();
    
    for _ in 0..t {
        let line = lines.next().unwrap().unwrap();
        let mut input = line.split_whitespace();
        let n: usize = input.next().unwrap().parse().unwrap();
        let k: i64 = input.next().unwrap().parse().unwrap();
        let ps1: usize = input.next().unwrap().parse().unwrap() - 1; // convert to 0-indexed
        let ps2: usize = input.next().unwrap().parse().unwrap() - 1; // convert to 0-indexed

        let mut p = Vec::with_capacity(n);
        let mut a = Vec::with_capacity(n);

        line = lines.next().unwrap().unwrap();
        let mut positions = line.split_whitespace();
        for _ in 0..n {
            p.push(positions.next().unwrap().parse::<usize>().unwrap() - 1);  // convert to 0-indexed
        }

        line = lines.next().unwrap().unwrap();
        let mut values = line.split_whitespace();
        for _ in 0..n {
            a.push(values.next().unwrap().parse::<i64>().unwrap());
        }

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