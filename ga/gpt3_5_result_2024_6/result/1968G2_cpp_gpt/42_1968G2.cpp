use std::collections::HashMap;

fn find(mut k: usize, p: &mut Vec<usize>) -> usize {
    while k <= p.len() && p[k] != k {
        k = p[k];
    }
    k
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let t: usize = input.trim().parse().unwrap();

    for _ in 0..t {
        input.clear();
        std::io::stdin().read_line(&mut input).unwrap();
        let parts: Vec<usize> = input
            .split_whitespace()
            .map(|x| x.trim().parse().unwrap())
            .collect();
        let n = parts[0];
        let x = parts[1];
        let y = parts[2];

        input.clear();
        std::io::stdin().read_line(&mut input).unwrap();
        let a: Vec<char> = input.trim().chars().collect();
        
        let mut z = vec![0; n+1];
        let mut p = (0..=n).collect::<Vec<_>>();
        let mut t = (0..=n).collect::<Vec<_>>();
        let mut ans = vec![0; n+1];

        z[1] = n;
        let mut i = 2;
        let mut j = 0;
        while i <= n {
            if j + z[j] > i {
                z[i] = *(vec![z[i-j+1], j+z[j]-i].iter().min().unwrap());
            }
            while a[i+z[i]] == a[1+z[i]] {
                z[i] += 1;
            }
            if i + z[i] > j + z[j] {
                j = i;
            }
            i += 1;
        }

        let mut counts = HashMap::new();
        let mut k = 1;
        for i in 1..=n {
            let c: usize;
            while k <= n && z[t[k]] < i {
                p[t[k]] = t[k] + 1;
                k += 1;
            }
            for j in 1..=n {
                let mut idx = j;
                while idx + i <= n {
                    idx = find(idx + i, &mut p);
                    if idx <= n {
                        *counts.entry(idx).or_insert(0) += 1;
                    }
                }
            }
            c = counts.len();
            *counts.entry(i).or_insert(0) = c;
            ans[c] = i;
        }

        for i in (1..=n).rev() {
            ans[i] = ans[i].max(ans[i+1]);
        }

        for i in x..=y {
            print!("{} ", ans[i]);
        }
        println!();
    }
}