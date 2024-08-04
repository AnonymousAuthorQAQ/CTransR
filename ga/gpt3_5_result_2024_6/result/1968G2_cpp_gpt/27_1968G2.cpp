use std::cmp;

const N: usize = 200003;
const MAX_LENGTH: usize = 200000;

fn find(k: usize, p: &mut [usize]) -> usize {
    if k > MAX_LENGTH || p[k] == k {
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
        let parts: Vec<usize> = input.split_whitespace().map(|x| x.parse().unwrap()).collect();

        let n = parts[0];
        let x = parts[1];
        let y = parts[2];

        input.clear();
        std::io::stdin().read_line(&mut input).unwrap();
        let a = input.trim().chars().collect::<Vec<char>>();

        let mut p = vec![0; MAX_LENGTH + 3];
        let mut t = vec![0; MAX_LENGTH + 3];
        let mut ans = vec![0; MAX_LENGTH + 3];
        let mut z = vec![0; MAX_LENGTH + 3];
        
        z[1] = n;
        for i in 2..=n {
            let mut j = 0;
            if j + z[j] > i {
                z[i] = cmp::min(z[i - j + 1], j + z[j] - i);
            }
            for _ in 0.. {
                if a[i + z[i]] == a[1 + z[i]] {
                    z[i] += 1;
                } else {
                    break;
                }
            }
            if i + z[i] > j + z[j] {
                j = i;
            }
        }

        let mut t_sorted: Vec<usize> = (1..=n).collect();
        t_sorted.sort_by_key(|&i| z[i]);

        let mut k = 1;

        for i in 1..=n {
            let mut c = 0;
            let mut j = 1;
            while j <= n {
                j = find(j + i, &mut p);
                c += 1;
            }
            ans[c] = i;
        }

        for i in (1..=n).rev() {
            ans[i] = cmp::max(ans[i], ans[i + 1]);
        }

        for i in x..=y {
            print!("{} ", ans[i]);
        }
        println!();
    }
}