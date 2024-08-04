use std::io::{self, BufRead};

const N: usize = 2e5 as usize + 3;

fn find(k: usize, p: &mut [usize]) -> usize {
    if k > p.len() || p[k] == k {
        k
    } else {
        p[k] = find(p[k], p);
        p[k]
    }
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines().map(|line| line.unwrap());

    let t: usize = lines.next().unwrap().trim().parse().unwrap();
    
    for _ in 0..t {
        let line = lines.next().unwrap();
        let parts: Vec<&str> = line.split_whitespace().collect();
        let n: usize = parts[0].parse().unwrap();
        let x: usize = parts[1].parse().unwrap();
        let y: usize = parts[2].parse().unwrap();
        let a: Vec<char> = parts[3].chars().collect();

        let mut z: Vec<usize> = vec![0; N];
        let mut p: Vec<usize> = (0..=N).collect();
        let mut t: Vec<usize> = (0..=N).collect();
        let mut ans: Vec<usize> = vec![0; N];
        
        z[1] = n;
        for i in 2..=n {
            let mut j = 0;
            let mut k = j;
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

        t.sort_by_key(|&i| z[i]);

        for i in 1..=n {
            let mut k = 1;
            for j in (1..=n).step_by(i).map(|j| find(j, &mut p)) {
                k += 1;
            }
            ans[k] = i;
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