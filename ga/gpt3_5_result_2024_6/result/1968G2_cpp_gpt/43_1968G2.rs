use std::cmp::min;

const N: usize = 200003;

fn find(k: usize, p: &mut [usize; N]) -> usize {
    if k > n || p[k] == k {
        k
    } else {
        p[k] = find(p[k], p);
        p[k]
    }
}

fn main() {
    let mut t = String::new();
    std::io::stdin().read_line(&mut t).unwrap();
    let t: usize = t.trim().parse().unwrap();
    
    for _ in 0..t {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let mut iter = input.split_whitespace();
        let n: usize = iter.next().unwrap().parse().unwrap();
        let x: usize = iter.next().unwrap().parse().unwrap();
        let y: usize = iter.next().unwrap().parse().unwrap();
        let a: Vec<char> = iter.next().unwrap().chars().collect();

        let mut z = vec![0; n+1];
        let mut p = (0..=n).collect::<Vec<usize>>();
        let mut t = (0..=n).collect::<Vec<usize>>();
        let mut ans = vec![0; n+1];

        z[1] = n;
        for i in 2..=n {
            let mut j = 0;
            if j + z[j] > i {
                z[i] = min(z[j + z[j] - i], j + z[j] - i);
            }
            while a[i + z[i]] == a[1 + z[i]] {
                z[i] += 1;
            }
            if i + z[i] > j + z[j] {
                j = i;
            }
        }

        t.sort_by_cached_key(|&i| z[i]);

        let mut k = 1;
        for i in 1..=n {
            let mut c = 0;
            while k <= n && z[t[k]] < i {
                p[t[k]] = t[k] + 1;
                k += 1;
            }
            for j in 1..=n {
                let mut j_new = find(j + i, &mut p);
                c += 1;
                j = j_new;
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