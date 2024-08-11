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
        let mut items = input.trim().split_whitespace();
        let n: usize = items.next().unwrap().parse().unwrap();
        let x: usize = items.next().unwrap().parse().unwrap();
        let y: usize = items.next().unwrap().parse().unwrap();

        input.clear();
        std::io::stdin().read_line(&mut input).unwrap();
        let a: Vec<char> = input.trim().chars().collect();

        let mut p = vec![0; N];
        let mut t = vec![0; N];
        let mut ans = vec![0; N];
        let mut z = vec![0; N];
        let mut z_section = vec![0; N];

        z[1] = n;
        z_section[0] = n;

        let mut j = 0;
        for i in 2..=n {
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

        let mut sorted_t: Vec<usize> = (1..=n).collect();
        sorted_t.sort_by(|&i, &j| z[i].cmp(&z[j]));

        let mut k = 1;

        for i in 1..=n {
            let mut c = 0;
            while k <= n && z[sorted_t[k - 1]] < i {
                p[sorted_t[k - 1]] = sorted_t[k - 1] + 1;
                k += 1;
            }
            for j in 1..=n {
                let mut index = j;
                while index + i <= n {
                    index = find(index + i, &mut p);
                    c += 1;
                }
            }
            ans[c] = i;
        }

        for i in (1..=n).rev() {
            ans[i] = std::cmp::max(ans[i], ans[i + 1]);
        }

        for i in x..=y {
            print!("{} ", ans[i]);
        }
        println!();
    }
}