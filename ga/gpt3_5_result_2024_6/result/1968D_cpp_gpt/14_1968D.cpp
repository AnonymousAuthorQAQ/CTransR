use std::io::{self, Read};

fn calculate(p: &Vec<usize>, a: &Vec<usize>, mut pos: usize, mut k: i32) -> i64 {
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
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_whitespace();

    let t: usize = input.next().unwrap().parse().unwrap();
    for _ in 0..t {
        let n: usize = input.next().unwrap().parse().unwrap();
        let k: i32 = input.next().unwrap().parse().unwrap();
        let ps1: usize = input.next().unwrap().parse().unwrap();
        let ps2: usize = input.next().unwrap().parse().unwrap();
        
        let mut p = Vec::with_capacity(n);
        let mut a = Vec::with_capacity(n);
        
        for _ in 0..n {
            let val: usize = input.next().unwrap().parse().unwrap();
            p.push(val - 1);
        }
        
        for _ in 0..n {
            let val: usize = input.next().unwrap().parse().unwrap();
            a.push(val);
        }
        
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