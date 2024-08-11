use std::io::{self, BufRead};

fn calculate(p: &Vec<usize>, a: &Vec<i32>, mut pos: usize, mut k: i32) -> i64 {
    let n = p.len();
    let mut mx = 0;
    let mut cur = 0;
    let mut vis = vec![false; n];
    
    while !vis[pos] && k > 0 {
        vis[pos] = true;
        mx = mx.max(cur + (k as i64 * a[pos] as i64));
        cur += a[pos] as i64;
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
        let input: Vec<usize> = lines.next().unwrap()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        
        let n = input[0];
        let k = input[1];
        let ps1 = input[2] - 1;
        let ps2 = input[3] - 1;
        
        let p: Vec<usize> = lines.next().unwrap()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .map(|x: usize| x-1)
            .collect();
        
        let a: Vec<i32> = lines.next().unwrap()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        
        let ans1 = calculate(&p, &a, ps1, k);
        let ans2 = calculate(&p, &a, ps2, k);
        
        if ans1 == ans2 {
            println!("Draw");
        } else if ans1 > ans2 {
            println!("Bodya");
        } else {
            println!("Sasha");
        }
    }
}