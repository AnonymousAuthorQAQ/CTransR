use std::collections::HashMap;
use std::collections::HashSet;
use std::cmp::{min, max};

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
    let test_cases: usize = input.trim().parse().unwrap();

    for _ in 0..test_cases {
        input.clear();
        std::io::stdin().read_line(&mut input).unwrap();
        let mut split_input = input.trim().split_whitespace();
        let n: usize = split_input.next().unwrap().parse().unwrap();
        let x: usize = split_input.next().unwrap().parse().unwrap();
        let y: usize = split_input.next().unwrap().parse().unwrap();
        let a: String = split_input.next().unwrap().to_string();

        let mut z = vec![0; n + 1];
        let mut p = vec![0; n + 2];
        let mut t = vec![0; n + 2];
        let mut ans = vec![0; n + 2];

        let mut a_chars: Vec<char> = a.chars().collect();
        let mut z_p: HashSet<usize> = HashSet::new();

        p[0] = 0;
        t[0] = 0;
        for i in 0..n + 1 {
            p[i+1] = i + 1;
            t[i+1] = i + 1;
            ans[i+1] = 0;
            z_p.insert(i+1);
        }

        z[1] = n;
        let mut j = 0;
        for i in 1..n {
            if j + z[j] > i {
                z[i] = min(z[i-j+1], j+z[j]-i);
            }
            let mut k = i+z[i];
            while a_chars[k] == a_chars[1+z[i]] {
                z[i] += 1;
                k += 1;
            }

            if i+z[i] > j+z[j] {
                j = i;
            }
        }

        let mut sorted_t: Vec<usize> = z_p.into_iter().collect();
        sorted_t.sort_by(|&i, &j| z[i].partial_cmp(&z[j]).unwrap());

        let mut k = 1;
        for i in 0..n {
            let mut c = 0;
            while k <= n && z[sorted_t[k-1]] < i {
                p[sorted_t[k-1]] = sorted_t[k-1] + 1;
                k += 1;
            }

            for j in 1..n+1 {
                let mut root = find(j + i, &mut p);
                c += 1;
            }

            ans[c] = i;
        }

        for i in (1..=n).rev() {
            ans[i] = max(ans[i], ans[i+1]);
        }

        for i in x..=y {
            print!("{} ", ans[i]);
        }
        println!();
    }
}