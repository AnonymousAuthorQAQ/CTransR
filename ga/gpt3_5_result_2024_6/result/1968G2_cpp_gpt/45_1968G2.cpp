use std::collections::HashMap;
use std::cmp::min;

const N: usize = 2e5 as usize + 3;

fn find(k: usize, p: &mut [usize; N]) -> usize {
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
        let a: String = iter.next().unwrap().parse().unwrap();

        let mut p = [0; N];
        let mut t = [0; N];
        let mut ans = [0; N];
        let mut z = [0; N];
        let mut freq_map: HashMap<usize, usize> = HashMap::new();
        
        z[1] = n;
        for i in 2..=n {
            let mut j = 0;
            z[i] = 0;
            if j + z[j] > i {
                z[i] = min(z[i - j + 1], j + z[j] - i);
            }
            while a.chars().nth(i + z[i] - 1).unwrap() == a.chars().nth(1 + z[i]).unwrap() {
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
            while k <= n && z[t_sorted[k]] < i {
                p[t_sorted[k]] = t_sorted[k] + 1;
                k += 1;
            }
            for j in 1..=n {
                let root_j = find(j, &mut p);
                freq_map.entry(root_j).and_modify(|v| *v += 1).or_insert(1);
                if freq_map[root_j] == i {
                    c += 1;
                }
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