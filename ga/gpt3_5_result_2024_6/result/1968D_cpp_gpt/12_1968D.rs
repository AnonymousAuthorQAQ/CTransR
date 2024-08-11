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
    let mut input = stdin.lock();
    
    let mut buffer = String::new();
    input.read_line(&mut buffer).unwrap();
    let t: usize = buffer.trim().parse().unwrap();
    
    for _ in 0..t {
        buffer.clear();
        input.read_line(&mut buffer).unwrap();
        let parts: Vec<usize> = buffer
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        let n = parts[0];
        let k = parts[1] as i64;
        
        buffer.clear();
        input.read_line(&mut buffer).unwrap();
        let ps: Vec<usize> = buffer
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        
        buffer.clear();
        input.read_line(&mut buffer).unwrap();
        let a: Vec<i64> = buffer
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        
        let ps1 = ps[0] - 1;
        let ps2 = ps[1] - 1;
        
        let ans1 = calculate(&ps, &a, ps1, k);
        let ans2 = calculate(&ps, &a, ps2, k);
        
        if ans1 == ans2 {
            println!("Draw");
        } else if ans1 > ans2 {
            println!("Bodya");
        } else {
            println!("Sasha");
        }
    }
}