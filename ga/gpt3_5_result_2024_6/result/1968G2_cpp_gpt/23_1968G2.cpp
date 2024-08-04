use std::cmp;

const N: usize = 200003;

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
    let t: usize = input.trim().parse().unwrap();

    for _ in 0..t {
        input.clear();
        std::io::stdin().read_line(&mut input).unwrap();
        let mut iter = input.split_whitespace();
        let n: usize = iter.next().unwrap().parse().unwrap();
        let x: usize = iter.next().unwrap().parse().unwrap();
        let y: usize = iter.next().unwrap().parse().unwrap();
        let a: Vec<char> = iter.next().unwrap().chars().collect();

        let mut z: Vec<usize> = vec![0; N];
        let mut p: Vec<usize> = (1..=n+1).collect();
        let mut t: Vec<usize> = (1..=n+1).collect();
        let mut ans: Vec<usize> = vec![0; N];

        z[1] = n;
        for i in 2..=n {
            let mut j = 0;
            let mut k = z[j];
            if j + k > i {
                z[i] = cmp::min(z[i-j+1], j + k - i);
            }
            while a[i + z[i]] == a[1 + z[i]] {
                z[i] += 1;
            }
            if i + z[i] > j + k {
                j = i;
            }
        }

        t.sort_by_key(|&i| z[i]);

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