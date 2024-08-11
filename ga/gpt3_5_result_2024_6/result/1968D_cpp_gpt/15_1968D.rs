use std::io::{self, BufRead};

fn calculate(p: &Vec<usize>, a: &Vec<i64>, mut pos: usize, mut k: usize) -> i64 {
    let n = p.len();
    let mut mx = 0;
    let mut cur = 0;
    let mut vis = vec![false; n];
    
    while !vis[pos] && k > 0 {
        vis[pos] = true;
        mx = mx.max(cur + (k as i64 * a[pos]));
        cur += a[pos];
        pos = p[pos];
        k -= 1;
    }
    
    mx
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines().map(|line| line.unwrap());
    
    let t: usize = lines.next().unwrap().trim().parse().unwrap();
    
    for _ in 0..t {
        let params: Vec<usize> = lines.next().unwrap().split_whitespace().map(|x| x.parse().unwrap()).collect();
        let n = params[0];
        let k = params[1];
        let ps1 = params[2];
        let ps2 = params[3];
        
        let p: Vec<usize> = lines.next().unwrap().split_whitespace().map(|x| x.parse::<usize>().unwrap() - 1).collect();
        let a: Vec<i64> = lines.next().unwrap().split_whitespace().map(|x| x.parse().unwrap()).collect();
        
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