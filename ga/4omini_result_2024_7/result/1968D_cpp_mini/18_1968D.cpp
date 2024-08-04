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
    let reader = stdin.lock();
    let mut lines = reader.lines();
    let t: usize = lines.next().unwrap().unwrap().parse().unwrap();

    for _ in 0..t {
        let line = lines.next().unwrap().unwrap();
        let mut nums = line.split_whitespace().map(|x| x.parse::<i32>().unwrap());
        let n = nums.next().unwrap() as usize;
        let k = nums.next().unwrap();
        let ps1 = nums.next().unwrap() - 1;
        let ps2 = nums.next().unwrap() - 1;
        
        let mut p: Vec<usize> = Vec::with_capacity(n);
        let mut a: Vec<i64> = Vec::with_capacity(n);

        // Reading the p array
        let p_line = lines.next().unwrap().unwrap();
        for value in p_line.split_whitespace() {
            p.push(value.parse::<usize>().unwrap() - 1);
        }

        // Reading the a array
        let a_line = lines.next().unwrap().unwrap();
        for value in a_line.split_whitespace() {
            a.push(value.parse::<i64>().unwrap());
        }

        let ans1 = calculate(&p, &a, ps1 as usize, k as i64);
        let ans2 = calculate(&p, &a, ps2 as usize, k as i64);
        
        if ans1 == ans2 {
            println!("Draw");
        } else if ans1 > ans2 {
            println!("Bodya");
        } else {
            println!("Sasha");
        }
    }
}