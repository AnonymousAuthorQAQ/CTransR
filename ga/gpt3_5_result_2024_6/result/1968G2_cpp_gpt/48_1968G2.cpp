use std::cmp::min;
use std::collections::HashMap;

const N: usize = 2e5 + 3;
const ALPHABET: usize = 26;

fn find(k: usize, p: &mut HashMap<usize, usize>) -> usize {
    if k > N || !p.contains_key(&k) {
        return k;
    } else {
        let parent = *p.get(&k).unwrap();
        p.insert(k, find(parent, p));
        return *p.get(&k).unwrap();
    }
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let T: u32 = input.trim().parse().unwrap();

    for _ in 0..T {
        input.clear();
        std::io::stdin().read_line(&mut input).unwrap();
        let parts: Vec<&str> = input.split_whitespace().collect();
        let n: usize = parts[0].parse().unwrap();
        let x: usize = parts[1].parse().unwrap();
        let y: usize = parts[2].parse().unwrap();
        let a: &str = parts[3];

        let mut z: Vec<usize> = vec![0; n + 3];
        let mut p: HashMap<usize, usize> = HashMap::new();
        let mut t: Vec<usize> = (1..=n + 1).collect();
        let mut ans: Vec<usize> = vec![0; n + 3];

        let mut current_match: usize = 0;
        z[1] = n;

        for i in 2..=n {
            let mut j = current_match;
            if j + z[j] > i {
                z[i] = min(min(z[i - j + 1], j + z[j] - i), z[i]);
            }

            while a.chars().nth(i - 1).unwrap() == a.chars().nth(1 + z[i] - 1).unwrap() {
                z[i] += 1;
            }

            if i + z[i] > j + z[j] {
                current_match = i;
            }
        }

        t.sort_by(|&i, &j| z[i].cmp(&z[j]));

        let mut k = 1;
        let mut j = 0;

        while k <= n {
            if z[t[k]] < j {
                p.insert(t[k], t[k] + 1);
                k += 1;
            } else {
                let parent = find(t[k], &mut p);
                p.insert(t[k], parent);
            }

            let mut c = 0;

            for j in 1..=n {
                let mut current_node = j;

                loop {
                    current_node = find(current_node + i, &mut p);
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

        println!("");
    }
}