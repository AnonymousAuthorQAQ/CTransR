use std::io::{self, BufRead};

const N: usize = 2_00005;

fn find(k: usize, p: &mut [usize; N], n: usize) -> usize {
    if k > n || p[k] == k {
        k
    } else {
        p[k] = find(p[k], p, n);
        p[k]
    }
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines().map(|line| line.unwrap());
    
    let t: usize = lines.next().unwrap().trim().parse().unwrap();
    
    for _ in 0..t {
        let line = lines.next().unwrap();
        let mut parts = line.split_whitespace();
        let n: usize = parts.next().unwrap().parse().unwrap();
        let x: usize = parts.next().unwrap().parse().unwrap();
        let y: usize = parts.next().unwrap().parse().unwrap();
        let a: Vec<char> = parts.next().unwrap().chars().collect();
        
        let mut p: [usize; N] = [0; N];
        let mut t: [usize; N] = [0; N];
        let mut ans: [usize; N] = [0; N];
        let mut z: [usize; N] = [0; N];
        
        z[1] = n;
        let mut i = 2;
        let mut j = 0;
        
        while i <= n {
            if j + z[j] > i {
                z[i] = std::cmp::min(z[i - j + 1], j + z[j] - i);
            }
            
            let mut k = i + z[i];
            while a[k] == a[1 + z[i]] {
                z[i] += 1;
                k += 1;
            }
            
            if i + z[i] > j + z[j] {
                j = i;
            }
            
            i += 1;
        }
        
        let mut t_sorted: Vec<usize> = (1..=n).collect();
        t_sorted.sort_by_key(|&i| z[i]);
        
        let mut k = 1;
        
        for i in 1..=n {
            let mut c = 0;
            while k <= n && z[t_sorted[k]] < i {
                p[t_sorted[k]] = t_sorted[k] + 1;
                k += 1;
            }
            
            let mut j = 1;
            while j <= n {
                j = find(j + i, &mut p, n);
                c += 1;
            }
            
            ans[c] = i;
        }
        
        for i in (1..=n).rev() {
            ans[i] = std::cmp::max(ans[i], ans[i + 1]);
        }
        
        for i in x..=y {
            print!("{} ", ans[i]);
        }
        println!();
    }
}