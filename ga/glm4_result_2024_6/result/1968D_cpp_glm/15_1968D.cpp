use std::io;

fn calculate(p: &Vec<usize>, a: &Vec<usize>, pos: usize, k: usize) -> u64 {
    let n = p.len();
    let mut mx = 0;
    let mut cur = 0;
    let mut vis = vec![false; n];
    
    while !vis[pos] && k > 0 {
        vis[pos] = true;
        mx = std::cmp::max(mx, cur + (k as u64) * (a[pos] as u64));
        cur += a[pos];
        pos = p[pos];
        k -= 1;
    }
    
    mx
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let t: usize = input.trim().parse().expect("Failed to parse t");

    for _ in 0..t {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let mut iter = input.trim().split_whitespace();
        let n: usize = iter.next().unwrap().parse().expect("Failed to parse n");
        let k: usize = iter.next().unwrap().parse().expect("Failed to parse k");
        let ps1: usize = iter.next().unwrap().parse().expect("Failed to parse ps1");
        let ps2: usize = iter.next().unwrap().parse().expect("Failed to parse ps2");
        
        let mut p = Vec::with_capacity(n);
        let mut a = Vec::with_capacity(n);
        
        for _ in 0..n {
            p.push(iter.next().unwrap().parse().expect("Failed to parse p[i]"));
        }
        
        for _ in 0..n {
            a.push(iter.next().unwrap().parse().expect("Failed to parse a[i]"));
        }
        
        for i in 0..n {
            p[i] -= 1;
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