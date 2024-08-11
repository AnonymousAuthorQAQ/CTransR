use std::cmp::min;
use std::collections::HashMap;

const N: usize = 2e5 + 3;

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
        let parts: Vec<&str> = input.trim().split(' ').collect();
        let n: usize = parts[0].parse().unwrap();
        let x: usize = parts[1].parse().unwrap();
        let y: usize = parts[2].parse().unwrap();
        let a: String = parts[3].to_string();

        let mut p: Vec<usize> = (0..=n+1).collect();
        let mut t: Vec<usize> = (0..=n+1).collect();
        let mut ans: Vec<usize> = vec![0; n+2];
        let mut z: Vec<usize> = vec![0; n+2];
        z[1] = n;
        
        let a_chars: Vec<char> = a.chars().collect();
        for i in 2..=n {
            let mut j = 0;
            if j + z[j] > i {
                z[i] = min(z[i - j + 1], j + z[j] - i);
            }
            while a_chars[i + z[i]] == a_chars[1 + z[i]] {
                z[i] += 1;
            }
            if i + z[i] > j + z[j] {
                j = i;
            }
        }

        let mut t_sorted: Vec<usize> = (1..=n).collect();
        t_sorted.sort_by(|&i, &j| z[i].cmp(&z[j]));

        for i in 1..=n {
            let mut k = 1;
            let mut c = 0;
            while k <= n && z[t_sorted[k - 1]] < i {
                p[t_sorted[k - 1]] = t_sorted[k - 1] + 1;
                k += 1;
            }
            for j in 1..=n {
                let mut current = j;
                while current + i <= n {
                    current = find(current + i, &mut p);
                    c += 1;
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
        println!("");
    }
}