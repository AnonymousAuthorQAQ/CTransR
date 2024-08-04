use std::io;
use std::io::prelude::*;

fn calculate(p: &Vec<i32>, a: &Vec<i32>, pos: usize, k: i32) -> i64 {
    let n = p.len();
    let mut mx = 0;
    let mut cur = 0;
    let mut vis = vec![false; n];
    while !vis[pos] && k > 0 {
        vis[pos] = true;
        mx = mx.max(cur + (k as i64) * (a[pos] as i64));
        cur += a[pos];
        pos = p[pos];
        k -= 1;
    }
    mx
}

fn main() {
    let stdin = io::stdin();
    let mut stdin = stdin.lock();
    let mut t = 1;
    loop {
        let n;
        let k;
        let ps1;
        let ps2;
        if let Ok(line) = stdin.read_line() {
            let mut words = line.split_whitespace();
            if let (Some(n_str), Some(k_str), Some(ps1_str), Some(ps2_str)) = (words.next(), words.next(), words.next(), words.next()) {
                if let (Ok(n), Ok(k), Ok(ps1), Ok(ps2)) = (n_str.parse::<usize>(), k_str.parse::<i32>(), ps1_str.parse::<usize>(), ps2_str.parse::<usize>()) {
                    t = n;
                    k = k;
                    ps1 = ps1 - 1;
                    ps2 = ps2 - 1;
                    let mut p = Vec::with_capacity(n);
                    let mut a = Vec::with_capacity(n);
                    for _ in 0..n {
                        if let Ok(line) = stdin.read_line() {
                            let mut words = line.split_whitespace();
                            if let (Some(p_str), Some(a_str)) = (words.next(), words.next()) {
                                if let (Ok(p_val), Ok(a_val)) = (p_str.parse::<i32>(), a_str.parse::<i32>()) {
                                    p.push(p_val - 1);
                                    a.push(a_val);
                                } else {
                                    break;
                                }
                            } else {
                                break;
                            }
                        } else {
                            break;
                        }
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
                } else {
                    break;
                }
            } else {
                break;
            }
        } else {
            break;
        }
    }
}