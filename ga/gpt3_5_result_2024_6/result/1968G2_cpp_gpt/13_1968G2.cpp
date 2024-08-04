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
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let t: usize = input.trim().parse().unwrap();

    for _ in 0..t {
        input.clear();
        std::io::stdin().read_line(&mut input).unwrap();
        
        let mut parts = input.split_whitespace();
        let n: usize = parts.next().unwrap().parse().unwrap();
        let x: usize = parts.next().unwrap().parse().unwrap();
        let y: usize = parts.next().unwrap().parse().unwrap();
        let s: String = parts.next().unwrap().to_string();
        let a: Vec<char> = s.chars().collect();

        let mut z = vec![0; n + 1];
        let mut p = (1..=n+1).collect::<Vec<usize>>();
        let mut t = (1..=n).collect::<Vec<usize>>();
        let mut ans = vec![0; n + 1];

        let mut j = 0;
        let mut k = 2;

        z[1] = n;
        while k <= n {
            if j + z[j] > k {
                z[k] = std::cmp::min(z[k - j + 1], j + z[j] - k);
            }
            while k + z[k] <= n && a[k + z[k] - 1] == a[z[k] - 1] {
                z[k] += 1;
            }
            if k + z[k] > j + z[j] {
                j = k;
            }
            k += 1;
        }

        t.sort_by(|&i, &j| z[i].partial_cmp(&z[j]).unwrap());

        let mut k = 1;
        for i in 1..=n {
            let mut c = 0;
            while k <= n && z[t[k - 1]] < i {
                p[t[k - 1]] = t[k - 1] + 1;
                k += 1;
            }
            for j in 1..=n {
                let mut idx = j;
                while idx < n && find(idx + i, &mut p) == find(j, &mut p) {
                    idx += i
                }
                if idx == n {
                    c += 1;
                }
            }
            ans[c] = i;
        }

        for i in (0..=n).rev() {
            ans[i] = ans[i].max(ans[i + 1]);
        }

        for i in x..=y {
            print!("{} ", ans[i]);
        }
        println!();
    }
}