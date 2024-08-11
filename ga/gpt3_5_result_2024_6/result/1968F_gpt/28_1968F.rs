use std::time::{SystemTime, UNIX_EPOCH};

const N: usize = 200000;

static mut Z: u32 = 0;

fn rand_() -> u32 {
    unsafe {
        Z = Z.wrapping_mul(3) >> 1;
        Z
    }
}

fn srand_() {
    unsafe {
        let start = SystemTime::now();
        let since_the_epoch = start.duration_since(UNIX_EPOCH).expect("Time went backwards");
        Z = (since_the_epoch.as_secs() as u32) ^ (since_the_epoch.as_micros() as u32) | 1;
    }
}

fn sort(ii: &mut [usize], l: usize, r: usize, aa: &[u32]) {
    if l < r {
        let mut i = l;
        let mut j = l;
        let mut k = r;
        let i_ = aa[l + (rand_() % (r - l))];

        while j < k {
            let c = if aa[ii[j]] != i_ {
                aa[ii[j]] - i_
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
        sort(ii, k, r, aa);
    }
}

fn search(a: u32, i: usize, ii: &[usize], aa: &[u32]) -> i32 {
    let mut lower = -1;
    let mut upper = (ii.len() + 1) as i32;

    while upper - lower > 1 {
        let h = (lower + upper) / 2;

        if aa[ii[h as usize]] < a || (aa[ii[h as usize]] == a && ii[h as usize] < i) {
            lower = h;
        } else {
            upper = h;
        }
    }
    lower
}

fn main() {
    srand_();
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let t: usize = input.trim().parse().unwrap();

    for _ in 0..t {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let mut iter = input.split_whitespace();
        let n: usize = iter.next().unwrap().parse().unwrap();
        let q: usize = iter.next().unwrap().parse().unwrap();

        let mut aa = vec![0; n + 1];
        for i in 1..=n {
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            aa[i] = input.trim().parse().unwrap();
            aa[i] ^= aa[i - 1];
        }

        let mut ii = (0..=n).collect::<Vec<usize>>();
        sort(&mut ii, 0, n + 1, &aa);

        for _ in 0..q {
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            let mut iter = input.split_whitespace();
            let l: usize = iter.next().unwrap().parse().unwrap();
            let r: usize = iter.next().unwrap().parse().unwrap();

            if aa[l] == aa[r] || ii[(search(aa[r], l - 1, &ii, &aa) + 1) as usize] < ii[search(aa[l], r, &ii, &aa) as usize] {
                println!("YES");
            } else {
                println!("NO");
            }
        }
    }
}