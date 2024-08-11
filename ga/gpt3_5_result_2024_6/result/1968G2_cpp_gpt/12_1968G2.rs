use std::io::{self, BufRead};

const N: usize = 2e5 as usize + 3;

fn find(k: usize, p: &mut [usize; N]) -> usize {
    if k > N || p[k] == k {
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
        let input_parts: Vec<usize> = buffer
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        let n = input_parts[0];
        let x = input_parts[1];
        let y = input_parts[2];

        let mut a = String::new();
        input.read_line(&mut a).unwrap();
        let chars: Vec<char> = a.trim().chars().collect();

        let mut z = vec![0; N];
        let mut p = (1..=N).collect::<Vec<usize>>();
        let mut t = (1..=N).collect::<Vec<usize>>();
        let mut ans = vec![0; N];

        z[1] = n;
        for i in 2..=n {
            let mut j = 0;
            let mut k = i;
            if j + z[j] > i {
                z[i] = z[std::cmp::min(i - j + 1, j + z[j] - i)];
            }
            while chars[i + z[i] - 1] == chars[1 + z[i] - 1] {
                z[i] += 1;
            }
            if i + z[i] > j + z[j] {
                j = i;
            }
        }

        t.sort_by(|&i, &j| z[i].partial_cmp(&z[j]).unwrap());

        let mut k = 1;
        for i in 1..=n {
            let mut c = 0;
            while k <= n && z[t[k]] < i {
                p[t[k]] = t[k] + 1;
                k += 1;
            }
            let mut j = 1;
            loop {
                let result = find(j + i, &mut p);
                j = result;
                if j <= n {
                    c += 1;
                }
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