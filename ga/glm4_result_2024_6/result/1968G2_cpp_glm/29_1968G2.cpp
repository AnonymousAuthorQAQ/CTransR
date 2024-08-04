const N: usize = 2_000_003;
fn main() {
    let mut n;
    let mut x;
    let mut y;
    let mut z = vec![0; N];
    let mut p = vec![0; N];
    let mut t = vec![0; N];
    let mut ans = vec![0; N];
    let mut a = vec![0; N];

    for _ in 0.. {
        n = 0;
        x = 0;
        y = 0;
        a.clear();
        z.clear();
        p.clear();
        t.clear();
        ans.clear();

        a.resize(N, 0);
        z.resize(N, 0);
        p.resize(N, 0);
        t.resize(N, 0);
        ans.resize(N, 0);

        std::io::stdin().read_line(&mut a).expect("Failed to read line");

        for i in 1..=n + 1 {
            p[i] = i;
            t[i] = i;
            ans[i] = 0;
            z[i] = 0;
        }
        z[1] = n;

        for i in 2..=n {
            let mut j = 0;
            if j + z[j] > i {
                z[i] = std::cmp::min(z[i - j + 1], j + z[j] - i);
            }
            while a[i + z[i]] == a[1 + z[i]] {
                z[i] += 1;
            }
            if i + z[i] > j + z[j] {
                j = i;
            }
        }

        t.sort_unstable_by(|i, j| z[*i].cmp(&z[*j]));

        for i in 1..=n {
            let mut k = 1;
            while k <= n && z[t[k]] < i {
                p[t[k]] = t[k] + 1;
                k += 1;
            }

            let mut c = 0;
            for j in 1..=n {
                c += 1;
                j = find(j + i);
            }
            ans[c] = i;
        }

        for i in (0..n).rev() {
            ans[i] = std::cmp::max(ans[i], ans[i + 1]);
        }

        for i in x..=y {
            print!("{} ", ans[i]);
        }
        println!("");
    }
}

fn find(x: usize) -> usize {
    if x > n {
        return x;
    }
    let mut x = x;
    while p[x] != x {
        x = p[x];
    }
    x
}