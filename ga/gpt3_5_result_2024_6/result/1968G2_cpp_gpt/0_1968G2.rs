use std::io::{self, BufRead};

const N: usize = 2e5 + 3;

fn find(k: usize, p: &mut [usize;N]) -> usize {
    if k > N || p[k] == k {
        k
    } else {
        p[k] = find(p[k], p);
        p[k]
    }
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    let t: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    
    for _ in 0..t {
        let line = lines.next().unwrap().unwrap();
        let values: Vec<usize> = line.split_whitespace().map(|x| x.parse().unwrap()).collect();
        let n = values[0];
        let x = values[1];
        let y = values[2];
        let a = values[3].as_bytes();
        
        let mut p = [0; N];
        let mut t = [0; N];
        let mut ans = [0; N];
        let mut z = [0; N];
        
        z[1] = n;
        
        for i in 2..=n {
            let mut j = 0;
            let mut zi = 0;
            if j + z[j] > i {
                z[i] = std::cmp::min(z[i-j+1], j + z[j] - i);
            }
            while a[i + zi] == a[1 + zi] {
                zi += 1;
            }
            z[i] = zi;
            
            if i + z[i] > j + z[j] {
                j = i;
            }
        }
        
        let mut t_sorted = (1..=n).collect::<Vec<_>>();
        t_sorted.sort_by(|&i, &j| z[i].cmp(&z[j]));
        
        let mut k = 1;
        
        for i in 1..=n {
            let mut c = 0;
            while k <= n && z[t_sorted[k]] < i {
                p[t_sorted[k]] = t_sorted[k] + 1;
                k += 1;
            }
            for j in 1..=n {
                let mut current_j = j;
                while current_j + i <= n {
                    current_j = find(current_j + i, &mut p);
                    c += 1;
                }
            }
            ans[c] = i;
        }
        
        for i in (1..=n).rev() {
            ans[i] = std::cmp::max(ans[i], ans[i+1]);
        }
        
        for i in x..=y {
            print!("{} ", ans[i]);
        }
        
        println!();
    }
}