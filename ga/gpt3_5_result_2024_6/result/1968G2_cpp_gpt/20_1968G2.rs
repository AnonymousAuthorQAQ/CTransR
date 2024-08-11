use std::collections::HashMap;

fn find(k: usize, p: &mut Vec<usize>) -> usize {
    if k > p.len() || p[k] == k {
        k
    } else {
        let root = find(p[k], p);
        p[k] = root;
        root
    }
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let t: usize = input.trim().parse().unwrap();

    for _ in 0..t {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let parts: Vec<usize> = input
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        
        let n = parts[0];
        let x = parts[1];
        let y = parts[2];
        
        let mut a = String::new();
        std::io::stdin().read_line(&mut a).unwrap();

        let a = a.trim();

        let mut z = vec![0; n + 2];
        let mut p = (0..=n+1).collect::<Vec<usize>>();
        let mut t = (0..=n+1).collect::<Vec<usize>>();
        let mut ans = vec![0; n + 2];

        let mut j = 0;
        z[1] = n;
        
        for i in 2..=n {
            if j + z[j] > i {
                z[i] = std::cmp::min(z[i - j + 1], j + z[j] - i);
            }
            while a.chars().nth(i - 1 + z[i]).unwrap() == a.chars().nth(1 + z[i] - 1).unwrap() {
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
            loop {
                let root = find(j + i - 1, &mut p);
                j = root;
                if j <= n {
                    c += 1;
                } else {
                    break;
                }
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