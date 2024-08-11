use std::cmp;
use std::io::stdin;

const N: usize = 2e5 as usize + 3;

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
    stdin().read_line(&mut input).unwrap();
    let t: usize = input.trim().parse().unwrap();

    for _ in 0..t {
        input.clear();
        stdin().read_line(&mut input).unwrap();
        let values: Vec<usize> = input.split_whitespace().map(|x| x.parse().unwrap()).collect();
        let n = values[0];
        let x = values[1];
        let y = values[2];

        input.clear();
        stdin().read_line(&mut input).unwrap();
        let a = input.trim().chars().collect::<Vec<char>>();

        let mut z = vec![0; n+2];
        let mut p = (0..n+2).collect::<Vec<usize>>();
        let mut t = (0..n+2).collect::<Vec<usize>>();
        let mut ans = vec![0; n+2];
        
        let mut j = 0;
        z[1] = n;

        for i in 2..=n {
            if j + z[j] > i {
                z[i] = cmp::min(z[i-j+1], j + z[j] - i);
            }

            while a[i+z[i]] == a[1+z[i]] {
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
                let root = find(j + i, &mut p);
                c += 1;
                j = root;
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