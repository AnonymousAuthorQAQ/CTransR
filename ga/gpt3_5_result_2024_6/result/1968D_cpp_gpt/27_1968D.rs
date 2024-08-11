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
    let mut lines = stdin.lock().lines().map(Result::unwrap);
    
    let t: usize = lines.next().unwrap().parse().unwrap();
    
    for _ in 0..t {
        let line: Vec<usize> = lines.next().unwrap().split_whitespace()
                                    .map(|x| x.parse().unwrap()).collect();
        let n = line[0];
        let k = line[1] as i64;
        let ps1 = line[2];
        let ps2 = line[3];
        
        let p: Vec<usize> = lines.next().unwrap().split_whitespace()
                                .map(|x| x.parse().unwrap() - 1).collect();
        let a: Vec<i64> = lines.next().unwrap().split_whitespace()
                                .map(|x| x.parse().unwrap()).collect();
        
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