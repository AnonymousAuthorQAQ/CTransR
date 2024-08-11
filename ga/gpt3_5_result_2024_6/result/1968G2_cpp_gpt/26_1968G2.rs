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
    let mut lines = stdin.lock().lines().map(Result::unwrap);

    let t: usize = lines.next().unwrap().parse().unwrap();
    for _ in 0..t {
        let input: Vec<String> = lines.next().unwrap().split(" ").map(String::from).collect();
        let n: usize = input[0].parse().unwrap();
        let x: usize = input[1].parse().unwrap();
        let y: usize = input[2].parse().unwrap();
        let a: Vec<char> = input[3].chars().collect();

        let mut p: Vec<usize> = vec![0; n + 2];
        let mut t: Vec<usize> = (1..=n+1).collect();
        let mut ans: Vec<usize> = vec![0; n + 2];
        let mut z: Vec<usize> = vec![0; n + 2];

        z[1] = n;
        for i in 2..=n {
            let mut j = 0;
            let mut k = i;
            while j + z[j] > i {
                z[i] = std::cmp::min(z[i - j + 1], j + z[j] - i);
            }
            while a[i + z[i]] == a[1 + z[i]] {
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
                let root = find(j + i, &mut p);
                if root <= n {
                    c += 1;
                }
                j = root;
                if j > n {
                    break;
                }
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