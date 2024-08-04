use std::io::{self, Write};
use std::cmp::max;

fn calculate(p: &Vec<usize>, a: &Vec<usize>, mut pos: usize, mut k: usize) -> u64 {
    let n = p.len();
    let mut mx = 0;
    let mut cur = 0;
    let mut vis = vec![false; n];
    while !vis[pos] && k > 0 {
        vis[pos] = true;
        mx = max(mx, cur + (k as u64 * a[pos] as u64));
        cur += a[pos];
        pos = p[pos];
        k -= 1;
    }
    mx
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let t: usize = input.trim().parse().unwrap();
    
    for _ in 0..t {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let mut iter = input.trim().split_whitespace();
        let n: usize = iter.next().unwrap().parse().unwrap();
        let k: usize = iter.next().unwrap().parse().unwrap();
        let ps1: usize = iter.next().unwrap().parse().unwrap();
        let ps2: usize = iter.next().unwrap().parse().unwrap();
        
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let p: Vec<usize> = input.split_whitespace().map(|x| x.parse().unwrap() - 1).collect();
        
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let a: Vec<usize> = input.split_whitespace().map(|x| x.parse().unwrap()).collect();
        
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