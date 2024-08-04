use std::cmp::min;

const N: usize = 2e5 + 3;

fn find(k: usize, p: &mut [usize; N]) -> usize {
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
    let T: usize = input.trim().parse().unwrap();

    for _ in 0..T {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let parts: Vec<&str> = input.trim().split_whitespace().collect();
        let n: usize = parts[0].parse().unwrap();
        let x: usize = parts[1].parse().unwrap();
        let y: usize = parts[2].parse().unwrap();
        let a: Vec<char> = parts[3].chars().collect();

        let mut p: [usize; N] = [0; N];
        let mut t: [usize; N] = [0; N];
        let mut ans: [usize; N] = [0; N];
        let mut z: [usize; N] = [0; N];

        z[1] = n;
        let mut i = 2;
        let mut j = 0;
        while i <= n {
            if j + z[j] > i {
                z[i] = min(z[i - j + 1], j + z[j] - i);
            }
            while a[i + z[i] - 1] == a[1 + z[i] - 1] {
                z[i] += 1;
            }
            if i + z[i] > j + z[j] {
                j = i;
            }
            i += 1;
        }

        let mut t: Vec<usize> = (1..=n).collect();
        t.sort_by(|&i, &j| z[i].cmp(&z[j]));

        let mut k = 1;
        for i in 1..=n {
            let mut c = 0;
            while k <= n && z[t[k - 1]] < i {
                p[t[k - 1]] = t[k - 1] + 1;
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
            ans[i] = ans[i].max(ans[i + 1]);
        }

        for i in x..=y {
            print!("{} ", ans[i]);
        }
        println!();
    }
}