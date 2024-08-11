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
        pos = p[pos];  // move to the next position
        k -= 1;       // decrement the steps left
    }

    mx
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();
    
    let t: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    
    for _ in 0..t {
        let first_line = lines.next().unwrap().unwrap();
        let mut parts = first_line.split_whitespace();
        let n: usize = parts.next().unwrap().parse().unwrap();
        let k: i64 = parts.next().unwrap().parse().unwrap();
        let ps1: usize = parts.next().unwrap().parse().unwrap();
        let ps2: usize = parts.next().unwrap().parse().unwrap();
        
        let mut p: Vec<usize> = Vec::with_capacity(n);
        let mut a: Vec<i64> = Vec::with_capacity(n);
        
        let second_line = lines.next().unwrap().unwrap();
        for num in second_line.split_whitespace() {
            p.push(num.parse::<usize>().unwrap() - 1);  // converting to 0-based index
        }

        let third_line = lines.next().unwrap().unwrap();
        for num in third_line.split_whitespace() {
            a.push(num.parse::<i64>().unwrap());
        }

        let ans1 = calculate(&p, &a, ps1 - 1, k);  // convert to 0-based
        let ans2 = calculate(&p, &a, ps2 - 1, k);  // convert to 0-based
        
        if ans1 == ans2 {
            println!("Draw");
        } else if ans1 > ans2 {
            println!("Bodya");
        } else {
            println!("Sasha");
        }
    }
}