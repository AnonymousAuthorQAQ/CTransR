use std::io::{self, Read};

fn calculate(p: &Vec<usize>, a: &Vec<usize>, mut pos: usize, mut k: i64) -> i64 {
    let n = p.len();
    let mut mx = 0;
    let mut cur = 0;
    let mut vis = vec![false; n];
    
    while !vis[pos] && k > 0 {
        vis[pos] = true;
        mx = mx.max(cur + (k * a[pos] as i64));
        cur += a[pos] as i64;
        pos = p[pos];
        k -= 1;
    }
    
    return mx;
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    
    let mut lines = input.lines();
    let t: usize = lines.next().unwrap().trim().parse().unwrap();

    for _ in 0..t {
        let mut input_line = lines.next().unwrap().trim().split_whitespace();
        let n: usize = input_line.next().unwrap().parse().unwrap();
        let k: i64 = input_line.next().unwrap().parse().unwrap();
        let ps1: usize = input_line.next().unwrap().parse().unwrap();
        let ps2: usize = input_line.next().unwrap().parse().unwrap();
        
        let p: Vec<usize> = lines.next().unwrap().trim().split_whitespace().map(|x| x.parse().unwrap() - 1).collect();
        let a: Vec<usize> = lines.next().unwrap().trim().split_whitespace().map(|x| x.parse().unwrap()).collect();
        
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