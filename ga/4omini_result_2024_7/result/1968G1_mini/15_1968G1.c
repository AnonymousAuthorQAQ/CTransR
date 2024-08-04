use rand::Rng;
use std::io::{self, BufRead};

const N: usize = 200005;
const MOD: usize = 1000000007;

fn power(a: usize, b: usize) -> usize {
    if b == 0 {
        return 1;
    }
    let half = power(a, b / 2);
    let half = half * half % MOD;
    if b % 2 == 1 {
        half * a % MOD
    } else {
        half
    }
}

fn check(hash: &[usize], base_pow: &[usize], n: usize, k: usize, l: usize) -> usize {
    let mut count = 0;
    let mut i = 1;

    while i + k - 1 <= n {
        let current_hash = (hash[i + k - 1] + MOD - hash[i - 1]) % MOD;
        if current_hash == (hash[k] * base_pow[i - 1] % MOD) {
            count += 1;
            i += k;
        } else {
            i += 1;
        }
    }
    count
}

fn main() {
    let stdin = io::stdin();
    let input = stdin.lock().lines();
    let t: usize = input.next().unwrap().unwrap().trim().parse().unwrap();

    for _ in 0..t {
        let first_line = input.next().unwrap().unwrap();
        let nums: Vec<usize> = first_line.trim().split_whitespace().map(|s| s.parse().unwrap()).collect();
        let (n, l, r) = (nums[0], nums[1], nums[2]);
        
        let s: Vec<char> = input.next().unwrap().unwrap().chars().collect();
        let s = ['\0'] // 1-indexed adjustment
            .iter()
            .chain(s.iter())
            .collect::<Vec<_>>();

        let mut rng = rand::thread_rng();
        let mut base = rng.gen_range(200..MOD);
        
        let mut base_pow = vec![0; N + 1];
        let mut hash = vec![0; N + 1];

        base_pow[0] = 1;
        for i in 1..=n {
            base_pow[i] = base_pow[i - 1] * base % MOD;
            hash[i] = (hash[i - 1] + base_pow[i] * (s[i] as usize)) % MOD;
        }

        let mut lower = 0;
        let mut upper = n + 1;
        let mut yy;

        while upper - lower > 1 {
            let mid = lower + (upper - lower) / 2;
            yy = check(&hash, &base_pow, n, mid, l);
            if yy >= l {
                lower = mid;
            } else {
                upper = mid;
            }
        }
        print!("{}", lower);

        for i in (l + 1)..=r {
            if i > yy {
                upper = lower + 1;
                lower = 0;

                while upper - lower > 1 {
                    let mid = lower + (upper - lower) / 2;
                    yy = check(&hash, &base_pow, n, mid, i);
                    if yy >= i {
                        lower = mid;
                    } else {
                        upper = mid;
                    }
                }
            }
            print!("{}", lower);
        }

        println!(); // for newline after each test case
    }
}