use std::collections::HashMap;

fn find(k: usize, p: &mut Vec<usize>) -> usize {
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
        input.clear();
        std::io::stdin().read_line(&mut input).unwrap();
        let mut nums = input.trim().split_whitespace();
        let n: usize = nums.next().unwrap().parse().unwrap();
        let x: usize = nums.next().unwrap().parse().unwrap();
        let y: usize = nums.next().unwrap().parse().unwrap();

        input.clear();
        std::io::stdin().read_line(&mut input).unwrap();
        let a: Vec<char> = input.trim().chars().collect();

        let mut z = vec![0; n + 1];
        let mut p: Vec<usize> = (0..=n).collect();
        let mut t: Vec<usize> = (0..=n).collect();
        let mut ans: Vec<usize> = vec![0; n + 1];

        z[1] = n;
        for i in 2..=n {
            let mut j = 0;
            let mut k = j + z[j];
            if k > i {
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
                j = find(j + i, &mut p);
                if j > n {
                    break;
                }
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