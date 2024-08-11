use std::cmp::min;
use std::collections::HashMap;

fn find(k: usize, p: &mut HashMap<usize, usize>) -> usize {
    match p.get(&k) {
        Some(&val) if val == k || val > p.len() => k,
        Some(&val) => {
            let root = find(val, p);
            p.insert(k, root);
            root
        },
        None => k,
    }
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let t: usize = input.trim().parse().unwrap();

    for _ in 0..t {
        input.clear();
        std::io::stdin().read_line(&mut input).unwrap();
        let parts: Vec<usize> = input.split_whitespace().map(|s| s.trim().parse().unwrap()).collect();
        let n = parts[0];
        let x = parts[1];
        let y = parts[2];

        input.clear();
        std::io::stdin().read_line(&mut input).unwrap();
        let a = input.trim().chars().collect::<Vec<char>>();

        let mut z = vec![0; n + 2];
        let mut p = (1..=n+1).collect::<HashMap<usize, usize>>();
        let mut t = (1..=n).collect::<Vec<usize>>();
        let mut ans = vec![0; n+2];
        let mut ans_result = vec![0; n+2];

        z[1] = n;
        let mut j = 0;

        for i in 2..=n {
            if j + z[j] > i {
                z[i] = min(z[i - j + 1], j + z[j] - i);
            }

            while a[i + z[i]] == a[1 + z[i]] {
                z[i] += 1;
            }

            if i + z[i] > j + z[j] {
                j = i;
            }
        }

        t.sort_by_cached_key(|&k| z[k]);

        let mut k = 1;
        for i in 1..=n {
            let mut c = 0;

            while k <= n && z[t[k-1]] < i {
                p.insert(t[k-1], t[k-1] + 1);
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