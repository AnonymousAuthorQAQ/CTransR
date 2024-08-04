use std::io::{self, BufRead};

fn find(k: usize, p: &mut Vec<usize>) -> usize {
    if k > p.len() || p[k] == k {
        return k;
    }
    let root = find(p[k], p);
    p[k] = root;
    root
}

fn main() {
    let stdin = io::stdin();
    let mut buffer = String::new();

    stdin.lock().read_line(&mut buffer).unwrap();
    let t: usize = buffer.trim().parse().unwrap();

    for _ in 0..t {
        buffer.clear();
        stdin.lock().read_line(&mut buffer).unwrap();
        let inputs: Vec<usize> = buffer.split_whitespace().map(|x| x.parse().unwrap()).collect();
        let n = inputs[0];
        let x = inputs[1];
        let y = inputs[2];

        buffer.clear();
        stdin.lock().read_line(&mut buffer).unwrap();
        let a: Vec<char> = buffer.trim().chars().collect();

        let mut z = vec![0; n + 1];
        let mut p = (1..=n+1).collect::<Vec<usize>>();
        let mut t = (1..=n).collect::<Vec<usize>>();
        let mut ans = vec![0; n + 1];

        let mut j = 0;
        z[1] = n;
        for i in 2..=n {
            if j + z[j] > i {
                z[i] = *vec![z[i-j+1], j+z[j]-i].iter().min().unwrap();
            }
            while a[i+z[i]-1] == a[1+z[i]-1] {
                z[i] += 1;
            }
            if i+z[i] > j+z[j] {
                j = i;
            }
        }

        t.sort_by(|&a, &b| z[a].cmp(&z[b]));

        let mut k = 1;
        for i in 1..=n {
            let mut c = 0;
            while k <= n && z[t[k-1]] < i {
                p[t[k-1]] = t[k-1] + 1;
                k += 1;
            }
            for mut j in (1..=n).step_by(i) {
                j = find(j, &mut p);
                c += 1;
            }
            ans[c] = i;
        }

        for i in (1..=n).rev() {
            ans[i] = ans[i].max(ans[i+1]);
        }

        for i in x..=y {
            print!("{} ", ans[i]);
        }
        println!();
    }
}