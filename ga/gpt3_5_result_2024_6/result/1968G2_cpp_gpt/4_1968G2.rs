use std::cmp;

const N: usize = 2e5 + 3;

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
    std::io::stdin().read_line(&mut input).unwrap();
    let t: usize = input.trim().parse().unwrap();

    for _ in 0..t {
        input.clear();
        std::io::stdin().read_line(&mut input).unwrap();
        let mut parts = input.split_whitespace();
        let n: usize = parts.next().unwrap().parse().unwrap();
        let x: usize = parts.next().unwrap().parse().unwrap();
        let y: usize = parts.next().unwrap().parse().unwrap();
        let a: String = parts.next().unwrap().parse().unwrap();

        let mut p: Vec<usize> = vec![0; n + 2];
        let mut t: Vec<usize> = vec![0; n + 2];
        let mut ans: Vec<usize> = vec![0; n + 2];
        let mut z: Vec<usize> = vec![0; n + 2];

        z[1] = n;
        let mut j = 0;
        for i in 2..=n {
            if j + z[j] > i {
                z[i] = cmp::min(z[i - j + 1], j + z[j] - i);
            }
            while a.chars().nth(i + z[i] - 1) == a.chars().nth(1 + z[i]) {
                z[i] += 1;
            }
            if i + z[i] > j + z[j] {
                j = i;
            }
        }

        let mut t_sorted: Vec<usize> = (1..=n).collect();
        t_sorted.sort_by(|&i, &j| z[i].cmp(&z[j]));

        let mut k = 1;
        for i in 1..=n {
            let mut c = 0;
            while k <= n && z[t_sorted[k - 1]] < i {
                p[t_sorted[k - 1]] = t_sorted[k - 1] + 1;
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
            ans[i] = cmp::max(ans[i], ans[i + 1]);
        }

        for i in x..=y {
            print!("{} ", ans[i]);
        }
        println!();
    }
}