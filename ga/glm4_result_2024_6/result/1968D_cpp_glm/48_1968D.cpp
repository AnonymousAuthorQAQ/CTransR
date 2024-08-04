use std::io::{self, Read};

fn calculate(p: &Vec<usize>, a: &Vec<usize>, pos: usize, k: usize) -> u64 {
    let n = p.len();
    let mut mx = 0;
    let mut cur = 0;
    let mut vis = vec![false; n];
    while !vis[pos] && k > 0 {
        vis[pos] = true;
        mx = mx.max(cur + (k as u64) * (a[pos] as u64));
        cur += a[pos];
        pos = p[pos];
        k -= 1;
    }
    mx
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut lines = input.lines();

    let t: usize = lines.next().unwrap().parse().unwrap();

    for _ in 0..t {
        let mut parts = lines.next().unwrap().split_whitespace();
        let n: usize = parts.next().unwrap().parse().unwrap();
        let k: usize = parts.next().unwrap().parse().unwrap();
        let ps1: usize = parts.next().unwrap().parse().unwrap();
        let ps2: usize = parts.next().unwrap().parse().unwrap();

        let mut p: Vec<usize> = Vec::with_capacity(n);
        let mut a: Vec<usize> = Vec::with_capacity(n);

        for part in lines.next().unwrap().split_whitespace() {
            p.push(part.parse().unwrap() - 1);
        }

        for part in lines.next().unwrap().split_whitespace() {
            a.push(part.parse().unwrap());
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