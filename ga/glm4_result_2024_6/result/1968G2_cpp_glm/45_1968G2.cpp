use std::io;
use std::cmp::min;
use std::collections::HashMap;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let t: usize = input.trim().parse().unwrap();

    for _ in 0..t {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let mut values = input.trim().split_whitespace();
        let n: usize = values.next().unwrap().parse().unwrap();
        let x: usize = values.next().unwrap().parse().unwrap();
        let y: usize = values.next().unwrap().parse().unwrap();
        let a = values.next().unwrap();

        let mut z = vec![0; n + 1];
        let mut p = vec![0; n + 1];
        let mut t = (0..=n).collect::<Vec<usize>>();
        let mut ans = vec![0; n + 1];

        z[1] = n;
        let mut j = 0;
        for i in 2..=n {
            if j + z[j] > i {
                z[i] = min(z[i - j + 1], j + z[j] - i);
            }
            while a.chars().nth(i + z[i]) == a.chars().nth(1 + z[i]) {
                z[i] += 1;
            }
            if i + z[i] > j + z[j] {
                j = i;
            }
        }

        t.sort_by_key(|&i| z[i]);

        let mut parent = HashMap::new();
        for i in 1..=n {
            parent.insert(i, i + 1);
        }

        for i in 1..=n {
            let mut k = 0;
            while k <= n && z[t[k]] < i {
                parent.insert(t[k], t[k] + 1);
                k += 1;
            }
            let mut c = 0;
            for j in 1..=n {
                c += 1;
                let mut next = parent[&j];
                while next <= n {
                    j = next;
                    next = parent[&j];
                }
            }
            ans[c] = i;
        }

        for i in (1..=n).rev() {
            ans[i] = ans[i].max(ans[i + 1]);
        }

        for i in x..=y {
            print!("{} ", ans[i]);
        }
        println!("");
    }
}