use std::io;
use std::cmp::min;

const N: usize = 2e5 as usize + 3;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let t: usize = input.trim().parse().unwrap();

    for _ in 0..t {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let mut tokens = input.trim().split_whitespace();
        let n: usize = tokens.next().unwrap().parse().unwrap();
        let x: usize = tokens.next().unwrap().parse().unwrap();
        let y: usize = tokens.next().unwrap().parse().unwrap();
        let a = tokens.next().unwrap();

        let mut z = vec![0; n + 1];
        let mut p = vec![0; n + 1];
        let mut t = (0..n + 1).collect::<Vec<usize>>();
        let mut ans = vec![0; n + 1];

        z[1] = n;

        for i in 2..=n {
            let mut j = 0;
            while i + z[j] > i {
                z[i] = min(z[i - j + 1], j + z[j] - i);
                if a.chars().nth(i + z[i]) == a.chars().nth(1 + z[i]) {
                    z[i] += 1;
                } else {
                    break;
                }
            }
            if i + z[i] > j + z[j] {
                j = i;
            }
        }

        t.sort_by_key(|&i| z[i]);

        for i in 1..=n {
            let mut k = 0;
            while k <= n && z[t[k]] < i {
                p[t[k]] = t[k] + 1;
                k += 1;
            }
            let mut c = 0;
            for j in 1..=n {
                c += 1;
                while j + i <= n {
                    let mut parent = Find(p[j]);
                    if parent > n {
                        break;
                    }
                    j = parent;
                }
            }
            ans[c] = i;
        }

        for i in (1..n).rev() {
            ans[i] = max(ans[i], ans[i + 1]);
        }

        for i in x..=y {
            print!("{} ", ans[i]);
        }
        println!();
    }
}

fn Find(p: &mut Vec<usize>, mut i: usize) -> usize {
    if i > p.len() {
        return i;
    }
    while i != p[i] {
        p[i] = p[p[i]];
        i = p[i];
    }
    i
}