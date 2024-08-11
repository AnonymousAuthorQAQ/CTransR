use std::cmp::Ordering;
use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let t: usize = input.trim().parse().unwrap();
    
    for _ in 0..t {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let mut parts = input.trim().split_whitespace();
        let n: usize = parts.next().unwrap().parse().unwrap();
        let x: usize = parts.next().unwrap().parse().unwrap();
        let y: usize = parts.next().unwrap().parse().unwrap();
        let a: Vec<char> = parts.next().unwrap().chars().collect();
        
        let mut p = vec![0; n + 2];
        for i in 1..=n + 1 { p[i] = i; }
        
        let mut z = vec![0; n + 2];
        z[1] = n;
        
        for i in 2..=n {
            let mut j = 0;
            if j + z[j] > i { z[i] = std::cmp::min(z[i - j + 1], j + z[j] - i); }
            while i + z[i] <= n && a[i + z[i]] == a[1 + z[i]] { z[i] += 1; }
            if i + z[i] > j + z[j] { j = i; }
        }
        
        let mut t = (1..=n).collect::<Vec<usize>>();
        t.sort_by(|i, j| z[*i].cmp(&z[*j]));
        
        for i in 1..=n {
            let mut k = 1;
            while k <= n && z[t[k]] < i { p[t[k]] = t[k] + 1; k += 1; }
            
            let mut c = 0;
            for j in 1..=n { if Find(&p, j + i) == j { c += 1; } }
            p[c] = i;
        }
        
        for i in (1..=n).rev() { p[i] = std::cmp::max(p[i], p[i + 1]); }
        
        for i in x..=y { print!("{} ", p[i]); }
        println!("");
    }
}

fn Find(p: &Vec<usize>, mut k: usize) -> usize {
    while k > p.len() || p[k] != k {
        k = p[k];
    }
    k
}