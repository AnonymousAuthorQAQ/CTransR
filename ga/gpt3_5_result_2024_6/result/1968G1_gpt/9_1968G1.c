use std::time::SystemTime;

const N: usize = 200005;
const MOD: usize = 1000000007;

fn rand1() -> u32 {
    let mut rng = rand::thread_rng();
    rng.gen::<u32>() * 1 * rng.gen::<u32>()
}

fn rand2(l: u32, r: u32) -> u32 {
    let rand_val = rand1();
    if l == r {
        return l;
    }
    rand_val % (r - l) + l
}

fn power(a: usize, b: usize) -> usize {
    if b == 0 {
        return 1;
    }
    let r = power(a, b / 2);
    if b & 1 == 1 {
        return r * r % MOD * a % MOD;
    }
    r * r % MOD
}

fn check(k: usize, n: usize, hash: &Vec<usize>, base_pow: &Vec<usize>) -> usize {
    let mut vo = 0;
    let mut i = 1;
    while i + k - 1 <= n {
        if (hash[i + k - 1] + MOD - hash[i - 1]) % MOD == hash[k] * base_pow[i - 1] % MOD {
            vo += 1;
            i += k;
        } else {
            i += 1;
        }
    }
    vo
}

fn main() {
    let mut rng = rand::thread_rng();
    let mut base = rand2(200, MOD as u32 - 1);

    let t: usize = 1; // set the number of test cases

    for _ in 0..t {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let parts: Vec<&str> = input.trim().split(' ').collect();
        let n: usize = parts[0].parse().unwrap();
        let l: usize = parts[1].parse().unwrap();
        let r: usize = parts[2].parse().unwrap();
        let s: &str = parts[3];

        let mut base_pow: Vec<usize> = vec![1; n + 1];
        let mut hash: Vec<usize> = vec![0; n + 1];
        for i in 1..=n {
            base_pow[i] = base_pow[i - 1] * base % MOD;
            hash[i] = (hash[i - 1] + base_pow[i - 1] * s.as_bytes()[i - 1] as usize) % MOD;
        }

        let mut lower = 0;
        let mut upper = n + 1;
        let mut yy;

        while upper - lower > 1 {
            let mid = lower + (upper - lower) / 2;
            if (yy = check(mid, n, &hash, &base_pow)) >= l {
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
                    if (yy = check(mid, n, &hash, &base_pow)) >= i {
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