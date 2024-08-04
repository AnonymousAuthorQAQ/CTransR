use std::io::{self, BufRead};

const N: usize = 200003;

fn find(mut k: isize, p: &mut Vec<isize>) -> isize {
    while k > 0 && p[k as usize] != k {
        k = p[k as usize];
    }
    k
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines().map(Result::unwrap);

    let t: usize = lines.next().unwrap().parse().unwrap();
    for _ in 0..t {
        let line: Vec<usize> = lines.next().unwrap().split_whitespace()
            .map(|x| x.parse().unwrap()).collect();
        let n = line[0];
        let x = line[1];
        let y = line[2];
        let a = lines.next().unwrap().chars().collect::<Vec<char>>();

        let mut p: Vec<isize> = (0..=n+1).map(|x| x as isize).collect();
        let mut t: Vec<isize> = (0..=n+1).map(|x| x as isize).collect();
        let mut ans: Vec<usize> = vec![0; n+2];
        let mut z: Vec<usize> = vec![0; n+2];

        z[1] = n;
        let mut j = 0;
        for i in 2..=n {
            if j + z[j] > i {
                z[i] = z[i - j + 1].min(j + z[j] - i);
            }
            while a[i + z[i]] == a[1 + z[i]] {
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
                p[t_sorted[k - 1]] = t_sorted[k - 1] as isize + 1;
                k += 1;
            }
            let mut j = 1;
            while j <= n {
                let root = find(j as isize + i as isize, &mut p) as usize;
                c += 1;
                j = root + 1;
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