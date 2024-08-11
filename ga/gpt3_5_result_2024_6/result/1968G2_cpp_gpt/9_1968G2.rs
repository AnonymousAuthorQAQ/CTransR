use std::io::{self, Read};
use std::cmp;

const N: usize = 2e5 as usize + 3;

fn find(k: usize, p: &mut [usize; N]) -> usize {
    if k > p.len() || p[k] == k {
        k
    } else {
        p[k] = find(p[k], p);
        p[k]
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    
    let mut lines = input.trim().split('\n');
    let t: usize = lines.next().unwrap().trim().parse().unwrap();
    
    for _ in 0..t {
        let line = lines.next().unwrap();
        let mut parts = line.trim().split_whitespace();
        let n: usize = parts.next().unwrap().parse().unwrap();
        let x: usize = parts.next().unwrap().parse().unwrap();
        let y: usize = parts.next().unwrap().parse().unwrap();
        let a: Vec<char> = parts.next().unwrap().chars().collect();
        
        let mut z = vec![0; N];
        let mut p = (1..=n+1).collect::<Vec<usize>>();
        let mut t = (1..=n).collect::<Vec<usize>>();
        let mut ans = vec![0; N];
        
        z[1] = n;
        for i in 2..=n {
            let (mut j, mut k) = (0, 0);
            for j in 2..=n {
                if j + z[j] > i {
                    z[i] = cmp::min(z[i - j + 1], j + z[j] - i);
                } else {
                    break;
                }
            }
            while a[i + z[i]] == a[1 + z[i]] {
                z[i] += 1;
            }
            if i + z[i] > j + z[j] {
                j = i;
            }
        }
        
        t.sort_by(|&i, &j| z[i].cmp(&z[j]));
        
        let mut k = 1;
        for i in 1..=n {
            let mut c = 0;
            while k <= n && z[t[k]] < i {
                p[t[k]] = t[k] + 1;
                k += 1;
            }
            
            let mut j = 1;
            while j <= n {
                j = find(j + i, &mut p);
                c += 1;
            }
            
            ans[c] = i;
        }
        
        for i in (1..=n).rev() {
            ans[i] = cmp::max(ans[i], ans[i + 1]);
        }
        
        for i in x..=y {
            print!("{} ", ans[i]);
        }
        println!();
    }
}
