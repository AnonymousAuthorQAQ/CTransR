use std::io;
use rand::{Rng, thread_rng};
use std::cmp::{min, max};

const N: usize = 200005;
const MOD: usize = 1000000007;

fn power(a: usize, b: usize) -> usize {
    if b == 0 {
        return 1;
    }
    let mut r = power(a, b / 2);
    r = r * r % MOD;
    if b % 2 == 1 {
        r = r * a % MOD;
    }
    r
}

struct Hashing {
    hash: Vec<usize>,
    base_pow: Vec<usize>,
}

impl Hashing {
    fn new(s: &str, base: usize) -> Self {
        let n = s.len();
        let mut hash = vec![0; n + 1];
        let mut base_pow = vec![1; n + 1];

        for i in 0..n {
            let bp = base_pow[i] * base % MOD;
            base_pow[i + 1] = bp;
            hash[i + 1] = (bp * s[i..i + 1].as_bytes()[0] as usize) % MOD;
            hash[i + 1] = (hash[i + 1] + hash[i]) % MOD;
        }
        Hashing { hash, base_pow }
    }

    fn check(&self, k: usize) -> usize {
        let mut vo = 0;
        let n = self.hash.len() - 1;
        for i in 1..=n - k {
            if (self.hash[i + k - 1] + MOD - self.hash[i - 1]) % MOD == (self.hash[k] * self.base_pow[i - 1] % MOD) {
                vo += 1;
            }
        }
        vo
    }
}

fn main() {
    let mut stdin = io::stdin();
    let mut input = String::new();

    let mut rng = thread_rng();
    let base: usize = rng.gen_range(200..MOD as usize);

    stdin.read_line(&mut input).unwrap();
    let mut lines = input.trim().split_whitespace();
    let t: usize = lines.next().unwrap().parse().unwrap();

    for _ in 0..t {
        let n: usize;
        let l: usize;
        let r: usize;
        
        input.clear();
        stdin.read_line(&mut input).unwrap();
        let mut lines = input.trim().split_whitespace();
        n = lines.next().unwrap().parse().unwrap();
        l = lines.next().unwrap().parse().unwrap();
        r = lines.next().unwrap().parse().unwrap();

        input.clear();
        stdin.read_line(&mut input).unwrap();
        let s: Vec<char> = input.trim().chars().collect();

        let hashing = Hashing::new(&s.iter().collect::<String>(), base);
        let mut lower = 0;
        let mut upper = n + 1;

        while upper - lower > 1 {
            let mid = lower + (upper - lower) / 2;
            let yy = hashing.check(mid);
            if yy >= l {
                lower = mid;
            } else {
                upper = mid;
            }
        }
        print!("{}", lower);

        let mut yy = hashing.check(lower);

        for i in (l + 1)..=r {
            if i > yy {
                upper = lower + 1;
                lower = 0;
                while upper - lower > 1 {
                    let mid = lower + (upper - lower) / 2;
                    let yy_new = hashing.check(mid);
                    if yy_new >= i {
                        lower = mid;
                    } else {
                        upper = mid;
                    }
                }
            }
            print!("{}", lower);
            yy = hashing.check(lower);
        }

        println!();
    }
}