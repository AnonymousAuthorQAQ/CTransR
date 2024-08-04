use std::cmp;

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
        let mut a = String::new();
        parts.next().map(|s| a.push_str(s));

        let mut p: [usize; N] = [0; N];
        let mut t: [usize; N] = [0; N];
        let mut ans: [usize; N] = [0; N];
        let mut z: [usize; N] = [0; N];

        z[1] = n;
        for i in 2..=n {
            let mut j = 0;
            let mut k = j;
            while j + z[j] > i {
                z[i] = cmp::min(z[i - j + 1], j + z[j] - i);
                for _ in 0..z[i] {
                    if a.chars().nth(i + z[i]).unwrap() != a.chars().nth(1 + z[i]).unwrap() {
                        break;
                    }
                }
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
            while k <= n && z[t_sorted[k]] < i {
                p[t_sorted[k]] = t_sorted[k] + 1;
                k += 1;
            }
            let mut j = 1;
            loop {
                let mut j_find = find(j + i, &mut p);
                if j_find > n {
                    break;
                }
                c += 1;
                j = j_find;
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