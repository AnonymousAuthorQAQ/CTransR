use std::io::{self, Read};
use std::cmp::max;

fn calculate(p: &[i32], a: &[i32], pos: usize, k: i32) -> i64 {
    let n = p.len();
    let mut mx = 0;
    let mut cur = 0;
    let mut vis = vec![false; n];
    while !vis[pos] && k > 0 {
        vis[pos] = true;
        mx = max(mx, cur + (k as i64) * (a[pos] as i64));
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

    let t: i32 = lines.next().unwrap().parse().unwrap();
    while t > 0 {
        let n: i32 = lines.next().unwrap().parse().unwrap();
        let k: i32 = lines.next().unwrap().parse().unwrap();
        let ps1: i32 = lines.next().unwrap().parse().unwrap();
        let ps2: i32 = lines.next().unwrap().parse().unwrap();
        let p: Vec<i32> = lines.next().unwrap().split_whitespace().map(|s| s.parse().unwrap()).collect();
        let a: Vec<i32> = lines.next().unwrap().split_whitespace().map(|s| s.parse().unwrap()).collect();

        let ans1 = calculate(&p, &a, (ps1 - 1) as usize, k);
        let ans2 = calculate(&p, &a, (ps2 - 1) as usize, k);

        if ans1 == ans2 {
            println!("Draw");
        } else if ans1 > ans2 {
            println!("Bodya");
        } else {
            println!("Sasha");
        }

        t -= 1;
    }
}