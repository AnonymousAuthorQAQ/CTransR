use std::io::{self, Read};

fn calculate(p: &[usize], a: &[i32], pos: usize, k: usize) -> i64 {
    let mut mx = 0;
    let mut cur = 0;
    let mut vis = vec![false; p.len()];
    let mut pos = pos;
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
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut lines = input.lines();

    let t: usize = lines.next().unwrap().parse().unwrap();
    for _ in 0..t {
        let mut line = lines.next().unwrap().split_whitespace();
        let n: usize = line.next().unwrap().parse().unwrap();
        let k: usize = line.next().unwrap().parse().unwrap();
        let ps1: usize = line.next().unwrap().parse().unwrap();
        let ps2: usize = line.next().unwrap().parse().unwrap();

        let p: Vec<usize> = (0..n).map(|_| line.next().unwrap().parse().unwrap() - 1).collect();
        let a: Vec<i32> = (0..n).map(|_| line.next().unwrap().parse().unwrap()).collect();

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