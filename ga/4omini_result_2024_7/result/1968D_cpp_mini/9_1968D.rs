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
        let mut inputs = line.split_whitespace();
        let n: usize = inputs.next().unwrap().parse().unwrap();
        let k: i64 = inputs.next().unwrap().parse().unwrap();
        let ps1: usize = inputs.next().unwrap().parse::<usize>().unwrap() - 1;
        let ps2: usize = inputs.next().unwrap().parse::<usize>().unwrap() - 1;

        let mut p = Vec::new();
        let mut a = Vec::new();

        // Read p array
        let line = lines.next().unwrap().unwrap();
        for x in line.split_whitespace() {
            p.push(x.parse::<usize>().unwrap() - 1);
        }

        // Read a array
        let line = lines.next().unwrap().unwrap();
        for x in line.split_whitespace() {
            a.push(x.parse::<i64>().unwrap());
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