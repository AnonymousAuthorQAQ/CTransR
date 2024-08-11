use std::io::{self, Read};
use std::cmp;

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
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut lines = input.lines();

    let t: usize = lines.next().unwrap().trim().parse().unwrap();
    for _ in 0..t {
        let line: Vec<usize> = lines.next().unwrap().split_whitespace().map(|x| x.parse().unwrap()).collect();
        let n = line[0];
        let x = line[1];
        let y = line[2];
        let a = line[3].as_bytes();

        let mut z = vec![0; n+2];
        let mut p: Vec<usize> = (0..=n+1).collect();
        let mut t: Vec<usize> = (0..=n+1).collect();
        let mut ans = vec![0; n+2];

        z[1] = n;
        for i in 2..=n {
            let mut j = 0;
            let mut k = i;
            if j + z[j] > i {
                z[i] = cmp::min(z[i-j+1], j + z[j] - i);
            }
            while a[i+z[i]] == a[1+z[i]] {
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
            for j in 1..=n {
                let mut cur = j;
                while cur + i <= n {
                    cur = find(cur + i, &mut p);
                    c += 1;
                }
            }
            ans[c] = i;
        }

        for i in (1..=n).rev() {
            ans[i] = cmp::max(ans[i], ans[i+1]);
        }

        for i in x..=y {
            print!("{} ", ans[i]);
        }
        println!("");
    }
}