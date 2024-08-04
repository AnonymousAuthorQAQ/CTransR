use std::io::{self, BufRead};
use std::cmp::Ordering;

const N: usize = 2e5 as usize + 3;
const MAX_N: usize = N + 1;

fn find(p: &mut [usize], k: usize) -> usize {
    if k > N { return k; }
    if p[k] == k { return k; }
    p[k] = find(p, p[k]);
    p[k]
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    let mut t = lines.next().unwrap().unwrap().parse::<usize>().unwrap();
    while t > 0 {
        t -= 1;
        
        let mut input = lines.next().unwrap().unwrap().split_whitespace();
        let n = input.next().unwrap().parse::<usize>().unwrap();
        let x = input.next().unwrap().parse::<usize>().unwrap();
        let y = input.next().unwrap().parse::<usize>().unwrap();
        let a = input.next().unwrap();
        
        let mut p = vec![0; MAX_N];
        let mut z = vec![0; MAX_N];
        let mut ans = vec![0; MAX_N];
        let mut t = (1..=n).collect::<Vec<usize>>();
        
        z[1] = n;
        for i in 2..=n {
            let mut j = 0;
            while j + z[j] > i { j = i - j - z[j] + 1; }
            while a.chars().nth(i + z[i] - 1) == a.chars().nth(1 + z[i] - 1) { z[i] += 1; }
            if i + z[i] > j + z[j] { j = i; }
        }
        
        t.sort_by(|&i, &j| z[i].cmp(&z[j]));
        
        for i in 1..=n {
            let mut k = 1;
            while k <= n && z[t[k]] < i { p[t[k]] = t[k] + 1; k += 1; }
            let mut c = 0;
            for j in 1..=n { c += 1; while find(&mut p, j + i) != j { j = find(&mut p, j + i); } }
            ans[c] = i;
        }
        
        for i in (1..=n).rev() { ans[i] = ans[i].max(ans[i + 1]); }
        
        for i in x..=y { print!("{} ", ans[i]); }
        println!("");
    }
}