use std::cmp;
use std::collections::HashMap;

fn find(k: usize, p: &mut Vec<usize>) -> usize {
    if k > p.len() || p[k] == k {
        return k;
    }
    p[k] = find(p[k], p);
    p[k]
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let t: i32 = input.trim().parse().unwrap();
    
    for _ in 0..t {
        input.clear();
        std::io::stdin().read_line(&mut input).unwrap();
        let mut iter = input.split_whitespace();
        let n: usize = iter.next().unwrap().parse().unwrap();
        let x: usize = iter.next().unwrap().parse().unwrap();
        let y: usize = iter.next().unwrap().parse().unwrap();
        
        input.clear();
        std::io::stdin().read_line(&mut input).unwrap();
        let a: Vec<char> = input.trim().chars().collect();
        
        let mut z: Vec<usize> = vec![0; n+1];
        let mut p: Vec<usize> = (0..=n+1).collect();
        let mut t: Vec<usize> = (0..=n+1).collect();
        let mut ans: Vec<usize> = vec![0; n+1];
        
        let mut j = 0;
        z[1] = n;
        for (i, ch) in a.iter().enumerate().skip(1) {
            if j + z[j] > i {
                z[i] = cmp::min(z[i-j+1], j + z[j] - i);
            }
            while a[i+z[i]] == a[1+z[i]] {
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
                let root = find(j + i, &mut p);
                c += 1;
                j = root;
            }
            ans[c] = i;
        }
        
        for i in (1..=n).rev() {
            ans[i] = cmp::max(ans[i], ans[i+1]);
        }
        
        for i in x..=y {
            print!("{} ", ans[i]);
        }
        println!("");
    }
}