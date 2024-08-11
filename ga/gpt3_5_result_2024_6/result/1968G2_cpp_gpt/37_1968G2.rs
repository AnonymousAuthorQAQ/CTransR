use std::cmp::min;
use std::collections::HashMap;
use std::io::{self, BufRead};

const N: usize = 2e5 + 3;

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
    let mut lines = stdin.lock().lines();
    
    let t: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    for _ in 0..t {
        let input_line: Vec<usize> = lines
            .next()
            .unwrap()
            .unwrap()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        let n = input_line[0];
        let x = input_line[1];
        let y = input_line[2];
        let a: Vec<char> = input_line[3].chars().collect();

        let mut z = vec![0; N];
        let mut p = [0; N];
        let mut t = [0; N];
        let mut ans = [0; N];
        let mut count_map: HashMap<usize, usize> = HashMap::new();

        p[0] = 0;
        t[0] = 0;
        ans[0] = 0;
        z[0] = 0;
        
        z[1] = n;
        for i in 2..n+1 {
            let mut j = 0;
            if j + z[j] > i {
                z[i] = min(z[j - i + 1], j + z[j] - i);
            }
            while a[i + z[i]] == a[1 + z[i]] {
                z[i] += 1;
            }
            if i + z[i] > j + z[j] {
                j = i;
            }
        }

        let mut t_sorted: Vec<usize> = (1..n+1).collect();
        t_sorted.sort_by(|&i, &j| z[i].cmp(&z[j]));

        let mut k = 1;
        for i in 1..n+1 {
            let mut c = 0;
            while k <= n && z[t_sorted[k]] < i {
                p[t_sorted[k]] = t_sorted[k] + 1;
                k += 1;
            }
            let mut j = 1;
            loop {
                j = find(j + i, &mut p);
                if j <= n {
                    c += 1;
                } else {
                    break;
                }
            }
            *count_map.entry(c).or_insert(0) += 1;
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