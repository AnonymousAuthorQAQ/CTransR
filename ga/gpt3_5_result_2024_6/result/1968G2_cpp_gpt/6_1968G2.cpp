use std::cmp;
use std::collections::HashMap;

const N: usize = 2 * 10usize.pow(5) + 3;

fn find(k: usize, p: &mut Vec<usize>) -> usize {
    if k > p.len() || p[k] == k {
        k
    } else {
        let parent = find(p[k], p);
        p[k] = parent;
        parent
    }
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let t: usize = input.trim().parse().unwrap();

    for _ in 0..t {
        input.clear();
        std::io::stdin().read_line(&mut input).unwrap();
        let mut parts = input.trim().split_whitespace();
        let n: usize = parts.next().unwrap().parse().unwrap();
        let x: usize = parts.next().unwrap().parse().unwrap();
        let y: usize = parts.next().unwrap().parse().unwrap();
        
        input.clear();
        std::io::stdin().read_line(&mut input).unwrap();
        let a = input.trim().chars().collect::<Vec<char>>();

        let mut p: Vec<usize> = (0..=n+1).collect();
        let mut t: Vec<usize> = (0..=n+1).collect();
        let mut ans: Vec<usize> = vec![0; n+2];
        let mut z: Vec<usize> = vec![0; n+2];

        z[1] = n;
        for i in 2..=n {
            let mut j = 0;
            if j + z[j] > i {
                z[i] = cmp::min(z[i-j+1], j+z[j]-i);
            }
            while a[i+z[i]] == a[1+z[i]] {
                z[i] += 1;
            }
            if i+z[i] > j+z[j] {
                j = i;
            }
        }

        t.sort_by_key(|&i| z[i]);
        
        let mut k = 1;
        for i in 1..=n {
            let mut c = 0;
            while k <= n && z[t[k]] < i {
                p[t[k]] = t[k] + 1;
                k += 1;
            }
            let mut j = 1;
            while j <= n {
                j = find(j+i, &mut p);
                c += 1;
            }
            ans[c] = i;
        }

        for i in (1..=n).rev() {
            ans[i] = cmp::max(ans[i], ans[i+1]);
        }

        for i in x..=y {
            print!("{} ", ans[i]);
        }
        println!();
    }
}