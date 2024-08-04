use std::io::{self, Read};
use std::cmp::Ordering;
use std::collections::HashMap;

const N: usize = 2e5 as usize + 3;
const MOD: i64 = 1e9 as i64 + 7;

fn find(p: &mut Vec<usize>, i: usize) -> usize {
    if i > N || p[i] == i {
        i
    } else {
        p[i] = find(p, p[i]);
        p[i]
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut lines = input.lines();
    let t = lines.next().unwrap().parse::<usize>().unwrap();
    
    for _ in 0..t {
        let mut nums = lines.next().unwrap().split_whitespace().map(|s| s.parse::<usize>().unwrap());
        let n = nums.next().unwrap();
        let x = nums.next().unwrap();
        let y = nums.next().unwrap();
        let s = lines.next().unwrap();
        let mut p = vec![0; N + 1];
        for i in 1..=n + 1 {
            p[i] = i;
        }
        
        let mut z = vec![0; N + 1];
        z[1] = n;
        for i in 2..=n {
            let mut j = 0;
            while j + z[j] > i {
                z[i] = std::cmp::min(z[i - j + 1], j + z[j] - i);
            }
            while i + z[i] > j + z[j] {
                j = i;
            }
        }
        
        let mut t = (1..=n).collect::<Vec<usize>>();
        t.sort_by(|&i, &j| z[i].cmp(&z[j]));
        
        let mut ans = vec![0; N + 1];
        for i in 1..=n {
            let mut k = 1;
            while k <= n && z[t[k]] < i {
                p[t[k]] = t[k] + 1;
                k += 1;
            }
            
            let mut c = 0;
            for j in 1..=n {
                if find(&mut p, j + i) <= n {
                    c += 1;
                }
            }
            ans[c] = i;
        }
        
        for i in (1..=n).rev() {
            ans[i] = std::cmp::max(ans[i], ans[i + 1]);
        }
        
        for i in x..=y {
            print!("{} ", ans[i]);
        }
        println!("");
    }
}