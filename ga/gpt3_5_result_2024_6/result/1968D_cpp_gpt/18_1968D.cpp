use std::io::{self, Read};

fn calculate(p: &Vec<usize>, a: &Vec<i64>, pos: usize, k: i64) -> i64 {
    let n = p.len();
    let mut mx = 0;
    let mut cur = 0;
    let mut vis = vec![false; n];
    let mut curr_pos = pos;
    
    let mut remaining_steps = k;
    while !vis[curr_pos] && remaining_steps > 0 {
        vis[curr_pos] = true;
        mx = mx.max(cur + remaining_steps * a[curr_pos] as i64);
        cur += a[curr_pos] as i64;
        curr_pos = p[curr_pos];
        remaining_steps -= 1;
    }
    mx
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut lines = input.lines();
    
    let t: usize = lines.next().unwrap().trim().parse().unwrap();
    
    for _ in 0..t {
        let line: Vec<usize> = lines.next().unwrap().split_whitespace()
                                    .map(|x| x.parse().unwrap()).collect();
        let n = line[0];
        let k = line[1];
        let ps1 = line[2] - 1;
        let ps2 = line[3] - 1;
        
        let p: Vec<usize> = lines.next().unwrap().split_whitespace()
                                .map(|x| x.parse().unwrap() - 1).collect();
        let a: Vec<i64> = lines.next().unwrap().split_whitespace()
                                .map(|x| x.parse().unwrap()).collect();
        
        let ans1 = calculate(&p, &a, ps1, k as i64);
        let ans2 = calculate(&p, &a, ps2, k as i64);
        
        if ans1 == ans2 {
            println!("Draw");
        } else if ans1 > ans2 {
            println!("Bodya");
        } else {
            println!("Sasha");
        }
    }
}