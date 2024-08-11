use std::io::{self, Write};

fn calculate(p: &Vec<usize>, a: &Vec<i64>, pos: usize, k: i64) -> i64 {
    let n = p.len();
    let mut mx = 0;
    let mut cur = 0;
    let mut vis = vec![false; n];
    let mut pos = pos;
    let mut k = k;
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
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let t: usize = input.trim().parse().unwrap();
    
    for _ in 0..t {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let parts: Vec<usize> = input.split_whitespace().map(|x| x.trim().parse().unwrap()).collect();
        let n = parts[0];
        let k = parts[1] as i64;
        let ps1 = parts[2];
        let ps2 = parts[3];
        
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let p: Vec<usize> = input.split_whitespace().map(|x| x.trim().parse().unwrap()).collect();
        
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let a: Vec<i64> = input.split_whitespace().map(|x| x.trim().parse().unwrap()).collect();
        
        let ans1 = calculate(&p, &a, ps1-1, k);
        let ans2 = calculate(&p, &a, ps2-1, k);
        
        if ans1 == ans2 {
            println!("Draw");
        } else if ans1 > ans2 {
            println!("Bodya");
        } else {
            println!("Sasha");
        }
    }
}