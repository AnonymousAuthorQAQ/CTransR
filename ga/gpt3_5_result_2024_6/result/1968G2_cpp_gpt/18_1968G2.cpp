use std::io::{self, BufRead};

const N: usize = 200003;

fn find(k: usize, p: &mut [usize]) -> usize {
    if k > p.len() || p[k] == k {
        k
    } else {
        p[k] = find(p[k], p);
        p[k]
    }
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines().map(|x| x.unwrap());

    let t: usize = lines.next().unwrap().parse().unwrap();
    
    for _ in 0..t {
        let input: Vec<usize> = lines
            .next().unwrap()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        
        let n = input[0];
        let x = input[1];
        let y = input[2];
        let a: Vec<char> = input[3].chars().collect();

        let mut z: Vec<usize> = vec![0; N];
        let mut p: Vec<usize> = (0..=n+1).collect();
        let mut t: Vec<usize> = (0..=n+1).collect();
        let mut ans: Vec<usize> = vec![0; N];

        z[1] = n;
        
        for i in 2..=n {
            let (mut j, mut k) = (0, 0);
            if j + z[j] > i {
                z[i] = std::cmp::min(z[i - j + 1], j + z[j] - i);
            }
            while a[i + z[i]] == a[1 + z[i]] {
                z[i] += 1;
            }
            if i + z[i] > j + z[j] {
                j = i;
            }
        }

        let mut t_sorted: Vec<usize> = (1..=n).collect();
        t_sorted.sort_by(|&i, &j| z[i].cmp(&z[j]));

        for i in 1..=n {
            let (mut c, mut k) = (0, 1);
            for t_i in &t_sorted {
                let t_k = *t_i;
                if z[t_k] < i {
                    p[t_k] = t_k + 1;
                }
                k += 1;
                let mut j = 1;
                while j <= n {
                    j = find(j + i, &mut p);
                    c += 1;
                }
                ans[c] = i;
            }
        }

        for i in (1..=n).rev() {
            ans[i] = std::cmp::max(ans[i], ans[i + 1]);
        }

        for i in x..=y {
            print!("{} ", ans[i]);
        }
        println!();
    }
}