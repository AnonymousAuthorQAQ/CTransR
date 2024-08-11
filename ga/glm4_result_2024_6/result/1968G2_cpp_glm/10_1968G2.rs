const N: usize = 2e5 as usize + 3;
fn main() {
    let mut t: i32;
    unsafe {
        t = std::io::stdin().read_i32().unwrap();
    }
    while t > 0 {
        let n: i32;
        let x: i32;
        let y: i32;
        let mut a = String::with_capacity(N as usize);
        unsafe {
            n = std::io::stdin().read_i32().unwrap();
            x = std::io::stdin().read_i32().unwrap();
            y = std::io::stdin().read_i32().unwrap();
            std::io::stdin().read_line(&mut a).unwrap();
        }
        let mut z = vec![0; N];
        let mut p = vec![0; N];
        let mut t = vec![0; N];
        let mut ans = vec![0; N];
        for i in 0..N {
            p[i] = i;
            t[i] = i;
            ans[i] = 0;
            z[i] = 0;
        }
        z[0] = n as usize;
        for i in 1..=n as usize {
            let mut j = 0;
            while j + z[j] > i {
                z[i] = std::cmp::min(z[i - j + 1], j + z[j] - i);
            }
            while a.chars().nth(i as usize + z[i]) == a.chars().nth(1 + z[i]) {
                z[i] += 1;
            }
            if i + z[i] > j + z[j] {
                j = i;
            }
        }
        t.sort_unstable_by(|i, j| z[*i].cmp(&z[*j]));
        for i in 1..=n as usize {
            let mut c = 0;
            for k in 1..=n as usize {
                while k <= n as usize && z[t[k] as usize] < i {
                    p[t[k] as usize] = t[k] as usize + 1;
                    k += 1;
                }
                let mut j = 1;
                while j <= n as usize {
                    let mut j = Find(j as i32 + i);
                    c += 1;
                    j = Find(j);
                }
                ans[c] = i;
            }
            for i in (x as usize..=y as usize).rev() {
                ans[i] = std::cmp::max(ans[i], ans[i + 1]);
            }
            for i in x as usize..=y as usize {
                print!("{} ", ans[i]);
            }
            println!("");
        }
    }
}
fn Find(k: i32) -> i32 {
    if k as usize > N || p[k as usize] == k as usize {
        k
    } else {
        p[k as usize] = Find(p[k as usize]);
        p[k as usize]
    }
}