use std::time::SystemTime;

const N: usize = 200005;

const MOD: u64 = 1000000007;

fn rand1() -> u64 {
    let t = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs();
    return t.wrapping_mul(t).wrapping_mul(t);
}

fn rand2(l: u64, r: u64) -> u64 {
    if l == r {
        return l;
    }
    return rand1() % (r - l) + l;
}

fn power(a: u64, b: u64) -> u64 {
    if b == 0 {
        return 1;
    }
    let mut r = power(a, b / 2);
    if b % 2 == 1 {
        return r.wrapping_mul(r).wrapping_mul(a) % MOD;
    }
    return r.wrapping_mul(r) % MOD;
}

fn check(hash: &Vec<u64>, base_pow: &Vec<u64>, n: usize, k: usize) -> i32 {
    let mut vo = 0;
    for i in 1..=n {
        if i + k - 1 <= n {
            if (hash[i + k - 1].wrapping_add(MOD).wrapping_sub(hash[i - 1])) % MOD as u64 == hash[k] * base_pow[i - 1] % MOD as u64 {
                vo += i as i32;
            } else {
                continue;
            }
        }
    }
    return vo;
}

fn main() {
    let t: usize;
    let mut input = String::new();
    let mut count = 0;
    let mut n = 0;
    let mut l = 0;
    let mut r = 0;
    let s: Vec<char>;

    std::io::stdin().read_line(&mut input).unwrap();
    t = input.trim().parse().unwrap();

    for _ in 0..t {
        input.clear();
        std::io::stdin().read_line(&mut input).unwrap();
        let parts: Vec<&str> = input.trim().split_whitespace().collect();

        n = parts[0].parse().unwrap();
        l = parts[1].parse().unwrap();
        r = parts[2].parse().unwrap();
        s = parts[3].chars().collect();

        let mut bp = 1;
        let mut base_pow: Vec<u64> = vec![0; n + 1];
        let mut hash: Vec<u64> = vec![0; n + 2];

        for i in 1..=n {
            bp = bp.wrapping_mul(200).wrapping_rem(MOD as u64);
            base_pow[i] = bp.wrapping_mul(200).wrapping_rem(MOD as u64);
            hash[i] = bp.wrapping_mul(s[i] as u64).wrapping_rem(MOD as u64);
            hash[i] = (hash[i].wrapping_add(hash[i - 1])).wrapping_rem(MOD as u64);
        }

        let mut lower = 0;
        let mut upper = n + 1;
        let mut yy = 0;

        while upper - lower > 1 {
            let mid = lower + (upper - lower) / 2;
            if (yy = check(&hash, &base_pow, n, mid)) >= l {
                lower = mid;
            } else {
                upper = mid;
            }
        }

        print!("{}", lower);

        for i in ((l + 1)..=r) {
            if i > yy {
                upper = lower + 1;
                lower = 0;
                while upper - lower > 1 {
                    let mid = lower + (upper - lower) / 2;
                    if (yy = check(&hash, &base_pow, n, mid)) >= i {
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