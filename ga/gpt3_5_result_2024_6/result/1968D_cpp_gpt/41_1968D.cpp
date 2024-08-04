use std::io::{self, BufRead};

fn calculate(p: &Vec<usize>, a: &Vec<usize>, mut pos: usize, mut k: usize) -> u64 {
    let n = p.len();
    let mut mx = 0;
    let mut cur = 0;
    let mut vis = vec![false; n];
    
    while !vis[pos] && k > 0 {
        vis[pos] = true;
        mx = mx.max(cur + (k as u64 * a[pos] as u64));
        cur += a[pos];
        pos = p[pos];
        k -= 1;
    }
    mx
}

fn main() {
    let stdin = io::stdin();
    let mut input = stdin.lock().lines().map(|line| line.unwrap());
    
    let t: usize = input.next().unwrap().trim().parse().unwrap();
    
    for _ in 0..t {
        let line: String = input.next().unwrap().trim().to_string();
        let mut parts = line.split_whitespace().map(|x| x.parse().unwrap());
        let n: usize = parts.next().unwrap();
        let k: usize = parts.next().unwrap();
        let ps1: usize = parts.next().unwrap();
        let ps2: usize = parts.next().unwrap();
        
        let line: String = input.next().unwrap().trim().to_string();
        let p: Vec<usize> = line.split_whitespace().map(|x| x.parse().unwrap()).collect();
        
        let line: String = input.next().unwrap().trim().to_string();
        let a: Vec<usize> = line.split_whitespace().map(|x| x.parse().unwrap()).collect();
        
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