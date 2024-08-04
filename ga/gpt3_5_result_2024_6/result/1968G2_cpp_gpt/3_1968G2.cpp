use std::io::{self, BufRead};

const N: usize = 2e5 as usize + 3;

fn find(p: &mut [usize; N], k: usize) -> usize {
    if k > p.len() || p[k] == k {
        k
    } else {
        p[k] = find(p, p[k]);
        p[k]
    }
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    let t: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    
    for _ in 0..t {
        let line = lines.next().unwrap().unwrap();
        let mut parts = line.split_whitespace().map(|x| x.parse::<usize>().unwrap());
        let n = parts.next().unwrap();
        let x = parts.next().unwrap();
        let y = parts.next().unwrap();
        let a = parts.last().unwrap().chars().collect::<Vec<char>>();
        
        let mut z = vec![0; n + 2];
        let mut p = (1..=n + 1).collect::<Vec<usize>>();
        let mut t = (1..=n).collect::<Vec<usize>>();
        let mut ans = vec![0; n + 2];
        
        z[1] = n;
        let mut i = 2;
        let mut j = 0;
        
        while i <= n {
            if j + z[j] > i {
                z[i] = std::cmp::min(z[i - j + 1], j + z[j] - i);
            }
            for _ in 0.. {
                if a[i + z[i]] == a[1 + z[i]] {
                    z[i] += 1;
                } else {
                    break;
                }
            }
            if i + z[i] > j + z[j] {
                j = i;
            }
            i += 1;
        }
        
        t.sort_by(|&i, &j| z[i].cmp(&z[j]));
        let mut k = 1;
        
        for i in 1..=n {
            let mut c = 0;
            while k <= n && z[t[k - 1]] < i {
                p[t[k - 1]] = t[k - 1] + 1;
                k += 1;
            }
            let mut j = 1;
            while j <= n {
                j = find(&mut p, j + i);
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