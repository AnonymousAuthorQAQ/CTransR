use std::io::{self, BufRead};

fn calculate(p: &Vec<usize>, a: &Vec<i64>, pos: usize, k: i64) -> i64 {
    let n = p.len();
    let mut mx = 0;
    let mut cur = 0;
    let mut vis = vec![false; n];
    
    let mut curr_pos = pos;
    let mut moves_left = k;
    
    while !vis[curr_pos] && moves_left > 0 {
        vis[curr_pos] = true;
        mx = mx.max(cur + moves_left * a[curr_pos]);
        cur += a[curr_pos];
        curr_pos = p[curr_pos];
        moves_left -= 1;
    }
    mx
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines().map(|line| line.unwrap());

    let t: usize = lines.next().unwrap().parse().unwrap();
    
    for _ in 0..t {
        let input: Vec<usize> = lines.next().unwrap().split_whitespace()
            .map(|x| x.parse().unwrap()).collect();
        let n = input[0];
        let k = input[1];
        let ps1 = input[2] - 1;
        let ps2 = input[3] - 1;

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