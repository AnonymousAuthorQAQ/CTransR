use std::io;

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
    let mut input = String::new();
    stdin.read_line(&mut input).unwrap();
    let mut lines = input.split_whitespace();
    let t: usize = lines.next().unwrap().parse().unwrap();

    for _ in 0..t {
        let mut line = String::new();
        stdin.read_line(&mut line).unwrap();
        let mut nums = line.split_whitespace();
        
        let n: usize = nums.next().unwrap().parse().unwrap();
        let k: i64 = nums.next().unwrap().parse().unwrap();
        let ps1: usize = nums.next().unwrap().parse::<usize>().unwrap() - 1; // Convert to 0-based
        let ps2: usize = nums.next().unwrap().parse::<usize>().unwrap() - 1; // Convert to 0-based
        
        let mut p = Vec::new();
        let mut a = Vec::new();
        
        line.clear();
        stdin.read_line(&mut line).unwrap();
        for val in line.split_whitespace() {
            p.push(val.parse::<usize>().unwrap() - 1); // Convert to 0-based
        }
        
        line.clear();
        stdin.read_line(&mut line).unwrap();
        for val in line.split_whitespace() {
            a.push(val.parse::<i64>().unwrap());
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