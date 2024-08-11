use std::io::{self, BufRead};

fn main() {
    let mut input = String::new();
    io::stdin().lock().read_line(&mut input).unwrap();
    let t: usize = input.trim().parse().unwrap();

    for _ in 0..t {
        input.clear();
        io::stdin().lock().read_line(&mut input).unwrap();
        let parts: Vec<usize> = input.trim().split_whitespace().map(|x| x.parse().unwrap()).collect();
        let n = parts[0];
        let l = parts[1];
        let r = parts[2];

        input.clear();
        io::stdin().lock().read_line(&mut input).unwrap();
        let s: Vec<char> = input.trim().chars().collect();

        let mut hash = vec![0; n + 1];
        let mut base_pow = vec![0; n + 1];
        let mut base = rand2(200, 1000000007);

        let mut bp = 1;
        for i in 1..=n {
            bp = bp.wrapping_mul(base) % 1000000007;
            base_pow[i] = bp.wrapping_mul(base) % 1000000007;
            hash[i] = (bp.wrapping_mul(s[i] as u64) % 1000000007 + hash[i - 1]) % 1000000007;
        }

        let mut lower = 0;
        let mut upper = n + 1;
        let mut yy;

        while upper - lower > 1 {
            let mid = lower + (upper - lower) / 2;
            if (yy = check(&hash, mid)) >= l {
                lower = mid;
            } else {
                upper = mid;
            }
        }

        print!("{}", lower);

        for i in l + 1..=r {
            if i > yy {
                upper = lower + 1;
                lower = 0;
                while upper - lower > 1 {
                    let mid = lower + (upper - lower) / 2;
                    if (yy = check(&hash, mid)) >= i {
                        lower = mid;
                    } else {
                        upper = mid;
                    }
                }
            }
            print!("{}", lower);
        }

        println!();
    }
}

fn rand1() -> u64 {
    let val = rand().wrapping_mul(1) * rand();
    val as u64
}

fn rand2(l: u64, r: u64) -> u64 {
    if l == r {
        return l;
    }
    return rand1() % (r - l) + l;
}

fn power(a: i64, b: i64) -> i64 {
    if b == 0 {
        return 1;
    }
    let r = power(a, b / 2);
    if b % 2 == 1 {
        return r.wrapping_mul(r).wrapping_mul(a) % 1000000007;
    }
    return r.wrapping_mul(r) % 1000000007;
}

fn check(hash: &Vec<u64>, k: usize) -> i64 {
    let mut vo = 0;
    let mut i = 1;
    while i + k - 1 <= hash.len() {
        if (hash[i + k - 1].wrapping_add(1000000007).wrapping_sub(hash[i - 1])) % 1000000007 == hash[k] as u64).wrapping_mul(base_pow[i - 1]) % 1000000007 {
            vo += 1;
            i += k;
        } else {
            i += 1;
        }
    }
    vo
}