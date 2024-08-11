use std::cmp::Ordering;
use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let t: usize = input.trim().parse().expect("Invalid input");

    for _ in 0..t {
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let mut values = input.trim().split_whitespace();
        let n: usize = values.next().unwrap().parse().expect("Invalid input");
        let x: usize = values.next().unwrap().parse().expect("Invalid input");
        let y: usize = values.next().unwrap().parse().expect("Invalid input");
        let a = values.next().unwrap();

        let mut z = vec![0; n + 1];
        let mut p = vec![0; n + 1];
        let mut t = (0..=n).collect::<Vec<usize>>();
        let mut ans = vec![0; n + 1];

        z[1] = n;

        for i in 2..=n {
            let mut j = 0;
            while i + z[j] > i {
                z[i] = std::cmp::min(z[i - j + 1], j + z[j] - i);
                if a.chars().nth(i + z[i]) == a.chars().nth(1 + z[i]) {
                    z[i] += 1;
                } else {
                    break;
                }
            }
            if i + z[i] > j + z[j] {
                j = i;
            }
        }

        t.sort_by(|&i, &j| z[i].cmp(&z[j]));

        for i in 1..=n {
            let mut k = 0;
            while k <= n && z[t[k]] < i {
                p[t[k]] = t[k] + 1;
                k += 1;
            }
            let mut c = 0;
            for j in 1..=n {
                c += 1;
                let mut root = j;
                while root <= n {
                    root = p[root];
                    c += 1;
                }
            }
            ans[c] = i;
        }

        for i in (1..=n).rev() {
            ans[i] = std::cmp::max(ans[i], ans[i + 1]);
        }

        for i in x..=y {
            print!("{} ", ans[i]);
        }
        println!();
    }
}