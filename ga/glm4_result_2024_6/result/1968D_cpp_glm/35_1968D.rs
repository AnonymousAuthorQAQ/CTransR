use std::io::{self, Read};

fn calculate(p: &[usize], a: &[i32], pos: usize, k: usize) -> i64 {
    let n = p.len();
    let mut mx = 0;
    let mut cur = 0;
    let mut vis = vec![false; n];
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
        let n: usize;
        let k: usize;
        let ps1: usize;
        let ps2: usize;
        (n, k, ps1, ps2) = lines.next().unwrap().split_whitespace().map(|s| s.parse().unwrap()).collect_tuple().unwrap();

        let p: Vec<usize> = lines.next().unwrap().split_whitespace().map(|s| s.parse().unwrap()).collect();
        let a: Vec<i32> = lines.next().unwrap().split_whitespace().map(|s| s.parse().unwrap()).collect();

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