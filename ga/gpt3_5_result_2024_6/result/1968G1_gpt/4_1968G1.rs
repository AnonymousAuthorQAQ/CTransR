use std::time::{SystemTime, UNIX_EPOCH};

const N: usize = 200005;
const MOD: u64 = 1000000007;

fn rand1() -> u32 {
    let rand_num: u32 = rand::random();
    rand_num.wrapping_mul(rand_num)
}

fn rand2(l: u32, r: u32) -> u32 {
    if l == r {
        return l;
    }
    rand1() % (r - l) + l
}

fn power(a: u64, b: u64) -> u64 {
    if b == 0 {
        1
    } else {
        let r = power(a, b / 2);
        if b % 2 != 0 {
            r.wrapping_mul(r).rem_euclid(MOD).wrapping_mul(a).rem_euclid(MOD)
        } else {
            r.wrapping_mul(r).rem_euclid(MOD)
        }
    }
}

fn check(hash: &Vec<u64>, base_pow: &Vec<u64>, n: usize, k: usize) -> u32 {
    let mut vo = 0;
    for i in 1..=n {
        if (hash[i + k - 1] + MOD - hash[i - 1]) % MOD == (hash[k] * base_pow[i - 1]) % MOD {
            vo += 1;
        }
    }
    vo
}

fn main() {
    let mut rng = rand::thread_rng();
    let mut s = String::new();
    
    let mut base = rand2(200, MOD - 1);
    let mut base_pow: Vec<u64> = vec![1; N + 1];
    for i in 1..=N {
        base_pow[i] = base_pow[i - 1].wrapping_mul(base).rem_euclid(MOD);
    }

    let t: usize = read_line().trim().parse().unwrap();
    for _ in 0..t {
        let mut input_line = String::new();
        std::io::stdin().read_line(&mut input_line).unwrap();
        let mut iter = input_line.split_whitespace();
        
        let n: usize = iter.next().unwrap().parse().unwrap();
        let l: u32 = iter.next().unwrap().parse().unwrap();
        let r: u32 = iter.next().unwrap().parse().unwrap();
        s = iter.next().unwrap().to_string();

        let mut hash: Vec<u64> = vec![0; N + 1];
        for i in 1..=n {
            hash[i] = (base_pow[i - 1] * s.chars().nth(i-1).unwrap() as u64).rem_euclid(MOD);
            hash[i] = (hash[i] + hash[i - 1]).rem_euclid(MOD);
        }

        let mut lower: u32 = 0;
        let mut upper: usize = n + 1;
        let mut yy: u32 = 0;

        while upper - lower as usize > 1 {
            let mid = lower + (upper - lower as usize) / 2;
            if (yy = check(&hash, &base_pow, n, mid)) >= l {
                lower = mid as u32;
            } else {
                upper = mid as usize;
            }
        }
        print!("{}", lower);

        for i in l + 1..=r {
            if i > yy {
                upper = (lower + 1) as usize;
                lower = 0;
                while upper - lower as usize > 1 {
                    let mid = lower + (upper - lower as usize) / 2;
                    if (yy = check(&hash, &base_pow, n, mid as usize)) >= i {
                        lower = mid as u32;
                    } else {
                        upper = mid as usize;
                    }
                }
            }
            print!("{}", lower);
        }

        println!("");
    }
}

fn read_line() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read line");
    input
}