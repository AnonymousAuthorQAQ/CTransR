use std::io::{self, Read};

fn calculate(p: &[usize], a: &[i64], pos: usize, k: usize) -> i64 {
    let n = p.len();
    let mut mx = 0;
    let mut cur = 0;
    let mut vis = vec![false; n];
    let mut current_pos = pos;
    let mut steps = k;

    while !vis[current_pos] && steps > 0 {
        vis[current_pos] = true;
        mx = mx.max(cur + (steps as i64 * a[current_pos]));
        cur += a[current_pos];
        current_pos = p[current_pos];
        steps -= 1;
    }
    
    mx
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut lines = input.lines();

    let t: usize = lines.next().unwrap().parse().unwrap();
    
    for _ in 0..t {
        let (n, k, ps1, ps2): (usize, usize, usize, usize) = {
            let line = lines.next().unwrap();
            let mut it = line.split_whitespace();
            (
                it.next().unwrap().parse().unwrap(),
                it.next().unwrap().parse().unwrap(),
                it.next().unwrap().parse().unwrap(),
                it.next().unwrap().parse().unwrap(),
            )
        };

        let p: Vec<usize> = lines.next().unwrap().split_whitespace()
            .map(|x| x.parse::<usize>().unwrap() - 1) // converting to 0-based indexing
            .collect();
        
        let a: Vec<i64> = lines.next().unwrap().split_whitespace()
            .map(|x| x.parse::<i64>().unwrap())
            .collect();

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