use std::time::{SystemTime, UNIX_EPOCH};

const N: usize = 200005;

fn rand1() -> u32 {
    let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as u32;
    timestamp.wrapping_mul(timestamp)
}

fn rand2(l: u32, r: u32) -> u32 {
    if l == r {
        return l;
    }
    rand1() % (r - l) + l
}

fn power(a: i32, b: i32) -> i32 {
    if b == 0 {
        return 1;
    }
    let r = power(a, b / 2);
    if b % 2 != 0 {
        return r.wrapping_mul(r).wrapping_mul(a) % MOD;
    }
    return r.wrapping_mul(r) % MOD;
}

const MOD: i32 = 1000000007;
const BASE: i32 = 200;

struct Test {
    n: usize,
    l: i32,
    r: i32,
    s: String,
}

impl Test {
    fn new(n: usize, l: i32, r: i32, s: String) -> Test {
        Test { n, l, r, s }
    }
}

fn check(k: usize, n: usize, hash: &mut [i32], base_pow: &mut [i32]) -> i32 {
    let mut vo = 0;
    let mut i = 1;
    while i + k - 1 <= n {
        if (hash[i + k - 1].wrapping_add(MOD).wrapping_sub(hash[i - 1]) % MOD)
            == hash[k] * base_pow[i - 1] % MOD
        {
            vo += 1;
            i += k;
        } else {
            i += 1;
        }
    }
    vo
}

fn main() {
    let mut tests: Vec<Test> = Vec::new();

    let t: i32 = 1; // Number of test cases, can be changed

    // Taking input for each test case and creating Test objects
    for _ in 0..t {
        let n: usize = 10; // Sample size of the string, can be changed
        let l: i32 = 1; // Sample lower range, can be changed
        let r: i32 = 2; // Sample upper range, can be changed
        let s: String = String::from("abcdefg"); // Sample string, can be changed

        let new_test = Test::new(n, l, r, s);
        tests.push(new_test);
    }

    let mut base_pow: [i32; N + 1] = [1; N + 1];
    for i in 1..=N {
        base_pow[i] = base_pow[i - 1].wrapping_mul(BASE).rem_euclid(MOD);
    }

    for test in tests {
        let mut hash: [i32; N] = [0; N];
        for i in 1..=test.n {
            hash[i] = base_pow[i - 1].wrapping_mul(test.s.chars().nth(i - 1).unwrap() as i32).rem_euclid(MOD);
            hash[i] = (hash[i] + hash[i - 1]).rem_euclid(MOD);
        }

        let mut lower = 0;
        let mut upper = test.n + 1;
        let mut yy;

        while upper - lower > 1 {
            let mid = lower + (upper - lower) / 2;
            if (yy = check(mid, test.n, &mut hash, &mut base_pow)) >= test.l {
                lower = mid;
            } else {
                upper = mid;
            }
        }

        print!("{}", lower);

        for i in test.l + 1..=test.r {
            if i > yy {
                upper = lower + 1;
                lower = 0;
                while upper - lower > 1 {
                    let mid = lower + (upper - lower) / 2;
                    if (yy = check(mid, test.n, &mut hash, &mut base_pow)) >= i {
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