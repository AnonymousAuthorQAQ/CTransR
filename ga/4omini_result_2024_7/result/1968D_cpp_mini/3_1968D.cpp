use std::io::{self, BufRead};

fn calculate(p: &Vec<i32>, a: &Vec<i32>, pos: usize, k: i32) -> i64 {
    let n = p.len();
    let mut mx = 0;
    let mut cur = 0;
    let mut vis = vec![false; n];
    let mut k = k;

    let mut current_pos = pos;
    while !vis[current_pos] && k > 0 {
        vis[current_pos] = true;
        mx = mx.max(cur as i64 + (k as i64 * a[current_pos] as i64));
        cur += a[current_pos];
        current_pos = p[current_pos] as usize;
        k -= 1;
    }

    mx
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();
    
    let first_line = lines.next().unwrap().unwrap();
    let t: usize = first_line.trim().parse().unwrap();

    for _ in 0..t {
        let second_line = lines.next().unwrap().unwrap();
        let nums: Vec<i32> = second_line.trim().split_whitespace()
                                            .map(|s| s.parse().unwrap())
                                            .collect();

        let n = nums[0];
        let k = nums[1];
        let ps1 = nums[2] - 1; // zero-indexed
        let ps2 = nums[3] - 1; // zero-indexed

        let mut p = Vec::new();
        let mut a = Vec::new();

        let p_line = lines.next().unwrap().unwrap();
        p.extend(p_line.trim().split_whitespace().map(|s| s.parse().unwrap::<i32>() - 1));

        let a_line = lines.next().unwrap().unwrap();
        a.extend(a_line.trim().split_whitespace().map(|s| s.parse().unwrap()));

        let ans1 = calculate(&p, &a, ps1 as usize, k);
        let ans2 = calculate(&p, &a, ps2 as usize, k);

        if ans1 == ans2 {
            println!("Draw");
        } else if ans1 > ans2 {
            println!("Bodya");
        } else {
            println!("Sasha");
        }
    }
}