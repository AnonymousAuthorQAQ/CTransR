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
        parts.next().unwrap().chars().for_each(|c| a.push(c));

        let mut p = vec![0; n + 2];
        let mut t = (1..=n+1).collect::<Vec<usize>>();
        let mut ans = vec![0; n + 1];
        let mut z = vec![0; n + 2];
        z[1] = n;

        for i in 2..=n {
            let mut j = 0;
            let mut k = i;
            if j + z[j] > i {
                z[i] = cmp::min(z[i - j + 1], j + z[j] - i);
            }
            while a.chars().nth(i+z[i]-1) == a.chars().nth(1+z[i]-1) {
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
            while j <= n {
                j = find(j + i, &mut p);
                c += 1;
            }
            ans[c] = i;
        }

        for i in (1..=n).rev() {
            ans[i-1] = cmp::max(ans[i-1], ans[i]);
        }

        for i in x..=y {
            print!("{} ", ans[i]);
        }
        println!();
    }
}