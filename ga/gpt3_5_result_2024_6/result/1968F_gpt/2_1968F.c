use std::time::{SystemTime, UNIX_EPOCH};

const N: usize = 200000;

static mut Z: u32 = 0;

fn rand_() -> i32 {
    unsafe {
        Z = Z.wrapping_mul(3) >> 1;
        Z as i32
    }
}

fn srand_() {
    unsafe {
        if let Ok(duration) = SystemTime::now().duration_since(UNIX_EPOCH) {
            Z = (duration.as_secs() as u32) ^ (duration.as_micros() as u32) | 1;
        }
    }
}

fn sort(ii: &mut [i32], l: usize, r: usize, aa: &[i32]) {
    let mut l = l;
    let mut r = r;

    while l < r {
        let i = l;
        let mut j = l;
        let mut k = r;
        let mut i_ = aa[ii[l + rand_() as usize % (r - l)] as usize];

        while j < k {
            let c = if aa[ii[j] as usize] != i_ {
                aa[ii[j] as usize] - i_
            } else {
                ii[j] - i_
            };

            if c == 0 {
                j += 1;
            } else if c < 0 {
                ii.swap(i, j);
                i += 1;
                j += 1;
            } else {
                k -= 1;
                ii.swap(j, k);
            }
        }
        sort(ii, l, i, aa);
        l = k;
    }
}

fn search(a: i32, i: i32, aa: &[i32], ii: &[i32]) -> i32 {
    let mut lower = -1;
    let mut upper = (ii.len() + 1) as i32;

    while upper - lower > 1 {
        let h = (lower + upper) / 2;

        if aa[ii[h as usize] as usize] < a || (aa[ii[h as usize] as usize] == a && ii[h as usize] < i) {
            lower = h;
        } else {
            upper = h;
        }
    }
    lower
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read input");
    let t: i32 = input.trim().parse().unwrap();

    srand_();

    for _ in 0..t {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("Failed to read input");
        let mut iter = input.split_whitespace();
        let n: usize = iter.next().unwrap().parse().unwrap();
        let q: usize = iter.next().unwrap().parse().unwrap();

        let mut aa = vec![0; n + 1];
        for i in 1..=n {
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).expect("Failed to read input");
            aa[i] = input.trim().parse().unwrap();
            aa[i] ^= aa[i - 1];
        }

        let mut ii: Vec<i32> = (0..=n as i32).collect();

        sort(&mut ii, 0, n + 1, &aa);

        for _ in 0..q {
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).expect("Failed to read input");
            let mut iter = input.split_whitespace();
            let l: i32 = iter.next().unwrap().parse().unwrap();
            let r: i32 = iter.next().unwrap().parse().unwrap();

            let result = if aa[l as usize] == aa[r as usize] || ii[search(aa[r as usize], l, &aa, &ii) as usize + 1] < ii[search(aa[l as usize], r, &aa, &ii) as usize] {
                "YES".to_string()
            } else {
                "NO".to_string()
            };

            println!("{}", result);
        }
    }
}