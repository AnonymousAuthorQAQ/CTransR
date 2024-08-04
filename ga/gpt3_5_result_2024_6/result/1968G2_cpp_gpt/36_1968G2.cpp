use std::io;
use std::io::BufRead;

const N: usize = 200003;

fn find(k: usize, p: &mut Vec<usize>) -> usize {
    if k > p.len() || p[k] == k {
        return k;
    }
    p[k] = find(p[k], p);
    p[k]
}

fn main() {
    let stdin = io::stdin();
    let mut input = stdin.lock();
    let mut buffer = String::new();
    
    input.read_line(&mut buffer).unwrap();
    let t: usize = buffer.trim().parse().unwrap();
    
    for _ in 0..t {
        buffer.clear();
        input.read_line(&mut buffer).unwrap();
        let mut parts = buffer.split_whitespace();
        let n: usize = parts.next().unwrap().parse().unwrap();
        let x: usize = parts.next().unwrap().parse().unwrap();
        let y: usize = parts.next().unwrap().parse().unwrap();
        let a: &str = parts.next().unwrap();
        
        let mut z = vec![0; n + 1];
        let mut p = (0..n + 2).collect::<Vec<usize>>();
        let mut t = (0..n + 2).collect::<Vec<usize>>();
        let mut ans = vec![0; n + 2];
        
        z[1] = n;
        
        for i in 2..n + 1 {
            let mut j = 0;
            if j + z[j] > i {
                z[i] = std::cmp::min(z[i - j + 1], j + z[j] - i);
            }
            let mut match_len = 0;
            for _ in 0.. {
                if a.chars().nth(i + match_len) != a.chars().nth(1 + match_len) {
                    break;
                }
                match_len += 1;
            }
            z[i] = match_len;
            if i + z[i] > j + z[j] {
                j = i;
            }
        }
        
        t.sort_by(|&i, &j| z[i].cmp(&z[j]));
        
        let mut k = 1;
        for i in 1..n + 1 {
            let mut c = 0;
            while k <= n && z[t[k]] < i {
                p[t[k]] = t[k] + 1;
                k += 1;
            }
            for j in 1..n + 1 {
                let mut cur = j;
                while cur + i < n + 2 {
                    cur = find(cur + i, &mut p);
                    c += 1;
                }
            }
            ans[c] = i;
        }
        
        for i in (1..n + 1).rev() {
            ans[i] = std::cmp::max(ans[i], ans[i + 1]);
        }
        
        for i in x..=y {
            print!("{} ", ans[i]);
        }
        println!();
    }
}