use std::io::{self, Read};

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
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut lines = input.lines();
    
    let t: usize = lines.next().unwrap().trim().parse().unwrap();
    for _ in 0..t {
        let nkps: Vec<usize> = lines.next().unwrap().split_whitespace()
            .map(|x| x.parse().unwrap()).collect();
        let n = nkps[0];
        let k = nkps[1];
        let ps1 = nkps[2];
        let ps2 = nkps[3];
        
        let p: Vec<usize> = lines.next().unwrap().split_whitespace()
            .map(|x| x.parse().unwrap()).map(|x: usize| x - 1).collect();
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