use std::io::{self, BufRead};

const N: usize = 2e5 as usize + 3;

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
    let mut input = stdin.lock();
    
    let mut buffer = String::new();
    input.read_line(&mut buffer).unwrap();
    let t: usize = buffer.trim().parse().unwrap();

    for _ in 0..t {
        buffer.clear();
        input.read_line(&mut buffer).unwrap();
        let parts: Vec<usize> = buffer.split_whitespace().map(|x| x.parse().unwrap()).collect();
        let n = parts[0];
        let x = parts[1];
        let y = parts[2];

        buffer.clear();
        input.read_line(&mut buffer).unwrap();
        let a: Vec<char> = buffer.trim().chars().collect();

        let mut z = vec![0; N];
        let mut p = [0; N];
        let mut t = [0; N];
        let mut ans = [0; N];

        let mut j = 0;
        let mut z_j = 0;
        z[1] = n;
        for i in 2..=n {
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

        let mut t_iter = (1..=n).collect::<Vec<usize>>();
        t_iter.sort_by(|&i, &j| z[i].cmp(&z[j]));

        let mut k = 1;
        for i in 1..=n {
            let mut c = 0;
            while k <= n && z[t_iter[k - 1]] < i {
                p[t_iter[k - 1]] = t_iter[k - 1] + 1;
                k += 1;
            }
            let mut j = 1;
            while j <= n {
                j = find(j + i, &mut p);
                c += 1;
            }
            ans[c] = i;
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