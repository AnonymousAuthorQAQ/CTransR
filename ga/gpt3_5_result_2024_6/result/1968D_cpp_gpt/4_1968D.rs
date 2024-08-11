use std::io::{self, BufRead};

fn calculate(p: &Vec<usize>, a: &Vec<i64>, pos: usize, k: i64) -> i64 {
    let n = p.len();
    let mut mx = 0;
    let mut cur = 0;
    let mut vis = vec![false; n];
    
    let mut current_pos = pos;
    let mut remaining_steps = k;
    
    while !vis[current_pos] && remaining_steps > 0 {
        vis[current_pos] = true;
        mx = mx.max(cur + remaining_steps * a[current_pos]);
        cur += a[current_pos];
        current_pos = p[current_pos];
        remaining_steps -= 1;
    }
    
    mx
}

fn main() {
    let stdin = io::stdin();
    let mut input = stdin.lock();
    
    let mut buffer = String::new();
    input.read_line(&mut buffer).unwrap();
    let t: usize = buffer.trim().parse().unwrap();
    
    for _ in 0..t {
        buffer.clear();
        input.read_line(&mut buffer).unwrap();
        let parts: Vec<usize> = buffer.split_whitespace().map(|x| x.trim().parse().unwrap()).collect();
        let n = parts[0];
        let k = parts[1] as i64;
        let ps1 = parts[2];
        let ps2 = parts[3];
        
        buffer.clear();
        input.read_line(&mut buffer).unwrap();
        let p: Vec<usize> = buffer.split_whitespace().map(|x| x.trim().parse().unwrap()).map(|x: usize| x - 1).collect();
        
        buffer.clear();
        input.read_line(&mut buffer).unwrap();
        let a: Vec<i64> = buffer.split_whitespace().map(|x| x.trim().parse().unwrap()).collect();
        
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