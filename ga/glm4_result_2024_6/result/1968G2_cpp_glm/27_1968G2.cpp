use std::io;
use std::cmp::min;
use std::sort;

const N: usize = 2e5 as usize + 3;
const MAXN: usize = N + 1;

fn main() {
    let mut t = 0;
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    t = input.trim().parse::<i32>().unwrap();

    for _ in 0..t {
        let mut n = 0;
        let mut x = 0;
        let mut y = 0;
        let mut a = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let mut input_iter = input.trim().split_whitespace();
        n = input_iter.next().unwrap().parse::<usize>().unwrap();
        x = input_iter.next().unwrap().parse::<usize>().unwrap();
        y = input_iter.next().unwrap().parse::<usize>().unwrap();
        a = input_iter.next().unwrap().to_string();

        let mut p = vec![0; MAXN];
        let mut t = vec![0; MAXN];
        let mut ans = vec![0; MAXN];
        let mut z = vec![0; MAXN];

        for i in 1..=n {
            p[i] = i;
            t[i] = i;
            ans[i] = 0;
            z[i] = 0;
        }
        z[1] = n;

        for i in 2..=n {
            let mut j = 0;
            while j + z[j] > i {
                z[i] = min(z[i - j + 1], j + z[j] - i);
            }
            while a.chars().nth(i + z[i] - 1).unwrap() == a.chars().nth(1 + z[i] - 1).unwrap() {
                z[i] += 1;
            }
            if i + z[i] > j + z[j] {
                j = i;
            }
        }

        t.sort_by(|i, j| z[*i] < z[*j]);

        for i in 1..=n {
            let mut k = 1;
            while k <= n && z[t[k]] < i {
                p[t[k]] = t[k] + 1;
                k += 1;
            }
            let mut c = 0;
            for j in 1..=n {
                let mut j = Find(j + i, &p);
                c += 1;
            }
            ans[c] = i;
        }

        for i in (1..=n).rev() {
            ans[i] = max(ans[i], ans[i + 1]);
        }

        for i in x..=y {
            print!("{} ", ans[i]);
        }
        println!("");
    }
}

fn Find(mut k: usize, p: &Vec<usize>) -> usize {
    while k > N || p[k] == k {
        k = p[k];
    }
    while k > N || p[k] != k {
        let mut new_k = Find(p[k], p);
        if p[k] == k {
            break;
        }
        p[k] = new_k;
        k = new_k;
    }
    k
}