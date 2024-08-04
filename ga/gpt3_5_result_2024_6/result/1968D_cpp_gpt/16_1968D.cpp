use std::io::{self, BufRead};

fn calculate(p: &Vec<usize>, a: &Vec<i64>, mut pos: usize, mut k: i64) -> i64 {
    let n = p.len();
    let mut mx = 0;
    let mut cur = 0;
    let mut vis = vec![false; n];
    
    while !vis[pos] && k > 0 {
        vis[pos] = true;
        mx = mx.max(cur + k * a[pos] as i64);
        cur += a[pos];
        pos = p[pos];
        k -= 1;
    }
    mx
}

fn main() {
    let stdin = io::stdin();
    
    let t: usize = stdin.lock().lines().next().unwrap().unwrap().trim().parse().unwrap();
    
    for _ in 0..t {
        let mut input_line = stdin.lock().lines().next().unwrap().unwrap();
        let mut words = input_line.trim().split_whitespace();
        
        let n: usize = words.next().unwrap().parse().unwrap();
        let k: i64 = words.next().unwrap().parse().unwrap();
        let ps1: usize = words.next().unwrap().parse().unwrap();
        let ps2: usize = words.next().unwrap().parse().unwrap();
        
        input_line = stdin.lock().lines().next().unwrap().unwrap();
        let p: Vec<usize> = input_line.trim().split_whitespace().map(|x| x.parse().unwrap()).collect();
        
        let input_line = stdin.lock().lines().next().unwrap().unwrap();
        let a: Vec<i64> = input_line.trim().split_whitespace().map(|x| x.parse().unwrap()).collect();
        
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