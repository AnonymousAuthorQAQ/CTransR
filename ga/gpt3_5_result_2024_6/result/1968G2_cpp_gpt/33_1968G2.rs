use std::cmp;

const N: usize = 200003;

fn find(k: usize, p: &mut Vec<usize>) -> usize {
    if k > p.len() || p[k] == k {
        return k;
    }
    p[k] = find(p[k], p);
    p[k]
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let t: usize = input.trim().parse().unwrap();

    for _ in 0..t {
        input.clear();
        std::io::stdin().read_line(&mut input).unwrap();
        let parts: Vec<usize> = input.trim().split(" ").map(|x| x.parse().unwrap()).collect();
        let n = parts[0];
        let x = parts[1];
        let y = parts[2];

        input.clear();
        std::io::stdin().read_line(&mut input).unwrap();
        let a: Vec<char> = input.trim().chars().collect();

        let mut p: Vec<usize> = (0..=n+1).collect();
        let mut t: Vec<usize> = (0..=n+1).collect();
        let mut ans: Vec<usize> = vec![0; n+3];
        let mut z: Vec<usize> = vec![0; n+3];

​        z[1] = n;
        for i in 2..=n {
            let mut j = 0;
            let mut k = 0;
            while j + z[j] > i {
                z[i] = cmp::min(z[i-j+1], j+z[j]-i);
            }
            while a[i+z[i]] == a[1+z[i]] {
                z[i] += 1;
            }
​
            if i+z[i] > j+z[j] {
                j = i;
            }
        }

        let mut t_sorted = (1..=n).collect();
        t_sorted.sort_by_key(|&k| z[k]);
​
        let mut k = 1;
        for i in 1..=n {
            let mut c = 0;
            while k <= n && z[t_sorted[k]] < i {
                p[t_sorted[k]] = t_sorted[k] + 1;
                k += 1;
            }
            for mut j in 1..=n {
                j = find(j, &mut p);
                c += 1;
            }
            ans[c] = i;
        }

        for i in (1..=n).rev() {
            ans[i] = cmp::max(ans[i], ans[i+1]);
        }

        for i in x..=y {
            print!("{} ", ans[i]);
        }
        println!();
    }
}