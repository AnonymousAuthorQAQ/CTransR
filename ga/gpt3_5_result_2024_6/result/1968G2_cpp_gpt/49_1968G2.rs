use std::io::{self, BufRead};

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
    let stdin = io::stdin();
    let mut iterator = stdin.lock().lines().map(|x| x.unwrap());
    
    let t: usize = iterator.next().unwrap().trim().parse().unwrap();
    
    for _ in 0..t {
        let input: Vec<usize> = iterator
            .next()
            .unwrap()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        let (n, x, y) = (input[0], input[1], input[2]);
        let a: Vec<char> = iterator.next().unwrap().chars().collect();
        
        let mut z = vec![0; n + 1];
        let mut p = (0..=n).collect::<Vec<_>>();
        let mut t = (0..=n).collect::<Vec<_>>();
        let mut ans = vec![0; n + 1];
        
        z[1] = n;
        
        for i in 2..=n {
            let mut j = 0;
            if j + z[j] > i {
                z[i] = z[i-j+1].min(j + z[j] - i);
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
            ans[i] = ans[i].max(ans[i + 1]);
        }
        
        for i in x..=y {
            print!("{} ", ans[i]);
        }
        
        println!();
    }
}