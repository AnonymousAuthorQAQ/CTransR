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
    let handle = stdin.lock();
    let reader = handle.lines();
    
    let mut iter = reader.map(|line| line.unwrap()).peekable();
    
    let t: usize = iter.next().unwrap().parse().unwrap();
    for _ in 0..t {
        let line = iter.next().unwrap();
        let mut nums = line.split_whitespace().map(|x| x.parse::<i64>().unwrap());
        let n = nums.next().unwrap() as usize;
        let k = nums.next().unwrap();
        let ps1 = nums.next().unwrap() as usize - 1; // Convert ps1 to 0-based index
        let ps2 = nums.next().unwrap() as usize - 1; // Convert ps2 to 0-based index
        
        let mut p = vec![0; n];
        for i in 0..n {
            let line = iter.next().unwrap();
            p[i] = line.parse::<usize>().unwrap() - 1; // Convert to 0-based index
        }
        
        let mut a = vec![0; n];
        for i in 0..n {
            let line = iter.next().unwrap();
            a[i] = line.parse::<i64>().unwrap();
        }
        
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