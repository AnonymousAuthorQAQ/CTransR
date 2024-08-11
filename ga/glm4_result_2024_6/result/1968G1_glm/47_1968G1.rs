use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let t = lines.next().unwrap().unwrap().parse::<i32>().unwrap();

    for _ in 0..t {
        let mut line = lines.next().unwrap().unwrap();
        let mut iter = line.split_whitespace();
        let n = iter.next().unwrap().parse::<i32>().unwrap();
        let l = iter.next().unwrap().parse::<i32>().unwrap();
        let r = iter.next().unwrap().parse::<i32>().unwrap();
        let s = iter.next().unwrap();

        let base = 26;
        let mut base_pow = vec![1; n as usize + 1];
        for i in 1..=n {
            base_pow[i] = base_pow[i - 1] * base;
        }

        let mut hash = vec![0; n as usize + 1];
        for i in 1..=n {
            hash[i] = hash[i - 1] * base + s.chars().nth(i as usize - 1).unwrap() as i32;
        }

        let mut lower = 0;
        let mut upper = n + 1;
        while upper - lower > 1 {
            let mid = lower + (upper - lower) / 2;
            if check(mid, &hash, &base_pow, n) >= l {
                lower = mid;
            } else {
                upper = mid;
            }
        }
        println!("{}", lower);

        for i in l + 1..=r {
            if i > check(lower, &hash, &base_pow, n) {
                upper = lower + 1;
                lower = 0;
                while upper - lower > 1 {
                    let mid = lower + (upper - lower) / 2;
                    if check(mid, &hash, &base_pow, n) >= i {
                        lower = mid;
                    } else {
                        upper = mid;
                    }
                }
            }
            println!("{}", lower);
        }
        println!();
    }
}

fn check(k: i32, hash: &Vec<i32>, base_pow: &Vec<i32>, n: i32) -> i32 {
    let mut vo = 0;
    let mut i = 1;
    while i + k - 1 <= n {
        if (hash[i + k - 1] + std::i32::MAX - hash[i - 1]) % std::i32::MAX == hash[k as usize] * base_pow[i - 1] % std::i32::MAX {
            vo += 1;
            i += k;
        } else {
            i += 1;
        }
    }
    vo
}