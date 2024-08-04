use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut iterator = stdin.lock().lines().map(|x| x.unwrap());

    let t: usize = iterator.next().unwrap().trim().parse().unwrap();

    for _ in 0..t {
        let input: Vec<&str> = iterator.next().unwrap().trim().split_whitespace().collect();
        let n: usize = input[0].parse().unwrap();
        let kl: usize = input[1].parse().unwrap();
        let kr: usize = input[2].parse().unwrap();
        let cc: Vec<char> = input[3].chars().collect();

        let mut zz: Vec<usize> = vec![0; n];
        let mut l = 0;
        let mut r = 0;

        for i in 1..n {
            if i + zz[i - l] < r {
                zz[i] = zz[i - l];
            } else {
                l = i;
                r = r.max(i);
                while r < n && cc[r] == cc[r - l] {
                    r += 1;
                }
                zz[i] = r - l;
            }
        }

        let mut ans: Vec<usize> = vec![0; n + 1];
        for p in 1..=n {
            let mut k = 0;
            let mut i = 0;
            while let Some(val) = query(i + p, p, &zz, n) {
                i = val;
                k += 1;
            }
            ans[k] = ans[k].max(p);
        }

        for k in (1..=n - 1).rev() {
            ans[k] = ans[k].max(ans[k + 1]);
        }

        for k in kl..=kr {
            print!("{} ", ans[k]);
        }

        println!();
    }
}

fn max(a: usize, b: usize) -> usize {
    if a > b { a } else { b }
}

fn query(mut l: usize, a: usize, zz: &Vec<usize>, n: usize) -> Option<usize> {
    let mut r = n - 1;

    while l <= r {
        if l & 1 == 1 {
            if zz[l] >= a {
                let mut val = l;
                while val < n {
                    val = if zz[val * 2] >= a { val * 2 } else { val * 2 + 1 };
                }
                return Some(val - n);
            }
            l += 1;
        }
        l >>= 1;
        r >>= 1;
    }

    None
}