use std::io::{self, BufRead};

fn calculate(p: &[usize], a: &[i64], pos: usize, k: usize) -> i64 {
    let n = p.len();
    let mut mx = 0;
    let mut cur = 0;
    let mut vis = vec![false; n];
    let mut moves_remaining = k;
    let mut current_pos = pos;

    while !vis[current_pos] && moves_remaining > 0 {
        vis[current_pos] = true;
        mx = mx.max(cur + (moves_remaining as i64) * a[current_pos]);
        cur += a[current_pos];
        current_pos = p[current_pos];
        moves_remaining -= 1;
    }

    mx
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines().filter_map(Result::ok);

    let t: usize = lines.next().unwrap().trim().parse().unwrap();
    for _ in 0..t {
        let line = lines.next().unwrap();
        let nums: Vec<usize> = line.split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        let n = nums[0];
        let k = nums[1];
        let ps1 = nums[2] - 1; // Convert to 0-based index
        let ps2 = nums[3] - 1; // Convert to 0-based index

        let mut p = Vec::with_capacity(n);
        let mut a = Vec::with_capacity(n);

        let p_line = lines.next().unwrap();
        for x in p_line.split_whitespace() {
            p.push(x.parse::<usize>().unwrap() - 1); // Convert to 0-based index
        }

        let a_line = lines.next().unwrap();
        for x in a_line.split_whitespace() {
            a.push(x.parse::<i64>().unwrap());
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