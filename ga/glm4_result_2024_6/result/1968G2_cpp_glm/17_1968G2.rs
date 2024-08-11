use std::io::{self, BufRead};
use std::collections::HashMap;

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let mut z = Vec::new();
    let mut p = Vec::new();
    let mut t = Vec::new();
    let mut ans = Vec::new();

    let mut p_ref = Vec::new();

    let mut find = |k| {
        if k > n { return k; }
        if p_ref[k] == k { return k; }
        let r = find(p_ref[k]);
        p_ref[k] = r;
        r
    };

    let mut n: i32;
    let mut x: i32;
    let mut y: i32;
    let mut a = Vec::new();

    loop {
        n = lines.next().unwrap().unwrap().parse::<i32>().unwrap();
        if n == 0 { break; }

        x = lines.next().unwrap().unwrap().parse::<i32>().unwrap();
        y = lines.next().unwrap().unwrap().parse::<i32>().unwrap();
        a = lines.next().unwrap().unwrap().chars().map(|c| c as i32).collect();

        z.resize(n as usize + 1, 0);
        p.resize(n as usize + 1, 0);
        t.resize(n as usize + 1, 0);
        ans.resize(n as usize + 1, 0);
        p_ref.resize(n as usize + 1, 0);

        z[1] = n;

        for i in 2..=n {
            let mut j = 0;
            while i + z[j] > i { z[i] = std::cmp::min(z[i - j + 1], j + z[j] - i); }
            while a[i + z[i]] == a[1 + z[i]] { z[i] += 1; }
            if i + z[i] > j + z[j] { j = i; }
        }

        t[..n as usize].copy_from_slice(&z[1..n as usize + 1]);
        t.sort_by(|a, b| z[*a].cmp(&z[*b]));

        for i in 1..=n {
            let mut k = 1;
            while k <= n && z[t[k]] < i { p[t[k]] = t[k] + 1; k += 1; }
            let mut c = 0;
            for j in 1..=n {
                let r = find(j + i);
                c += 1;
                p_ref[j] = r;
            }
            ans[c] = i;
        }

        for i in (0..n as usize).rev() {
            ans[i] = std::cmp::max(ans[i], ans[i + 1]);
        }

        for i in x as usize..=y as usize {
            println!("{}", ans[i]);
        }
    }
}