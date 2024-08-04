use std::io::{self, BufRead};

const N: usize = 200005;
const MOD: i64 = 1000000007;

fn rand1() -> u32 {
    (rand() as u32).wrapping_mul(rand() as u32)
}

fn rand2(l: usize, r: usize) -> usize {
    if l == r {
        return l;
    }
    rand1() as usize % (r - l) + l
}

struct Test {
    n: usize,
    l: usize,
    r: usize,
    s: Vec<char>,
    hash: Vec<i64>,
    BASE: i64,
    base_pow: Vec<i64>,
}

impl Test {
    fn new(n: usize, l: usize, r: usize, s: Vec<char>) -> Self {
        Test {
            n,
            l,
            r,
            s,
            hash: vec![0; n + 1],
            BASE: rand2(200, MOD as usize - 1) as i64,
            base_pow: vec![0; n + 1],
        }
    }

    fn power(a: i64, b: usize) -> i64 {
        if b == 0 {
            return 1;
        }
        let r = Self::power(a, b / 2);
        if b % 2 == 1 {
            return r.wrapping_mul(r).wrapping_rem(MOD as i64).wrapping_mul(a).wrapping_rem(MOD as i64);
        }
        r.wrapping_mul(r).wrapping_rem(MOD as i64)
    }

    fn check(&self, k: usize) -> usize {
        let mut vo = 0;
        let mut i = 1;
        while i + k - 1 <= self.n {
            if (self.hash[i + k - 1] + MOD - self.hash[i - 1]) % MOD == self.hash[k] * self.base_pow[i - 1] % MOD {
                vo += 1;
                i += k;
            } else {
                i += 1;
            }
        }
        vo
    }
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines().map(|line| line.unwrap());

    let t: usize = lines.next().unwrap().parse().unwrap();
    for _ in 0..t {
        let input: Vec<usize> = lines.next().unwrap().split_whitespace().map(|s| s.parse().unwrap()).collect();
        let n = input[0];
        let l = input[1];
        let r = input[2];
        let s: Vec<char> = input[3].chars().collect();

        let mut test = Test::new(n, l, r, s);

        let mut bp = 1;
        test.base_pow[0] = 1;
        for i in 1..=n {
            bp = bp.wrapping_mul(test.BASE).wrapping_rem(MOD as i64);
            test.base_pow[i] = bp.wrapping_mul(test.BASE).wrapping_rem(MOD as i64);
            test.hash[i] = bp.wrapping_mul(s[i - 1] as i64).wrapping_rem(MOD as i64);
            test.hash[i] = (test.hash[i] + test.hash[i - 1]).wrapping_rem(MOD as i64);
        }

        let mut lower = 0;
        let mut upper = n + 1;
        let mut yy;

        while upper - lower > 1 {
            let mid = lower + (upper - lower) / 2;
            if (yy = test.check(mid)) >= test.l {
                lower = mid;
            } else {
                upper = mid;
            }
        }
        print!("{} ", lower);

        for i in test.l + 1..=test.r {
            if i > yy {
                upper = lower + 1;
                lower = 0;
                while upper - lower > 1 {
                    let mid = lower + (upper - lower) / 2;
                    if (yy = test.check(mid)) >= i {
                        lower = mid;
                    } else {
                        upper = mid;
                    }
                }
            }
            print!("{} ", lower);
        }

        println!();
    }
}